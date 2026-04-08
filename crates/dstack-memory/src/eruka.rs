use crate::{Field, MemoryError, MemoryProvider, Result};
use async_trait::async_trait;
use serde::Deserialize;

/// Eruka REST API memory backend.
/// Calls GET/POST /api/v1/context with X-Service-Key authentication.
pub struct ErukaProvider {
    pub base_url: String,
    pub service_key: String,
    client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct ErukaContextResponse {
    #[serde(default)]
    fields: Vec<ErukaField>,
}

#[derive(Debug, Deserialize)]
struct ErukaField {
    field_path: String,
    value: String,
    #[serde(default)]
    confidence: Option<f64>,
    #[serde(default)]
    updated_at: Option<String>,
}

impl ErukaProvider {
    pub fn new(base_url: impl Into<String>, service_key: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            service_key: service_key.into(),
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap_or_default(),
        }
    }

    fn map_field(ef: &ErukaField) -> Field {
        Field {
            path: ef.field_path.clone(),
            value: ef.value.clone(),
            confidence: ef.confidence.unwrap_or(0.5),
            source: "eruka".into(),
            updated_at: ef
                .updated_at
                .as_ref()
                .and_then(|s| {
                    chrono::DateTime::parse_from_rfc3339(s)
                        .ok()
                        .or_else(|| {
                            chrono::DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f%:z").ok()
                        })
                })
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(chrono::Utc::now),
        }
    }
}

#[async_trait]
impl MemoryProvider for ErukaProvider {
    async fn load(&self, path: &str) -> Result<Vec<Field>> {
        let url = format!("{}/api/v1/context?path={}", self.base_url, path);
        let resp = self
            .client
            .get(&url)
            .header("X-Service-Key", &self.service_key)
            .send()
            .await
            .map_err(|e| MemoryError::Http(e.to_string()))?;
        if !resp.status().is_success() {
            return Err(MemoryError::Http(format!("HTTP {}", resp.status())));
        }
        let body: ErukaContextResponse = resp
            .json()
            .await
            .map_err(|e| MemoryError::Http(e.to_string()))?;
        Ok(body.fields.iter().map(Self::map_field).collect())
    }

    async fn write(&self, field: &Field) -> Result<()> {
        let url = format!("{}/api/v1/context", self.base_url);
        let resp = self
            .client
            .post(&url)
            .header("X-Service-Key", &self.service_key)
            .json(&serde_json::json!({
                "path": field.path,
                "value": field.value,
                "source": field.source,
                "confidence": field.confidence,
            }))
            .send()
            .await
            .map_err(|e| MemoryError::Http(e.to_string()))?;
        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(MemoryError::Http(format!("Write failed: {}", body)));
        }
        Ok(())
    }

    async fn search(&self, query: &str) -> Result<Vec<Field>> {
        // Try search endpoint first, fall back to load-and-filter
        let url = format!("{}/api/v1/context/search?q={}", self.base_url, query);
        let resp = self
            .client
            .get(&url)
            .header("X-Service-Key", &self.service_key)
            .send()
            .await;
        match resp {
            Ok(r) if r.status().is_success() => {
                let body: ErukaContextResponse = r
                    .json()
                    .await
                    .map_err(|e| MemoryError::Http(e.to_string()))?;
                Ok(body.fields.iter().map(Self::map_field).collect())
            }
            _ => {
                // Fallback: load all and filter client-side
                let all = self.load("").await.unwrap_or_default();
                let q = query.to_lowercase();
                Ok(all
                    .into_iter()
                    .filter(|f| {
                        f.value.to_lowercase().contains(&q)
                            || f.path.to_lowercase().contains(&q)
                    })
                    .collect())
            }
        }
    }

    async fn delete(&self, path: &str) -> Result<()> {
        let url = format!("{}/api/v1/context?path={}", self.base_url, path);
        let resp = self
            .client
            .delete(&url)
            .header("X-Service-Key", &self.service_key)
            .send()
            .await
            .map_err(|e| MemoryError::Http(e.to_string()))?;
        if !resp.status().is_success() {
            return Err(MemoryError::Http(format!(
                "Delete failed: HTTP {}",
                resp.status()
            )));
        }
        Ok(())
    }

    async fn export_all(&self) -> Result<Vec<Field>> {
        self.load("").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn provider_creation() {
        let provider = ErukaProvider::new("http://localhost:8081", "test-key");
        assert_eq!(provider.base_url, "http://localhost:8081");
        assert_eq!(provider.service_key, "test-key");
    }

    #[tokio::test]
    async fn unreachable_eruka_returns_error() {
        let provider = ErukaProvider::new("http://localhost:99999", "fake-key");
        let result = provider.load("test").await;
        assert!(result.is_err());
    }

    #[test]
    fn empty_service_key_allowed() {
        let provider = ErukaProvider::new("http://localhost:8081", "");
        assert!(provider.service_key.is_empty());
    }

    #[test]
    fn map_field_handles_missing_confidence() {
        let ef = ErukaField {
            field_path: "a/b".into(),
            value: "test".into(),
            confidence: None,
            updated_at: None,
        };
        let f = ErukaProvider::map_field(&ef);
        assert_eq!(f.confidence, 0.5);
        assert_eq!(f.source, "eruka");
    }

    #[test]
    fn map_field_parses_rfc3339_timestamp() {
        let ef = ErukaField {
            field_path: "x/y".into(),
            value: "v".into(),
            confidence: Some(0.9),
            updated_at: Some("2026-04-08T19:00:00+00:00".into()),
        };
        let f = ErukaProvider::map_field(&ef);
        assert_eq!(f.confidence, 0.9);
        assert_eq!(f.updated_at.year(), 2026);
    }
}
