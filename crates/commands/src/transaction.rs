//! EngineV2::batch prepares all basic commands against a private content/ID copy and publishes once.
pub fn is_atomic_command(c: &str) -> bool {
    matches!(
        c,
        "cell.set"
            | "site.add"
            | "site.setFractional"
            | "display.setHidden"
            | "boundary.set"
            | "view.setCamera"
    )
}
