use craft_testkit::with_deadline;
use std::process::Command;
use std::time::Duration;
#[test]
fn real_cli_registers_ns2_without_display() {
    let cli = std::env::var_os("VESTA_TEST_CLI")
        .unwrap_or_else(|| env!("CARGO_BIN_EXE_vestacraft-cli").into());
    let o = with_deadline(Duration::from_secs(10), "CLI registration", || {
        Command::new(cli)
            .args(["commands", "--json"])
            .output()
            .unwrap()
    });
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    let v: serde_json::Value = serde_json::from_slice(&o.stdout).unwrap();
    assert_eq!(v["protocol"], "vestacraft.native-session/2");
    assert_eq!(v["methods"].as_array().unwrap().len(), 17);
    assert_eq!(v["coreCommands"].as_array().unwrap().len(), 11);
}
