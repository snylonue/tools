use serde_json;
use serde_json::Value;
use serde_json::map;
use std::process;
use std::process::Stdio;
use std::panic;

type Res<T> = Result<T, String>;

pub enum Website {
    Bilibili(bool),
}

pub struct MediaInfo {
    pub urls: Vec<String>,
    pub title: String,
    pub referrer: String,
    pub website: Website,
}

pub fn parse_output(output: process::Output) -> Res<(String, String)> {
    let stdout = match String::from_utf8(output.stdout) {
        Ok(r) => r.replace("\r", "").replace("\n", "").replace(" ", ""),
        Err(e) => return Err(format!("Failed to parse stdout: {:?}", e)),
    };
    let stderr = match String::from_utf8(output.stderr) {
        Ok(r) => r,
        Err(e) => String::from(format!("Failed to parse stderr: {:?}", e)),
    };
    Ok((stdout, stderr))
}
pub fn parse_url(json: &Value) -> Res<(map::Map<String, Value>, bool)> {
    panic::catch_unwind(|| {
        match json["streams"]["dash-flv"].clone() {
            Value::Object(o) => Ok((o, true)),
            _ => match json["streams"]["flv"].clone() {
                Value::Object(o) => Ok((o, false)),
                _ => match json["streams"]["dash-flv720"].clone() {
                    Value::Object(o) => Ok((o, true)),
                    _ => match json["streams"]["flv720"].clone() {
                        Value::Object(o) => Ok((o, false)),
                        _ => match json["streams"]["dash-flv480"].clone() {
                            Value::Object(o) => Ok((o, true)),
                            _ => match json["streams"]["flv480"].clone() {
                                Value::Object(o) => Ok((o, true)),
                                _ => match json["streams"]["dash-flv360"].clone() {
                                    Value::Object(o) => Ok((o, true)),
                                    _ => match json["streams"]["flv360"].clone() {
                                        Value::Object(o) => Ok((o, false)),
                                        _ => Err("No url is found".to_string()),
                                    },
                                },
                            },
                        },
                    },
                },
            },
        }
    }).unwrap_or_else(|e| {
        return Err(format!("Failed to parse json as url: {:?}", e));
    })
}
pub fn get_url(orig_url: &String) -> Res<MediaInfo> {
    let (stdout, stderr) = match process::Command::new("you-get").arg(orig_url).arg("--json").output() {
        Ok(r) => {
            parse_output(r)?
        },
        Err(e) => return Err(format!("{:?}", e)),
    };
    let json_stdout = match serde_json::from_str(&*stdout) {
        Ok(j) => j,
        Err(e) => return Err(format!("Failed to deserialize stdout: {:?}", e)),
    };
    let (obj_url, dash) = parse_url(&json_stdout)?;
    let website = Website::Bilibili(dash);
    let urls = panic::catch_unwind(|| {
        match obj_url["src"].clone() {
            Value::String(s) => Ok(vec![s]),
            Value::Array(a) => Ok(a.iter().map(|v| {
                match v {   
                    Value::String(s) => s.clone(),
                    Value::Array(a) => match a[0].clone() {
                        Value::String(s) => s,
                        _ => String::new(),
                    },
                    _ => String::new(),
                }
            }).collect()),
            _ => Err(format!(r#"No url is found, stdout: {}, stderr: {}"#, stdout, stderr))
        }
    }).unwrap_or_else(|e| {
        return Err(format!("Failed to parse stdout as url\nerror: {:?}\nstdout: {}\nstderr: {}", e, stdout, stderr));
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
    Ok(MediaInfo { urls, referrer, title, website })
}
pub fn play_with_mpv(media_info: MediaInfo, sto: Stdio) -> Res<()> {
    let MediaInfo { urls, title, referrer, website } = media_info;
    let mut cmd = process::Command::new("mpv");
    match website {
        Website::Bilibili(b) => {
            if b {
                cmd.arg(urls[0].clone())
                    .arg(format!("--audio-file={}", urls[1]));
            } else {
                for i in urls.iter() {
                    cmd.arg(i);
                }
            }
        },
    };
    cmd.arg(format!("--referrer={}", referrer))
        .arg(format!("--title={}", title))
        .arg("--merge-files")
        .arg("--no-ytdl")
        .stdout(sto)
        .spawn().expect("Failed to spawn child process")
        .wait().expect("Failed to run command");
    Ok(())
}