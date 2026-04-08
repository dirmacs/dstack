use std::path::PathBuf;

use async_trait::async_trait;

use crate::{Field, MemoryProvider, Result};

/// JSON file-based memory backend.
///
/// Each field is stored as a separate `.json` file. The field path maps directly
/// to the filesystem: `"projects/ehb/learnings/tags"` becomes `{root}/projects/ehb/learnings/tags.json`.
pub struct FileProvider {
    root: PathBuf,
}

impl FileProvider {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// Convert a logical path to a filesystem path, sanitizing dangerous components.
    fn field_path(&self, path: &str) -> PathBuf {
        let sanitized: PathBuf = path
            .split('/')
            .filter(|seg| !seg.is_empty() && *seg != ".." && !seg.contains('\0'))
            .collect();
        let mut full = self.root.join(sanitized);
        full.set_extension("json");
        full
    }

    /// Recursively walk the root directory collecting all `.json` files.
    fn all_json_files(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();
        if self.root.is_dir() {
            self.walk_dir(&self.root, &mut files);
        }
        files
    }

    fn walk_dir(&self, dir: &std::path::Path, out: &mut Vec<PathBuf>) {
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                self.walk_dir(&path, out);
            } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
                out.push(path);
            }
        }
    }

    /// Read and deserialize a single JSON file into a Field.
    fn read_field(&self, path: &std::path::Path) -> Result<Field> {
        let data = std::fs::read_to_string(path)?;
        let field: Field = serde_json::from_str(&data)?;
        Ok(field)
    }
}

#[async_trait]
impl MemoryProvider for FileProvider {
    async fn load(&self, path: &str) -> Result<Vec<Field>> {
        let files = self.all_json_files();
        let prefix = if path.ends_with('/') {
            path.to_string()
        } else {
            format!("{}/", path)
        };

        let mut fields: Vec<Field> = files
            .into_iter()
            .filter_map(|f| self.read_field(&f).ok())
            .filter(|field| field.path.starts_with(&prefix) || field.path == path)
            .collect();

        fields.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(fields)
    }

    async fn write(&self, field: &Field) -> Result<()> {
        let file_path = self.field_path(&field.path);
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(field)?;
        std::fs::write(&file_path, json)?;
        Ok(())
    }

    async fn search(&self, query: &str) -> Result<Vec<Field>> {
        let query_lower = query.to_lowercase();
        let files = self.all_json_files();

        let fields: Vec<Field> = files
            .into_iter()
            .filter_map(|f| self.read_field(&f).ok())
            .filter(|field| {
                field.value.to_lowercase().contains(&query_lower)
                    || field.path.to_lowercase().contains(&query_lower)
            })
            .collect();

        Ok(fields)
    }

    async fn delete(&self, path: &str) -> Result<()> {
        let file_path = self.field_path(path);
        if file_path.exists() {
            std::fs::remove_file(&file_path)?;
        }
        Ok(())
    }

    async fn export_all(&self) -> Result<Vec<Field>> {
        let files = self.all_json_files();
        let fields: Vec<Field> = files
            .into_iter()
            .filter_map(|f| self.read_field(&f).ok())
            .collect();
        Ok(fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Field;
    use tempfile::TempDir;

    async fn test_provider() -> (FileProvider, TempDir) {
        let dir = TempDir::new().unwrap();
        let provider = FileProvider::new(dir.path().to_path_buf());
        (provider, dir)
    }

    #[tokio::test]
    async fn write_and_load() {
        let (provider, _dir) = test_provider().await;
        let field = Field::new("project/test/key1", "hello world", "test");
        provider.write(&field).await.unwrap();
        let loaded = provider.load("project/test").await.unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].value, "hello world");
    }

    #[tokio::test]
    async fn search_finds_match() {
        let (provider, _dir) = test_provider().await;
        provider
            .write(&Field::new("a/b", "rust compiler error", "test"))
            .await
            .unwrap();
        provider
            .write(&Field::new("a/c", "python import issue", "test"))
            .await
            .unwrap();
        let results = provider.search("rust").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].path, "a/b");
    }

    #[tokio::test]
    async fn delete_removes_field() {
        let (provider, _dir) = test_provider().await;
        provider
            .write(&Field::new("x/y", "val", "test"))
            .await
            .unwrap();
        provider.delete("x/y").await.unwrap();
        let loaded = provider.load("x").await.unwrap();
        assert!(loaded.is_empty());
    }

    #[tokio::test]
    async fn load_empty_returns_empty() {
        let (provider, _dir) = test_provider().await;
        let loaded = provider.load("nonexistent").await.unwrap();
        assert!(loaded.is_empty());
    }

    #[tokio::test]
    async fn export_all_returns_everything() {
        let (provider, _dir) = test_provider().await;
        provider
            .write(&Field::new("a/1", "v1", "test"))
            .await
            .unwrap();
        provider
            .write(&Field::new("b/2", "v2", "test"))
            .await
            .unwrap();
        let all = provider.export_all().await.unwrap();
        assert_eq!(all.len(), 2);
    }
}
