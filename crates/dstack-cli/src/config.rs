use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

/// Top-level dstack configuration.
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub memory: MemoryConfig,
    #[serde(default)]
    pub repos: RepoConfig,
    #[serde(default)]
    pub deploy: HashMap<String, DeployTarget>,
    #[serde(default)]
    pub git: GitConfig,
}

/// Memory backend configuration.
#[derive(Debug, Deserialize)]
pub struct MemoryConfig {
    /// "file" or "eruka"
    #[serde(default = "default_backend")]
    pub backend: String,
    /// Path to local memory store (supports ~ expansion)
    #[serde(default = "default_memory_path")]
    pub path: String,
    /// Eruka-specific settings (only used when backend = "eruka")
    #[serde(default)]
    pub eruka: ErukaConfig,
}

/// Eruka memory backend configuration.
#[derive(Debug, Deserialize)]
pub struct ErukaConfig {
    #[serde(default = "default_eruka_url")]
    pub url: String,
    /// Service key for Eruka API authentication.
    /// Can also be set via $DSTACK_ERUKA_KEY env var (takes precedence).
    #[serde(default)]
    pub service_key: Option<String>,
}

/// Repository discovery and tracking configuration.
#[derive(Debug, Deserialize)]
pub struct RepoConfig {
    /// Root directory to scan for repos
    #[serde(default = "default_repo_root")]
    pub root: String,
    /// Explicit list of tracked repo names
    #[serde(default)]
    pub tracked: Vec<String>,
}

/// Deployment target for a service.
#[derive(Debug, Deserialize)]
pub struct DeployTarget {
    /// Build command (e.g., "cargo build --release")
    pub build: String,
    /// Systemd service name (e.g., "ares")
    pub service: String,
    /// Optional smoke test command
    pub smoke: Option<String>,
}

/// Git authorship configuration.
#[derive(Debug, Deserialize)]
pub struct GitConfig {
    #[serde(default)]
    pub author_name: Option<String>,
    #[serde(default)]
    pub author_email: Option<String>,
}

// --- Defaults ---

fn default_backend() -> String {
    "file".to_string()
}

fn default_memory_path() -> String {
    "~/.dstack/memory".to_string()
}

fn default_eruka_url() -> String {
    "http://localhost:8081".to_string()
}

fn default_repo_root() -> String {
    "/opt".to_string()
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            backend: default_backend(),
            path: default_memory_path(),
            eruka: ErukaConfig::default(),
        }
    }
}

impl Default for ErukaConfig {
    fn default() -> Self {
        Self {
            url: default_eruka_url(),
            service_key: None,
        }
    }
}

impl Default for RepoConfig {
    fn default() -> Self {
        Self {
            root: default_repo_root(),
            tracked: Vec::new(),
        }
    }
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            author_name: None,
            author_email: None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            memory: MemoryConfig::default(),
            repos: RepoConfig::default(),
            deploy: HashMap::new(),
            git: GitConfig::default(),
        }
    }
}

// --- Config loading ---

/// Returns the path to the dstack config file: ~/.config/dstack/config.toml
pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("dstack")
        .join("config.toml")
}

impl Config {
    /// Load configuration from ~/.config/dstack/config.toml.
    /// Returns defaults if the file does not exist.
    pub fn load() -> anyhow::Result<Self> {
        let path = config_path();
        if path.exists() {
            let contents = std::fs::read_to_string(&path)?;
            let config: Config = toml::from_str(&contents)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    /// Returns the memory path with ~ expanded to the user's home directory.
    pub fn memory_path(&self) -> PathBuf {
        expand_tilde(&self.memory.path)
    }

    /// Returns the Eruka service key.
    /// Checks $DSTACK_ERUKA_KEY env var first, falls back to config file value.
    pub fn eruka_service_key(&self) -> Option<String> {
        std::env::var("DSTACK_ERUKA_KEY")
            .ok()
            .or_else(|| self.memory.eruka.service_key.clone())
    }
}

/// Expand leading ~ to the user's home directory.
fn expand_tilde(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest);
        }
    } else if path == "~" {
        if let Some(home) = dirs::home_dir() {
            return home;
        }
    }
    PathBuf::from(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let cfg = Config::default();
        assert_eq!(cfg.memory.backend, "file");
        assert_eq!(cfg.memory.path, "~/.dstack/memory");
        assert_eq!(cfg.repos.root, "/opt");
        assert!(cfg.repos.tracked.is_empty());
        assert!(cfg.deploy.is_empty());
    }

    #[test]
    fn test_expand_tilde() {
        let expanded = expand_tilde("~/.dstack/memory");
        // Should not start with ~ after expansion (unless no home dir)
        if dirs::home_dir().is_some() {
            assert!(!expanded.to_string_lossy().starts_with('~'));
            assert!(expanded.to_string_lossy().ends_with(".dstack/memory"));
        }
    }

    #[test]
    fn test_expand_tilde_no_prefix() {
        let expanded = expand_tilde("/absolute/path");
        assert_eq!(expanded, PathBuf::from("/absolute/path"));
    }

    #[test]
    fn test_parse_minimal_toml() {
        let toml_str = r#"
[memory]
backend = "eruka"

[repos]
root = "/home/user/projects"
tracked = ["ares", "eruka"]
"#;
        let cfg: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.memory.backend, "eruka");
        assert_eq!(cfg.repos.root, "/home/user/projects");
        assert_eq!(cfg.repos.tracked, vec!["ares", "eruka"]);
    }

    #[test]
    fn test_parse_full_toml() {
        let toml_str = r#"
[memory]
backend = "eruka"
path = "/custom/memory"

[memory.eruka]
url = "https://eruka.example.com"
service_key = "secret123"

[repos]
root = "/opt"
tracked = ["ares", "eruka", "doltares"]

[deploy.ares]
build = "cargo build --release"
service = "ares"
smoke = "curl -sf http://localhost:3000/health"

[deploy.eruka]
build = "cargo build --release"
service = "eruka"

[git]
author_name = "bkataru"
author_email = "baalateja.k@gmail.com"
"#;
        let cfg: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.memory.backend, "eruka");
        assert_eq!(cfg.memory.eruka.url, "https://eruka.example.com");
        assert_eq!(
            cfg.memory.eruka.service_key,
            Some("secret123".to_string())
        );
        assert_eq!(cfg.repos.tracked.len(), 3);
        assert!(cfg.deploy.contains_key("ares"));
        assert!(cfg.deploy.contains_key("eruka"));
        assert_eq!(
            cfg.deploy["ares"].smoke,
            Some("curl -sf http://localhost:3000/health".to_string())
        );
        assert!(cfg.deploy["eruka"].smoke.is_none());
        assert_eq!(cfg.git.author_name, Some("bkataru".to_string()));
    }

    #[test]
    fn test_eruka_service_key_env_override() {
        let cfg = Config::default();
        // Without env var, should return None (no config file key set)
        // We can't reliably test env var override without setting it,
        // but we verify the fallback path works.
        let key = cfg.eruka_service_key();
        if std::env::var("DSTACK_ERUKA_KEY").is_err() {
            assert!(key.is_none());
        }
    }

    #[test]
    fn test_config_path() {
        let path = config_path();
        assert!(path.to_string_lossy().contains("dstack"));
        assert!(path.to_string_lossy().ends_with("config.toml"));
    }

    #[test]
    fn test_load_returns_defaults_when_no_file() {
        // config_path() likely doesn't exist in test env
        let cfg = Config::load().unwrap();
        assert_eq!(cfg.memory.backend, "file");
    }
}
