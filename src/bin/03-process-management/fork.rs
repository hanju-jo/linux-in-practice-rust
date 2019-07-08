extern crate nix;

use nix::libc::{EXIT_FAILURE, EXIT_SUCCESS};
use nix::unistd::{fork, getpid, ForkResult, Pid};
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
    exit(EXIT_SUCCESS);
}

fn parent(pid_child: Pid) {
    println!(
        "I'm parent! My pid is {} and the pid of my child is {}",
        getpid(),
        pid_child
    );
    exit(EXIT_SUCCESS);
}
