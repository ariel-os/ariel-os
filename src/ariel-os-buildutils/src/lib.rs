//! Ariel OS build-time tools.
use std::env;
use std::path::PathBuf;

/// Returns the first of the given contexts that is in the current `cfg` contexts.
pub fn context_any(contexts: &[&'static str]) -> Option<&'static str> {
    // Contexts cannot include commas.
    contexts.iter().find(|c| context(c)).copied()
}

/// Returns whether the given context is in the current 'cfg' contexts.
pub fn context(context: &'static str) -> bool {
    let Ok(context_var) = std::env::var("CARGO_CFG_CONTEXT") else {
        return false;
    };

    // Contexts cannot include commas.
    context_var.split(',').any(|c| c == context)
}

/// Tells Cargo to re-run the build script if the file has changed.
pub fn rerun_if_changed(file: &str) {
    println!("cargo::rerun-if-changed={file}");
}

/// Tells Cargo to re-run the build script if the environment variable has changed.
pub fn rerun_if_env_changed(env: &str) {
    println!("cargo::rerun-if-env-changed={env}");
}

/// Copies over the file to the `OUT_DIR` and tells cargo to re-run the build script if this file
/// has changed.
pub fn copy_and_rerun_if_changed(file: impl AsRef<std::path::Path>) {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let file = file.as_ref();
    std::fs::copy(file, out.join(file.file_name().unwrap())).unwrap();
    rerun_if_changed(file.to_str().unwrap())
}

/// Fetches the environment variable key from the current process and tells cargo to re-run the
/// build script if this variable has changed.
pub fn env_var_and_rerun_if_changed(key: &str) -> Result<String, std::env::VarError> {
    rerun_if_env_changed(key);
    std::env::var(key)
}
