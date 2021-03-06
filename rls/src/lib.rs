//! The Rust Language Server.
//!
//! The RLS provides a server that runs in the background, providing IDEs,
//! editors, and other tools with information about Rust programs. It supports
//! functionality such as 'goto definition', symbol search, reformatting, and
//! code completion, and enables renaming and refactorings.

#![feature(rustc_private, integer_atomics, drain_filter, hash_raw_entry)]
#![allow(unknown_lints)]
#![warn(clippy::all, rust_2018_idioms)]
#![allow(clippy::cyclomatic_complexity, clippy::too_many_arguments)]

pub use rls_analysis::{AnalysisHost, Target};
pub use rls_vfs::Vfs;

pub mod actions;
pub mod build;
pub mod cmd;
pub mod concurrency;
pub mod config;
pub mod lsp_data;
pub mod project_model;
pub mod server;

type Span = rls_span::Span<rls_span::ZeroIndexed>;

pub const RUSTC_SHIM_ENV_VAR_NAME: &str = "RLS_RUSTC_SHIM";

pub fn version() -> String {
    use rustc_tools_util::VersionInfo;

    rustc_tools_util::get_version_info!().to_string()
}
