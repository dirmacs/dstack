use crate::config::Config;
use dstack_memory::{Field, MemoryProvider};

pub async fn provider_from_config(cfg: &Config) -> anyhow::Result<Box<dyn MemoryProvider>> {
    match cfg.memory.backend.as_str() {
        "eruka" => {
            let key = cfg
                .eruka_service_key()
                .unwrap_or_default();
            Ok(Box::new(dstack_memory::eruka::ErukaProvider::new(
                &cfg.memory.eruka.url,
                &key,
            )))
        }
        _ => {
            let path = cfg.memory_path();
            std::fs::create_dir_all(&path)?;
            Ok(Box::new(dstack_memory::file::FileProvider::new(path)))
        }
    }
}

pub async fn load(cfg: &Config, project: Option<&str>) -> anyhow::Result<()> {
    let provider = provider_from_config(cfg).await?;
    let path = project.unwrap_or("");
    let fields = provider.load(path).await?;
    if fields.is_empty() {
        eprintln!(
            "No memory fields found for: {}",
            if path.is_empty() { "(all)" } else { path }
        );
        return Ok(());
    }
    for f in &fields {
        println!("[{:.0}%] {} = {}", f.confidence * 100.0, f.path, f.value);
    }
    eprintln!("\n{} field(s) loaded.", fields.len());
    Ok(())
}

pub async fn save(cfg: &Config, key: &str, value: &str) -> anyhow::Result<()> {
    let provider = provider_from_config(cfg).await?;
    let field = Field::new(key, value, "user").with_confidence(0.9);
    provider.write(&field).await?;
    eprintln!("Saved: {} = {}", key, value);
    Ok(())
}

pub async fn query(cfg: &Config, pattern: &str) -> anyhow::Result<()> {
    let provider = provider_from_config(cfg).await?;
    let results = provider.search(pattern).await?;
    if results.is_empty() {
        eprintln!("No matches for: {}", pattern);
        return Ok(());
    }
    for f in &results {
        println!("[{:.0}%] {} = {}", f.confidence * 100.0, f.path, f.value);
    }
    eprintln!("\n{} match(es).", results.len());
    Ok(())
}

pub async fn export(cfg: &Config) -> anyhow::Result<()> {
    let provider = provider_from_config(cfg).await?;
    let all = provider.export_all().await?;
    println!("{}", serde_json::to_string_pretty(&all)?);
    Ok(())
}
