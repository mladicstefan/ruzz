use nix::sys::ptrace;
use nix::sys::wait::waitpid;
use nix::unistd::{fork, ForkResult, Pid};
use std::error::Error;
use std::os::unix::process::CommandExt;
use std::process::Command;

fn compile_target(target: &str, binary_name: &str) -> Command {
    let mut command = Command::new("gcc");
    command.args(&[
        "-fsanitize=address",
        "-fsanitize=leak",
        "-fsanitize=undefined",
        "-g",
        "-O0",
        "-fno-omit-frame-pointer",
        target,
        "-o",
        binary_name,
    ]);
    command
}

fn spawn_traced_child(mut command: Command) -> Result<Pid, Box<dyn Error>> {
    match unsafe { fork() }? {
        ForkResult::Parent { child } => Ok(child),
        ForkResult::Child => {
            ptrace::traceme()?;
            let err = command.exec();
            //this should be uncreachable, unless err
            eprintln!("exec failed: {}", err);
            std::process::exit(1);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let target = "vuln.c";
    let binary_name = "vuln";

    // compile the target
    let mut compile_cmd = compile_target(target, binary_name);
    let compile_status = compile_cmd.status()?;
    if !compile_status.success() {
        return Err("Compilation failed".into());
    }

    // run the compiled binary under trace
    let run_cmd = Command::new(format!("./{}", binary_name));
    let child_pid = spawn_traced_child(run_cmd)?;

    println!("Forked process PID: {}", child_pid);

    let status = waitpid(child_pid, None)?;
    println!("Fork status: {:?}", status);

    Ok(())
}
