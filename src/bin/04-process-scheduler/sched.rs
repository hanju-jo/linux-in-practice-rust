extern crate chrono;
extern crate nix;

use std::env;
use std::process::{exit};
use std::io::{self, BufWriter, Write};

use chrono::{Local, DateTime};
use nix::unistd::{fork, ForkResult};

static NLOOP_FOR_ESTIMATION: i64 = 1_000_000;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: {} <nproc> <total[ms]> <resolution[ms]>", args[0]);
        exit(1);
    }

    let nproc = unwrap_and_validate_arg("nproc", &args[1]);
    let total = unwrap_and_validate_arg("total", &args[2]);
    let resol = unwrap_and_validate_arg("resol", &args[3]);

    if total % resol > 0 {
        eprintln!("<total>({}) should be multiple of <resolution>({})", total, resol);
        exit(1);
    }
    let nrecord: usize = (total / resol) as usize;
    
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    
    writeln!(stdout, "estimating workload which takes just one milliseconds");
    let nloop_per_resol: i64 = loops_per_msec();
    writeln!(stdout, "end estimation, {}", nloop_per_resol);
    stdout.flush();

    let start: DateTime<Local> = Local::now();

    for id in 0..nproc {
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {}
            Ok(ForkResult::Child) => {
                child_fn(&mut stdout, id, nrecord, nloop_per_resol, start);
            }
            Err(_) => println!("fork failed"),
        }
    }
    
}

fn unwrap_and_validate_arg(arg_name: &str, arg: &str) -> u32 {
    match arg.parse::<u32>() {
        Ok(value) => {
           if value < 1 {
                eprintln!("<{}>({}) should be >= 1", arg_name, value);
                exit(1);
            } 
            value
        }
        Err(_) => {
            eprintln!("wrong argument");
            exit(1);  
        }
    }
}

fn loops_per_msec() -> i64 {
    let before: DateTime<Local> = Local::now();
    for _ in 0..NLOOP_FOR_ESTIMATION {

    }
    let after: DateTime<Local> = Local::now();
    let diff_msec: i64 = after.timestamp_millis() - before.timestamp_millis();
    NLOOP_FOR_ESTIMATION / diff_msec
}

fn child_fn(stdout: &mut Write, id: u32, nrecord: usize, nloop_per_resol: i64, start: DateTime<Local>) {
    let mut datetime_vec = Vec::with_capacity(nrecord);
    let mut now: DateTime<Local>;
    for _ in 0..nrecord {
        for _ in 0..nloop_per_resol {

        };
        now = Local::now();
        datetime_vec.push(now);
    }
    for (i, now) in datetime_vec.iter().enumerate() {
       let diff_msec: i64 = now.timestamp_millis() - start.timestamp_millis();
        writeln!(stdout, "{}\t{}\t{}", id, diff_msec, (i+1)*100/nrecord);  
    }
    stdout.flush();
    exit(1);
}
