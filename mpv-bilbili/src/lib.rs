use std::process;
use std::process::Stdio;

type Res<T> = Result<T, &'static str>;

pub fn get_url(orig_url: &String) -> Res<Vec<String>> {
    let cmd = match process::Command::new("you-get").arg(orig_url).arg("-u").output() {
        Ok(r) => r,
        Err(_) => return Err("Failed to run command"),
    };
    let sto = match String::from_utf8(cmd.stdout) {
        Ok(r) => r,
        Err(_) => return Err("can not parse stdout"),
    };
    let mut sto = sto.split("\n").map(|x| { x.trim().to_string() });
    if let None = sto.position(|x| { x.trim() == "Real URLs:" }) {
        return Err("can not get real url");
    }
    let res: Vec<_> = sto.filter(|x| { x.trim() != "" }).map(|x| { x.trim().to_string() }).collect();
    Ok(res)
}
pub fn play_with_mpv(orig_url: &String, sto: Stdio) -> Res<()> {
	let url = get_url(orig_url)?;
	let mut cmd = process::Command::new("mpv");
	if url.len() == 2 {
		cmd.arg(&url[0]).arg(format!("--audio-file={}", url[1]));
	} else {
		cmd.arg(url.join(",")).arg("--merge-files");
	}
	cmd.arg("--referrer=https://www.bilibili.com")
	   .arg("--no-ytdl")
	   .stdout(sto)
	   .spawn().expect("Failed to spawn child process")
	   .wait_with_output().expect("Failed to run command");
	Ok(())
}