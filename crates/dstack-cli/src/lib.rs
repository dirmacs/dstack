//! dstack — Development stack for AI-assisted multi-repo work.
//!
//! This crate provides both a CLI binary (`dstack`) and a library for:
//! - Persistent memory (file-based or Eruka-backed)
//! - Multi-repo git sync
//! - Service deployment with smoke tests
//! - Pre-commit quality gates
//!
//! # Example
//!
//! ```rust,no_run
//! use dstack::config::Config;
//!
//! let cfg = Config::load().unwrap();
//! println!("Memory backend: {}", cfg.memory.backend);
//! ```

pub mod config;
pub mod cmd_memory;
pub mod cmd_deploy;
pub mod cmd_skills;
pub mod cmd_sync;
pub mod cmd_audit;
