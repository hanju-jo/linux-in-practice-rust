extern crate nix;

use nix::libc::{EXIT_FAILURE, EXIT_SUCCESS};
use nix::unistd::{execve, fork, getpid, ForkResult, Pid};
use std::ffi::CString;
use std::process::exit;

fn main() {
    match fork() {
        Ok(ForkResult::Parent { child, .. }) => parent(child),
        Ok(ForkResult::Child) => child(),
        Err(_) => {
            println!("fork failed");
            exit(EXIT_FAILURE);
        }
    }
    println!("shouldn't reach here");
    exit(EXIT_FAILURE);
}

fn child() {
    println!("I'm child! My pid is {}", getpid());
    let args: &[CString] = &[
        CString::new("/bin/echo").unwrap(),
        CString::new("hello").unwrap(),
    ];
    let env: &[CString] = &[];
    match execve(&args[0], args, env) {
        Ok(_) => exit(EXIT_SUCCESS),
        Err(_) => println!("execve failed"),
    }
    exit(EXIT_FAILURE);
}

fn parent(pid_child: Pid) {
    println!(
        "I'm parent! My pid is {} and the pid of my child is {}",
        getpid(),
        pid_child
    );
    exit(EXIT_SUCCESS);
}
