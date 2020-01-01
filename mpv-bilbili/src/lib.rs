use std::process;
use std::process::Stdio;

type Res<T> = Result<T, String>;

pub fn get_url(orig_url: &String) -> Res<Vec<String>> {
    let cmd = match process::Command::new("you-get").arg(orig_url).arg("-u").output() {
        Ok(r) => {
            r
        },
        Err(e) => return Err(format!("{:?}", e)),
    };
    let sto = match String::from_utf8(cmd.stdout) {
        Ok(r) => {
            r
        },
        Err(e) => return Err(format!("{:?}", e)),
    };
    let stos = sto.clone();
    let mut stos = stos.split("\n").map(|x| { x.trim().to_string() });
    if let None = stos.position(|x| { x.trim() == "Real URLs:" }) {
        return Err(format!("{}", "failed to parse stdout as url"));
    }
    let res: Vec<_> = stos.filter(|x| { x.trim() != "" }).map(|x| { x.trim().to_string() }).collect();
    Ok(res)
}
pub fn play_with_mpv(orig_url: &String, sto: Stdio) -> Res<()> {
    let url = get_url(orig_url)?;
    let mut cmd = process::Command::new("mpv");
    for i in url.iter() {
        cmd.arg(i);
    }
    cmd.arg("--merge-files")
        .arg("--referrer=https://www.bilibili.com")
        .arg("--no-ytdl")
        .stdout(sto)
        .spawn().expect("Failed to spawn child process")
	    .wait_with_output().expect("Failed to run command");
	Ok(())
}