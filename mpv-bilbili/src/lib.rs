use serde_json;
use serde_json::Value;
use std::process;
use std::process::Stdio;
use std::panic;

type Res<T> = Result<T, String>;

pub struct MediaInfo {
    pub urls: Vec<String>,
    pub title: String,
    pub referrer: String,
}

pub fn get_url(orig_url: &String) -> Res<(MediaInfo)> {
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
    let json_stdout = match serde_json::from_str(stos) {
        Ok(j) => match j {
            Value::Object(o) => o,
            _ => return Err(format!("Failed to parse stdout as url at top\nstdout: {}\nstderr: {}", stos, you_get_stderr)),
        },
        Err(e) => return Err(format!("Failed to deserialize stdout: {:?}", e)),
    };
    let urls =  panic::catch_unwind(|| {
        Ok(json_stdout["streams"]["flv"].clone())
    }).unwrap_or_else(|e| {
        return Err(format!("Failed to parse stdout as url\nerror: {:?}\nstdout: {}\nstderr: {}", e, stos, you_get_stderr));
    })?;
    let urls = panic::catch_unwind(|| {
        match urls["src"].clone() {
            Value::String(s) => Ok(vec![s]),
            Value::Array(a) => Ok(a.iter().map(|v| {
                match v {
                    Value::String(s) => s.clone(),
                    _ => String::new(),
                }
            }).collect()),
            _ => Err("No url is found".to_string())
        }
    }).unwrap_or_else(|e| {
        return Err(format!("Failed to parse stdout as url\nerror: {:?}\nstdout: {}\nstderr: {}", e, stos, you_get_stderr));
    })?;
    // referrer = json_output['extra']['referer']
    let referrer = match json_stdout["extra"].clone() {
        Value::Object(o) => match o["referer"].clone() {
            Value::String(s) => s,
            _ => String::new(),
        },
        _ => String::new(),
    };
    // title = json_output['title']
    let title = match json_stdout["title"].clone() {
        Value::String(s) => s,
        _ => String::new(),
    };
    Ok(MediaInfo { urls, referrer, title })
}
pub fn play_with_mpv(media_info: MediaInfo, sto: Stdio) -> Res<()> {
    let MediaInfo { urls, title, referrer } = media_info;
    let mut cmd = process::Command::new("mpv");
    for i in urls.iter() {
        cmd.arg(i);
    }
    cmd.arg(format!("--referrer={}", referrer))
        .arg(format!("--title={}", title))
        .arg("--merge-files")
        .arg("--no-ytdl")
        .stdout(sto)
        .spawn().expect("Failed to spawn child process")
        .wait_with_output().expect("Failed to run command");
    Ok(())
}