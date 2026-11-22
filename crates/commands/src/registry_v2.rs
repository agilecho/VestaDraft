pub const CORE_COMMANDS: [&str; 11] = [
    "cell.set",
    "site.add",
    "site.setFractional",
    "history.undo",
    "history.redo",
    "display.setHidden",
    "boundary.set",
    "view.setCamera",
    "domain.prepare",
    "domain.state.prepare",
    "domain.apply",
];
pub const METHODS: [&str; 17] = [
    "hello",
    "status",
    "cancel",
    "resource.begin",
    "resource.chunk",
    "resource.commit",
    "resource.read",
    "resource.release",
    "open",
    "new",
    "inspect",
    "execute",
    "snapshot",
    "save.confirm",
    "save.reject",
    "close",
    "shutdown",
];
pub fn history_effect(command: &str) -> &'static str {
    match command {
        "history.undo" | "history.redo" => "navigate-history",
        _ => "record",
    }
}
