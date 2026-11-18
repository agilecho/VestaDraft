use p1_ca_bootstrap::{copy, fs_guard, *};
use std::{path::PathBuf, process::ExitCode};

fn run() -> Result<PreparedReceipt> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    require((args.len() == 2 || args.len() == 3 && args[2] == "--verify-only") && args[0] == "--plan", "cli", "usage-plan-absolute-path-verify-only")?;
    let p = read_plan(&PathBuf::from(&args[1]))?;
    if args.len() == 3 {
        let receipt = copy::load_receipt(&p, true)?;
        fs_guard::absolute(&p.workspace_root, false).map_err(|_| BootstrapError::new("verify", "root"))?;
        let ws = craft_anything_engine::workspace::Workspace::open(&p.workspace_root).map_err(|_| BootstrapError::new("verify", "manifest"))?;
        return verify_prepared(&ws, &receipt);
    }
    let (ws, inputs) = if copy::state_dir(&p).try_exists().map_err(|_| BootstrapError::new("cli", "state"))? {
        copy::resume(&p)?
    } else {
        let inputs = preflight(&p)?;
        let ws = create(&p, &inputs)?;
        (ws, inputs)
    };
    let receipt = copy_study(&ws, &inputs)?;
    verify_prepared(&ws, &receipt)
}
fn main() -> ExitCode {
    match run() {
        Ok(receipt) => match serde_json::to_string(&receipt) {
            Ok(s) => {
                println!("{s}");
                ExitCode::SUCCESS
            }
            Err(_) => {
                eprintln!("{{\"stage\":\"output\",\"code\":\"serialize\"}}");
                ExitCode::from(1)
            }
        },
        Err(e) => {
            eprintln!("{{\"stage\":\"{}\",\"code\":\"{}\"}}", e.stage, e.code);
            ExitCode::from(1)
        }
    }
}
