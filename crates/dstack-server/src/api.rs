use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use dstack::config::Config;
use dstack_memory::{Field, MemoryProvider};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub struct AppState {
    pub config: Config,
}

pub fn router(config: Config) -> Router {
    let state = Arc::new(AppState { config });

    Router::new()
        .route("/health", get(health))
        .route("/config", get(config_info))
        .route("/memory/load", post(memory_load))
        .route("/memory/save", post(memory_save))
        .route("/memory/query", post(memory_query))
        .route("/memory/export", get(memory_export))
        .route("/sync/status", get(sync_status))
        .route("/audit", get(audit_summary))
        .route("/audit/stale", get(audit_stale))
        .route("/skills", get(skills_list))
        .route("/skills/install", post(skills_install))
        .route("/skills/sync", post(skills_sync))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

// --- Types ---

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

#[derive(Serialize)]
struct ConfigResponse {
    memory_backend: String,
    tracked_repos: Vec<String>,
    deploy_targets: Vec<String>,
}

#[derive(Deserialize)]
struct MemoryLoadRequest {
    #[serde(default)]
    project: String,
}

#[derive(Deserialize)]
struct MemorySaveRequest {
    key: String,
    value: String,
}

#[derive(Deserialize)]
struct MemoryQueryRequest {
    pattern: String,
}

#[derive(Serialize)]
struct FieldResponse {
    path: String,
    value: String,
    confidence: f64,
    source: String,
}

#[derive(Deserialize)]
struct SkillInstallRequest {
    name: String,
}

#[derive(Serialize)]
struct MessageResponse {
    message: String,
}

// --- Helpers ---

fn field_to_response(f: &Field) -> FieldResponse {
    FieldResponse {
        path: f.path.clone(),
        value: f.value.clone(),
        confidence: f.confidence,
        source: f.source.clone(),
    }
}

async fn make_provider(
    cfg: &Config,
) -> Result<Box<dyn MemoryProvider>, (StatusCode, Json<MessageResponse>)> {
    dstack::cmd_memory::provider_from_config(cfg)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(MessageResponse {
                    message: e.to_string(),
                }),
            )
        })
}

// --- Handlers ---

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
    })
}

async fn config_info(State(state): State<Arc<AppState>>) -> Json<ConfigResponse> {
    let cfg = &state.config;
    Json(ConfigResponse {
        memory_backend: cfg.memory.backend.clone(),
        tracked_repos: cfg.repos.tracked.clone(),
        deploy_targets: cfg.deploy.keys().cloned().collect(),
    })
}

async fn memory_load(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MemoryLoadRequest>,
) -> Result<Json<Vec<FieldResponse>>, (StatusCode, Json<MessageResponse>)> {
    let provider = make_provider(&state.config).await?;
    let fields = provider.load(&req.project).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse {
                message: e.to_string(),
            }),
        )
    })?;
    Ok(Json(fields.iter().map(field_to_response).collect()))
}

async fn memory_save(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MemorySaveRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<MessageResponse>)> {
    let provider = make_provider(&state.config).await?;
    let field = Field::new(&req.key, &req.value, "api").with_confidence(0.9);
    provider.write(&field).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse {
                message: e.to_string(),
            }),
        )
    })?;
    Ok(Json(MessageResponse {
        message: format!("Saved: {}", req.key),
    }))
}

async fn memory_query(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MemoryQueryRequest>,
) -> Result<Json<Vec<FieldResponse>>, (StatusCode, Json<MessageResponse>)> {
    let provider = make_provider(&state.config).await?;
    let results = provider.search(&req.pattern).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse {
                message: e.to_string(),
            }),
        )
    })?;
    Ok(Json(results.iter().map(field_to_response).collect()))
}

async fn memory_export(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<FieldResponse>>, (StatusCode, Json<MessageResponse>)> {
    let provider = make_provider(&state.config).await?;
    let all = provider.export_all().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse {
                message: e.to_string(),
            }),
        )
    })?;
    Ok(Json(all.iter().map(field_to_response).collect()))
}

async fn sync_status(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<serde_json::Value>> {
    let cfg = &state.config;
    let mut repos = Vec::new();

    for repo in &cfg.repos.tracked {
        let path = format!("{}/{}", cfg.repos.root, repo);
        if !std::path::Path::new(&path).exists() {
            repos.push(serde_json::json!({
                "name": repo, "status": "not_found"
            }));
            continue;
        }

        let branch = git_output(&path, &["branch", "--show-current"]);
        let dirty = git_output(&path, &["status", "--porcelain"]);
        let dirty_count = dirty.lines().filter(|l| !l.is_empty()).count();

        repos.push(serde_json::json!({
            "name": repo,
            "branch": branch.trim(),
            "dirty_files": dirty_count,
            "status": if dirty_count > 0 { "dirty" } else { "clean" }
        }));
    }

    Json(repos)
}

async fn audit_summary(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let cfg = &state.config;
    let total = cfg.repos.tracked.len();
    let mut dirty = 0;
    for repo in &cfg.repos.tracked {
        let path = format!("{}/{}", cfg.repos.root, repo);
        if std::path::Path::new(&path).exists() {
            let output = git_output(&path, &["status", "--porcelain"]);
            if !output.trim().is_empty() {
                dirty += 1;
            }
        }
    }

    Json(serde_json::json!({
        "total_repos": total,
        "dirty_repos": dirty,
        "clean_repos": total - dirty,
        "deploy_targets": cfg.deploy.len(),
        "memory_backend": cfg.memory.backend,
    }))
}

async fn audit_stale(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<serde_json::Value>> {
    let cfg = &state.config;
    let now = std::time::SystemTime::now();
    let mut stale = Vec::new();

    for repo in &cfg.repos.tracked {
        let path = format!("{}/{}", cfg.repos.root, repo);
        if let Ok(output) = std::process::Command::new("find")
            .args([&path, "-name", "*.implementation.md", "-type", "f"])
            .output()
        {
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                if line.is_empty() { continue; }
                if let Ok(meta) = std::fs::metadata(line) {
                    if let Ok(modified) = meta.modified() {
                        if let Ok(age) = now.duration_since(modified) {
                            let days = age.as_secs() / 86400;
                            if days > 14 {
                                stale.push(serde_json::json!({
                                    "file": line, "days": days, "status": "stale"
                                }));
                            }
                        }
                    }
                }
            }
        }
    }

    Json(stale)
}

async fn skills_list(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, Json<MessageResponse>)> {
    let repo_path = state.config.skills_repo.as_deref().ok_or_else(|| {
        (StatusCode::BAD_REQUEST, Json(MessageResponse { message: "skills_repo not configured".into() }))
    })?;

    let mut skills = Vec::new();
    if let Ok(entries) = std::fs::read_dir(repo_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && path.join("SKILL.md").exists() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    let installed = dirs::home_dir()
                        .map(|h| h.join(".claude/skills").join(name).join("SKILL.md").exists())
                        .unwrap_or(false);
                    skills.push(serde_json::json!({
                        "name": name,
                        "installed": installed,
                    }));
                }
            }
        }
    }
    skills.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
    Ok(Json(skills))
}

async fn skills_install(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SkillInstallRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<MessageResponse>)> {
    dstack::cmd_skills::install(&state.config, &req.name).map_err(|e| {
        (StatusCode::BAD_REQUEST, Json(MessageResponse { message: e.to_string() }))
    })?;
    Ok(Json(MessageResponse { message: format!("Installed: {}", req.name) }))
}

async fn skills_sync(
    State(state): State<Arc<AppState>>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<MessageResponse>)> {
    dstack::cmd_skills::sync_all(&state.config).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(MessageResponse { message: e.to_string() }))
    })?;
    Ok(Json(MessageResponse { message: "Skills synced".into() }))
}

fn git_output(repo_path: &str, args: &[&str]) -> String {
    std::process::Command::new("git")
        .args([&["-C", repo_path], args].concat())
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default()
}
