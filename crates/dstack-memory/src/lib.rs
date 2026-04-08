use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod file;
#[cfg(feature = "eruka")]
pub mod eruka;

/// A single memory field — the atomic unit of persistent context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub path: String,
    pub value: String,
    pub confidence: f64,
    pub source: String,
    pub updated_at: DateTime<Utc>,
}

impl Field {
    pub fn new(path: impl Into<String>, value: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            value: value.into(),
            confidence: 0.5,
            source: source.into(),
            updated_at: Utc::now(),
        }
    }

    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("Not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, MemoryError>;

/// Pluggable memory backend. Implement this for custom storage.
#[async_trait]
pub trait MemoryProvider: Send + Sync {
    /// Load all fields matching a path prefix
    async fn load(&self, path: &str) -> Result<Vec<Field>>;
    /// Write a single field (upsert by path)
    async fn write(&self, field: &Field) -> Result<()>;
    /// Search fields by keyword in path or value
    async fn search(&self, query: &str) -> Result<Vec<Field>>;
    /// Delete a field by exact path
    async fn delete(&self, path: &str) -> Result<()>;
    /// Export all fields
    async fn export_all(&self) -> Result<Vec<Field>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_serialization_roundtrip() {
        let field = Field {
            path: "projects/ehb/learnings/tag-pipeline".into(),
            value: "All agent prompts need ---TAGS--- instruction".into(),
            confidence: 0.95,
            source: "user_correction".into(),
            updated_at: chrono::Utc::now(),
        };
        let json = serde_json::to_string(&field).unwrap();
        let parsed: Field = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.path, field.path);
        assert_eq!(parsed.confidence, 0.95);
    }

    #[test]
    fn field_default_confidence() {
        let field = Field::new("test/path", "value", "test");
        assert_eq!(field.confidence, 0.5);
        assert_eq!(field.source, "test");
    }

    #[test]
    fn field_confidence_clamped() {
        let field = Field::new("a", "b", "c").with_confidence(1.5);
        assert_eq!(field.confidence, 1.0);
        let field2 = Field::new("a", "b", "c").with_confidence(-0.5);
        assert_eq!(field2.confidence, 0.0);
    }

    #[test]
    fn field_new_sets_timestamp() {
        let before = Utc::now();
        let field = Field::new("x", "y", "z");
        let after = Utc::now();
        assert!(field.updated_at >= before && field.updated_at <= after);
    }
}
