use serde_json;
use serde_json::Value;
use std::process;
use std::process::Stdio;
use std::panic;

type Res<T> = Result<T, String>;

pub struct Url {
    pub videos: Vec<String>,
    pub audios: Vec<String>,
}
pub struct MediaInfo {
    pub url: Url,
    pub title: String,
    pub referrer: String,
}

pub fn parse_output(output: process::Output) -> Res<(String, String)> {
    let stdout = match String::from_utf8(output.stdout) {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to parse stdout: {:?}", e)),
    };
    let stderr = match String::from_utf8(output.stderr) {
        Ok(r) => r,
        Err(e) => String::from(format!("Failed to parse stderr: {:?}", e)),
    };
    Ok((stdout, stderr))
}
pub fn parse_url(json: &Value) -> Option<(Vec<String>, Vec<String>)> {
    match json["site"].as_str()? {
        "Bilibili" => {
            //json['streams'] is ordered with BTreeMap
            match json["streams"].clone() {
                Value::Object(o) => {
                    let displays = ["dash-flv", "dash-flv360", "dash-flv480", "dash-flv720", "flv", "flv360", "flv480", "flv720"];
                    let (dp, stream) = {
                        let mut res = None;
                        for i in displays.iter() {
                            match o.iter().find(|x| { x.0 == i }) {
                                Some(el) => {
                                    res = Some(el);
                                    break;
                                },
                                None => continue,
                            }
                        }
                        match res {
                            Some(el) => el,
                            None => o.iter().next()?
                        }
                    };
                    if dp.matches("dash").next().is_none() {
                        let video_url = stream["src"]
                            .as_array()?
                            .iter()
                            .map(|x| {
                                match x.as_str() {
                                    Some(s) => String::from(s),
                                    None => String::new(),
                                }
                            })
                            .collect();
                        Some((video_url, vec![]))
                    } else {
                        let dash_url = stream["src"].as_array()?;
                        let video_url = vec![String::from(dash_url[0][0].as_str()?)];
                        let audio_url = vec![String::from(dash_url[1][0].as_str()?)];
                        Some((video_url, audio_url))
                    }
                },
                _ => None,
            }
        },
        _ => None,
    }
}
pub fn get_url(orig_url: &String) -> Res<MediaInfo> {
    let (stdout, _stderr) = match process::Command::new("you-get")
        .arg(orig_url)
        .arg("--json")
        .output() {
        Ok(r) => {
            parse_output(r)?
        },
        Err(e) => return Err(format!("{:?}", e)),
    };
    let json_stdout = match serde_json::from_str(&*stdout) {
        Ok(j) => j,
        Err(e) => return Err(format!("Failed to deserialize stdout: {:?}", e)),
    };
    let (videos, audios) = match parse_url(&json_stdout) {
        Some(el) => el,
        None => return Err("Failed to parse stdout as url".to_string()),
    };
    // referrer = json_output['extra']['referer']
    let referrer = panic::catch_unwind(|| {
        match json_stdout["extra"]["referer"].clone() {
            Value::String(s) => s,
            _ => String::new(),
        }
    }).unwrap_or(String::new());
    // title = json_output['title']
    let title = match json_stdout["title"].clone() {
        Value::String(s) => s,
        _ => String::new(),
    };
    Ok(MediaInfo { url: Url { videos, audios }, referrer, title })
}
pub fn play_with_mpv(media_info: MediaInfo, stdio: Stdio) -> Res<()> {
    let MediaInfo { url: Url { videos, audios }, title, referrer } = media_info;
    let mut cmd = process::Command::new("mpv");
    for i in videos {
        cmd.arg(i);
    }
    for i in audios {
        cmd.arg(format!("--audio-file={}", i));
    }
    cmd.arg(format!("--referrer={}", referrer))
        .arg(format!("--title={}", title))
        .arg("--merge-files")
        .arg("--no-ytdl")
        .stdout(stdio)
        .output().expect("Failed to run command");
    Ok(())
}