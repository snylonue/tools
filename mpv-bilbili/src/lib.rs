use serde_json;
use serde_json::Value;
use failure::err_msg;
use failure::Error;
use std::process;
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
}
impl MediaInfo {
    pub fn play(&self, vo: bool, ao: bool) -> Res<()> {
        let Url { videos, audios } = &self.url;
        let mut cmd = process::Command::new("mpv");
        if vo && videos.len() > 0 {
            for i in videos {
                cmd.arg(i);
            }
            if ao {
                for i in audios {
                    cmd.arg(format!("--audio-file={}", i));
                }
            }
        } else if ao && audios.len() > 0 {
            for i in audios {
                cmd.arg(i);
            }
        } else {
            return Err(err_msg("No urls to play"))
        }
        if let Some(referrer) = &self.referrer {
            cmd.arg(format!("--referrer={}", referrer));
        }
        if let Some(title) = &self.title {
            cmd.arg(format!("--title={}", title));
        }
        cmd.arg("--merge-files")
            .arg("--no-ytdl")
            .output()?;
        Ok(())
    }
}

fn search_displays<'a>(object: &'a Value, displays: &[&str]) -> Option<(&'a String, &'a Value)> {
    let object = object.as_object()?;
    let mut res = None;
    for i in displays.iter() {
        match object.iter().find(|(x, _)| { x == i }) {
            Some(el) => {
                res = Some(el);
                break;
            },
            None => continue,
        }
    }
    match res {
        Some(_) => res,
        None => Some(object.iter().next()?)
    }
}
#[inline]
pub fn parse_output(output: process::Output) -> Res<(String, String)> {
    Ok((String::from_utf8(output.stdout)?, String::from_utf8(output.stderr)?))
}
fn parse_url(value: &Value) -> Option<Url> {
    match value["site"].as_str()? {
        "Bilibili" => {
            let displays = ["dash-flv", "dash-flv360", "dash-flv480", "dash-flv720", "flv", "flv360", "flv480", "flv720"];
            //json['streams'] is ordered with BTreeMap
            let (dp, stream) = search_displays(&value["streams"], &displays)?;
            if dp.matches("dash").next().is_none() {
                let video_url = stream["src"]
                    .as_array()?
                    .iter()
                    .map(|x| { String::from(x.as_str().unwrap_or("")) })
                    .collect();
                Some(Url::new(video_url, vec![]))
            } else {
                let dash_url = stream["src"].as_array()?;
                let video_url = vec![String::from(dash_url[0][0].as_str()?)];
                let audio_url = vec![String::from(dash_url[1][0].as_str()?)];
                Some(Url::new(video_url, audio_url))
            }
        },
        "爱奇艺 (Iqiyi)" => {
            let displays = ["TD_H265", "TD", "HD_H265", "HD", "SD", "LD"];
            let (_, stream) = search_displays(&value["streams"], &displays)?;
            let video_url = stream["src"]
                .as_array()?
                .iter()
                .map(|x| { String::from(x.as_str().unwrap_or("")) })
                .collect();
            Some(Url::new(video_url, vec![]))
        },
        _ => None,
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
    let title = match json_stdout["title"].as_str() {
        Some(s) => Some(s.to_string()),
        _ => None,
    };
    Ok(MediaInfo { url, referrer, title })
}