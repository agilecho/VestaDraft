use std::env;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let Ok(cwd) = env::current_dir() else {
        eprintln!("BLOCKED xtask: cannot determine the working directory");
        return ExitCode::from(2);
    };
    let Some(root) = cwd
        .ancestors()
        .find(|path| path.join("xtask/ci.py").is_file())
    else {
        eprintln!("BLOCKED xtask: run from the VestaDraft checkout");
        return ExitCode::from(2);
    };

    let python = env::var_os("VESTA_PYTHON").unwrap_or_else(|| "python3".into());
    match Command::new(&python)
        .args(["-B", "xtask/ci.py"])
        .args(env::args_os().skip(1))
        .current_dir(root)
        .status()
    {
        Ok(status) => ExitCode::from(
            status
                .code()
                .and_then(|code| code.try_into().ok())
                .unwrap_or(1),
        ),
        Err(error) => {
            eprintln!("BLOCKED xtask: cannot start {python:?}: {error}");
            ExitCode::from(2)
        }
    }
}
