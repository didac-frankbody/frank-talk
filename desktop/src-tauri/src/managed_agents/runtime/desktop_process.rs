//! Recognising *this* product's own desktop process.
//!
//! Dead-instance detection scans the user's processes to decide whether the
//! desktop that owns a managed agent is still alive. Getting this wrong in the
//! false direction reaps a live desktop's agents, so the name list is kept here
//! on its own rather than buried in the runtime module.

/// Binary names for the desktop/Tauri process.
///
/// Tauri names the executable after `productName`, so the current bundle's
/// process is `frank talk`. The Buzz names stay in the list because an install
/// predating the rename still runs as `Buzz`, and dropping it would read that
/// live desktop as dead.
const DESKTOP_BINARY_NAMES: &[&str] = &[
    "frank talk",
    "frank-talk",
    "frank_talk",
    "Buzz",
    "buzz-desktop",
    "buzz_desktop",
];

/// Whether a process name matches a known desktop binary.
pub(super) fn is_desktop_binary(name: &str) -> bool {
    DESKTOP_BINARY_NAMES.contains(&name)
}

#[cfg(test)]
mod tests {
    use super::is_desktop_binary;

    #[test]
    fn recognises_the_current_and_pre_rename_bundle_names() {
        assert!(is_desktop_binary("frank talk"));
        assert!(is_desktop_binary("Buzz"));
        assert!(is_desktop_binary("buzz-desktop"));
    }

    #[test]
    fn rejects_unrelated_and_partial_names() {
        assert!(!is_desktop_binary("frank"));
        assert!(!is_desktop_binary("talk"));
        assert!(!is_desktop_binary("frank talk helper"));
        assert!(!is_desktop_binary("goose"));
        assert!(!is_desktop_binary(""));
    }
}
