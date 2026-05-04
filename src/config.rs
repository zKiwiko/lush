use std::sync::OnceLock;

static VERBOSE: OnceLock<bool> = OnceLock::new();

pub fn set_verbose() {
    VERBOSE.set(true).ok();
}

#[allow(dead_code)]
pub fn is_verbose() -> bool {
    *VERBOSE.get_or_init(|| false)
}
