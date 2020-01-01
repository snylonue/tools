use serde_json;
use serde_json::Value;
use std::process;
use std::process::Stdio;

type Res<T> = Result<T, String>;

pub fn get_url(orig_url: &String) -> Res<Vec<String>> {
    let (you_get_stdout, you_get_stderr) = match process::Command::new("you-get").arg(orig_url).arg("--json").output() {
        Ok(r) => {
            let stdout = match String::from_utf8(r.stdout) {
                Ok(r) => r,
                Err(e) => return Err(format!("{:?}", e)),
            };
            let stderr = match String::from_utf8(r.stderr) {
                Ok(r) => r,
                Err(e) => String::from(format!("Failed to read stderr: {:?}", e)),
            };
            (stdout, stderr)
        },
        Err(e) => return Err(format!("{:?}", e)),
    };
    //process you-get output as json only
    let stos = you_get_stdout.split("{")
        .map(|x| { x.trim().to_string() + "{" })
        .fold(String::new(), |mul, x| { mul + &x });
    let stos = stos.trim_end_matches('{');
    let json_stdout = serde_json::from_str(stos);
    //res = json_stdout['streams']['flv']['src']
    let res = match json_stdout {
        Ok(j) => match j {
            Value::Object(o) => match o["streams"].clone() {
                Value::Object(o) => match o["flv"].clone() {
                    Value::Object(o) => match o["src"].clone() {
                        Value::String(s) => vec![s],
                        Value::Array(a) => a.iter().map(|v| {
                            match v {
                                Value::String(s) => s.clone(),
                                _ => String::new(),
                            }
                        }).collect(),
                        _ => return Err(format!("Failed to parse stdout as url in src\nstdout: {}\nstderr: {}", stos, you_get_stderr)),
                    }
                    _ => return Err(format!("Failed to parse stdout as url in flv\nstdout: {}\nstderr: {}", stos, you_get_stderr)),
                }
                _ => return Err(format!("Failed to parse stdout as url in streams\nstdout: {}\nstderr: {}", stos, you_get_stderr)),
            }
            _ => return Err(format!("Failed to parse stdout as url at top\nstdout: {}\nstderr: {}", stos, you_get_stderr)),
        }
        Err(e) => return Err(format!("Failed to deserialize stdout: {:?}", e)),
    };
    Ok(res.clone())
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