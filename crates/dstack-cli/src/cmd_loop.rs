use crate::config::Config;

pub fn create(
    _cfg: &Config,
    name: &str,
    cron: &str,
    task: &str,
    dry_run: bool,
) -> anyhow::Result<()> {
    validate_name(name)?;
    validate_cron(cron)?;
    if task.trim().is_empty() {
        anyhow::bail!("task prompt must not be empty");
    }

    let payload = serde_json::json!({
        "name": name,
        "cron": cron,
        "task": task,
    });

    if dry_run {
        println!("DRY RUN — would POST to doltares /api/loops/start:");
        println!("{}", serde_json::to_string_pretty(&payload)?);
        return Ok(());
    }

    let endpoint = std::env::var("DOLTARES_URL")
        .unwrap_or_else(|_| "http://localhost:3100".to_string());
    println!("[dstack loop create] endpoint: {endpoint}/api/loops/start");
    println!(
        "[dstack loop create] payload: {}",
        serde_json::to_string(&payload)?
    );
    println!(
        "[dstack loop create] note: doltares /api/loops/start is pending (tracked as a separate task). Request body above is what will be posted once the endpoint lands."
    );
    Ok(())
}

fn validate_name(name: &str) -> anyhow::Result<()> {
    if name.trim().is_empty() {
        anyhow::bail!("loop name must not be empty");
    }
    if name.len() > 64 {
        anyhow::bail!("loop name must be at most 64 characters");
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        anyhow::bail!("loop name must be ASCII alphanumeric plus '-' or '_'");
    }
    Ok(())
}

fn validate_cron(cron: &str) -> anyhow::Result<()> {
    let field_count = cron.split_whitespace().count();
    if !(5..=6).contains(&field_count) {
        anyhow::bail!(
            "cron expression must have 5 or 6 fields, got {field_count}: {cron:?}"
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_name_accepts_alphanumeric_with_separators() {
        assert!(validate_name("my-loop_1").is_ok());
        assert!(validate_name("loop").is_ok());
    }

    #[test]
    fn validate_name_rejects_empty() {
        assert!(validate_name("").is_err());
        assert!(validate_name("   ").is_err());
    }

    #[test]
    fn validate_name_rejects_special_characters() {
        assert!(validate_name("my loop").is_err());
        assert!(validate_name("my/loop").is_err());
        assert!(validate_name("loop!").is_err());
    }

    #[test]
    fn validate_name_rejects_overlong() {
        let long = "a".repeat(65);
        assert!(validate_name(&long).is_err());
    }

    #[test]
    fn validate_cron_accepts_five_fields() {
        assert!(validate_cron("*/3 * * * *").is_ok());
        assert!(validate_cron("0 9 * * 1-5").is_ok());
    }

    #[test]
    fn validate_cron_accepts_six_fields() {
        assert!(validate_cron("0 */3 * * * *").is_ok());
    }

    #[test]
    fn validate_cron_rejects_too_few_fields() {
        assert!(validate_cron("* * * *").is_err());
    }

    #[test]
    fn validate_cron_rejects_too_many_fields() {
        assert!(validate_cron("0 0 0 0 0 0 0").is_err());
    }
}
