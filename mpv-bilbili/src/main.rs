pub mod check;

use clap::Arg;
use clap::App;
use std::process;
use std::process::Stdio;

use b2m::*;

const NAME: &str = "mpv-bilibili";
const VERSION: &str = "0.6.2";
const DESCRIPTION: &str = "play bilibili video with mpv";

fn main() {
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
            println!("you-get checking succeeded");
        } else {
            println!("you-get checking failed");
        }
        println!();
        if check::check_mpv() {
          println!("mpv checking succeeded");
        } else {
          println!("mpv checking failed");
        }
        process::exit(0);
    }
    let url = match matches.value_of("url") {
        Some(url) => String::from(url),
        None => panic!("Invaild input"),
    };
    let sto = if matches.is_present("debug") {
        Stdio::inherit()
    } else {
        Stdio::null()
    };
    let media_info = get_url(&url).unwrap();
    play_with_mpv(media_info, sto).unwrap();
}
