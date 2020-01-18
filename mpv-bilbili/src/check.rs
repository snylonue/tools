use std::process;

use b2m::parse_output;

const UNKNOWN: &str = "unknown";

pub fn check_you_get() -> bool {
    println!("Checking for you-get");
    println!("Running you-get -V");
    match process::Command::new("you-get")
        .arg("-V")
        .output() {
            Ok(r) => {
                let (stdout, stderr) = match parse_output(r) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("Failed to check for you-get: unable to parse stdout and stderr:\n{:?}", e);
                        return false;
                    },
                };
                let splits = stderr.split(' ').collect::<Vec<_>>();
                let version = splits.get(2).unwrap_or(&UNKNOWN);
                println!("you-get version: {}\n", version);
                println!("{}", format!("Stdout:\n{}", stdout).trim());
                println!("{}", format!("Stderr:\n{}", stderr).trim());
                true
            },
            Err(e) => {
                eprintln!("Failed to check for you-get: unable to run you-get:\n{:?}", e);
                false
            }
        }
}
pub fn check_mpv() -> bool {
    println!("Checking for mpv");
    println!("Running mpv -V");
    match process::Command::new("mpv")
        .arg("-V")
        .output() {
            Ok(r) => {
                let (stdout, stderr) = match parse_output(r) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("Failed to check for mpv: unable to parse stdout and stderr:\n {:?}", e);
                        return false;
                    },
                };
                let splits = stdout.split(' ').collect::<Vec<_>>();
                let version = splits.get(1).unwrap_or(&UNKNOWN);
                println!("mpv version: {}\n", version);
                println!("{}", format!("Stdout:\n{}", stdout).trim());
                println!("{}", format!("Stderr:\n{}", stderr).trim());
                true
            },
            Err(e) => {
                eprintln!("Failed to check for mpv: unable to run mpv:\n{:?}", e);
                false
            }
        }
}