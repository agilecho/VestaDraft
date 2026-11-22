use std::path::Path;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let mut args = std::env::args_os().skip(1);
    if args.next().as_deref() != Some(std::ffi::OsStr::new("contracts-check"))
        || args.next().is_some()
    {
        eprintln!("Usage: cargo xtask contracts-check");
        return ExitCode::from(2);
    }

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let python = std::env::var_os("VESTA_PYTHON").unwrap_or_else(|| "python3".into());
    let checks: [(std::ffi::OsString, &[&str]); 2] = [
        ("node".into(), &["scripts/test-contracts.mjs"]),
        (python, &["-B", "scripts/generate-source7.py", "--check"]),
    ];
    for (program, args) in checks {
        match Command::new(&program)
            .args(args)
            .current_dir(&root)
            .status()
        {
            Ok(status) if status.success() => {}
            Ok(status) => {
                eprintln!("{program:?} {args:?} exited with {status}");
                return ExitCode::from(
                    status
                        .code()
                        .and_then(|code| code.try_into().ok())
                        .unwrap_or(1),
                );
            }
            Err(error) => {
                eprintln!("Could not run {program:?} {args:?}: {error}");
                return ExitCode::FAILURE;
            }
        }
    }
    ExitCode::SUCCESS
}
