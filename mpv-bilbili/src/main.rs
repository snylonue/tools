use clap::Arg;
use clap::App;
use std::process::Stdio;

use b2m::*;

fn main() {
    let matches = App::new("mpv-bilibili")
                      .version("0.4.4")
                      .about("play bilibili video with mpv")
                      .arg(Arg::with_name("url")
                        .help("video url")
                        .index(1)
                        .required(true))
                      .arg(Arg::with_name("debug")
                        .help("run with stdout from mpv (may not work)")
                        .long("debug")
                        .multiple(true))
                      .get_matches();
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
