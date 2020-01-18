use serde_json;
use serde_json::Value;
use failure::err_msg;
use failure::Error;
use std::process;
use std::process::Stdio;
use std::panic;

type Res<T> = Result<T, Error>;

pub struct Url {
    pub videos: Vec<String>,
    pub audios: Vec<String>,
}
pub struct MediaInfo {
    pub url: Url,
    pub title: Option<String>,
    pub referrer: Option<String>,
}

impl Url {
    pub fn new(videos: Vec<String>, audios: Vec<String>) -> Self {
        Url { videos, audios }
    }
    pub fn from_value(value: &Value, site: &str, displays: &[&str]) -> Option<Self> {
        match site {
            "Bilibili" => {
                //json['streams'] is ordered with BTreeMap
                let (dp, stream) = search_displays(&value["streams"], &displays)?;
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
                    Some(Self::new(video_url, vec![]))
                } else {
                    let dash_url = stream["src"].as_array()?;
                    let video_url = vec![String::from(dash_url[0][0].as_str()?)];
                    let audio_url = vec![String::from(dash_url[1][0].as_str()?)];
                    Some(Self::new(video_url, audio_url))
                }
            },
            _ => None,
        }
    }
}
impl MediaInfo {
    pub fn play(&self, stdio: Stdio) -> Res<()> {
        let Url { videos, audios } = &self.url;
        let mut cmd = process::Command::new("mpv");
        for i in videos {
            cmd.arg(i);
        }
        for i in audios {
            cmd.arg(format!("--audio-file={}", i));
        }
        if let Some(referrer) = &self.referrer {
            cmd.arg(format!("--referrer={}", referrer));
        }
        if let Some(title) = &self.title {
            cmd.arg(format!("--title={}", title));
        }
        cmd.arg("--merge-files")
            .arg("--no-ytdl")
            .stdout(stdio)
            .output()?;
        Ok(())
    }
}

fn search_displays<'a>(object: &'a Value, displays: &[&str]) -> Option<(&'a String, &'a Value)> {
    let object = object.as_object()?;
    let mut res = None;
    for i in displays.iter() {
        match object.iter().find(|x| { x.0 == i }) {
            Some(el) => {
                res = Some(el);
                break;
            },
            None => continue,
        }
    }
    match res {
        Some(el) => Some(el),
        None => Some(object.iter().next()?)
    }
}
#[inline]
pub fn parse_output(output: process::Output) -> Res<(String, String)> {
    Ok((String::from_utf8(output.stdout)?, String::from_utf8(output.stderr)?))
}
pub fn parse_url(json: &Value) -> Option<Url> {
    match json["site"].as_str()? {
        "Bilibili" => {
            let displays = ["dash-flv", "dash-flv360", "dash-flv480", "dash-flv720", "flv", "flv360", "flv480", "flv720"];
            Url::from_value(json, "Bilibili", &displays)
        }
        _ => Url::from_value(json, "", &[]),
    }
}
pub fn get_url(orig_url: &String) -> Res<MediaInfo> {
    let (stdout, _) = parse_output(process::Command::new("you-get")
        .arg(orig_url)
        .arg("--json")
        .output()?)?;
    let json_stdout = match serde_json::from_str(&*stdout) {
            Ok(j) => j,
            Err(e) => return Err(err_msg(format!("Failed to deserialize stdout: {}", e))),
    };
    let url = match parse_url(&json_stdout) {
        Some(el) => el,
        None => return Err(err_msg("Failed to parse stdout as url")),
    };
    // referrer = json_output['extra']['referer']
    let referrer = panic::catch_unwind(|| {
        match json_stdout["extra"]["referer"] {
            Value::String(ref s) => Some(s.clone()),
            _ => Some(json_stdout["url"].as_str().unwrap().to_string()),
        }
    }).unwrap_or(None);
    // title = json_output['title']
    let title = match json_stdout["title"] {
        Value::String(ref s) => Some(s.clone()),
        _ => None,
    };
    Ok(MediaInfo { url, referrer, title })
}