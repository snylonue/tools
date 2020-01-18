pub mod check;

use clap::Arg;
use clap::App;
use failure::Error;
use std::process;
use std::process::Stdio;

use b2m::*;

const NAME: &str = "mpv-bilibili";
const VERSION: &str = "0.9.0";
const DESCRIPTION: &str = "play bilibili video with mpv";

fn main() -> Result<(), Error> {
    let matches = App::new(NAME)
                      .version(VERSION)
                      .about(DESCRIPTION)
                      .arg(Arg::with_name("url")
                          .help("video url")
                          .index(1)
                          .required_unless("check")
                      )
                      .arg(Arg::with_name("debug")
                          .help("run with stdout from mpv (may not work)")
                          .long("debug")
                          .multiple(true)
                      )
                      .arg(Arg::with_name("check")
                          .help("check if all dependencies are installed")
                          .short("c")
                          .long("check")
                          .multiple(true)
                    )
                    .get_matches();
    if matches.is_present("check") {
        if check::check_you_get() {
            println!("\nyou-get checking succeeded");
        } else {
            println!("\nyou-get checking failed");
        }
        println!();
        if check::check_mpv() {
          println!("\nmpv checking succeeded");
        } else {
          println!("\nmpv checking failed");
        }
        process::exit(0);
    }
    let url = match matches.value_of("url") {
        Some(url) => String::from(url),
        None => panic!("Invaild input"),
    };
    let stdio = if matches.is_present("debug") {
        Stdio::inherit()
    } else {
        Stdio::null()
    };
    get_url(&url)?.play(stdio)
}
