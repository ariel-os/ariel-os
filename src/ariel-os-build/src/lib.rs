//! Ariel OS build-time tools.

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
