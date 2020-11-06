//! Utility that acquires a POSIX lock on a file, before running a command.
//! Similar to the Linux command `flock`, but using POSIX locks instead of BSD
//! locks.

use file_lock::FileLock;
use std::io::Result;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Utility that acquires a POSIX lock on a file, before \
                     running a command.\n\n\
                     Similar to the Linux command `flock`, but using POSIX \
                     locks instead of BSD locks.")]
struct Opt {
    #[structopt(parse(from_os_str))]
    lock_file: PathBuf,

    #[structopt()]
    command: Vec<String>,

    #[structopt(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let lock_file_path =
        opt.lock_file.to_str().expect("not a valid unicode path");

    if opt.verbose {
        println!("Lock file: `{}`", lock_file_path);
        if !opt.command.is_empty() {
            println!("Command:   `{}`", opt.command.join(" "));
        }
        println!("");
    }

    let _lock = FileLock::lock(lock_file_path, true, true)?;

    if opt.verbose {
        println!("Lock acquired");
    }

    if !opt.command.is_empty() {
        let program = opt.command.first().unwrap();
        let args = &opt.command[1..];
        Command::new(program)
            .args(args)
            .spawn()
            .expect("command failed to start");
    }
    Ok(())
}
