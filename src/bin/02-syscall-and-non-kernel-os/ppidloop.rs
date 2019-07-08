extern crate nix;

use nix::unistd::getppid;

fn main() {
    loop {
        println!("{}", getppid());
    }
}
