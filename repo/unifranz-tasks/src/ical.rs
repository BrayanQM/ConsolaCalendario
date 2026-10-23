use crate::subjects;
use anyhow::Result;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone};
use regex::Regex;
use reqwest::blocking::Client;

#[derive(Clone, Debug)]
pub struct IcsTask {
    pub uid: String,
    pub title: String,
    pub subject: Option<String>,
    pub description: String,
    pub url: Option<String>,
    pub due: DateTime<Local>,
}

fn parse_ical_datetime(s: &str) -> Option<DateTime<Local>> {
    let v = s.trim();
    if v.ends_with('Z') {
        let core = &v[..v.len()-1];
        if core.len() == 15 {
            if let Ok(ndt) = NaiveDateTime::parse_from_str(core, "%Y%m%dT%H%M%S") {
                let utc = chrono::Utc.from_utc_datetime(&ndt);
                return Some(utc.with_timezone(&Local));
            }
        }
        if core.len() == 8 {
            if let Ok(nd) = NaiveDate::parse_from_str(core, "%Y%m%d") {
                if let Some(ndt) = nd.and_hms_opt(23, 59, 0) {
                    let utc = chrono::Utc.from_utc_datetime(&ndt);
                    return Some(utc.with_timezone(&Local));
                }
            }
        }
    }
    if v.len() == 15 && v.contains('T') {
        if let Ok(ndt) = NaiveDateTime::parse_from_str(v, "%Y%m%dT%H%M%S") {
            return Local.from_local_datetime(&ndt).earliest();
        }
    }
    if v.len() == 8 {
        if let Ok(nd) = NaiveDate::parse_from_str(v, "%Y%m%d") {
            if let Some(ndt) = nd.and_hms_opt(23, 59, 0) {
                return Local.from_local_datetime(&ndt).earliest();
            }
        }
    }
    let v2 = v.replace('T', " ");
    if let Ok(ndt) = NaiveDateTime::parse_from_str(&v2, "%Y%m%d %H%M%S") {
        return Local.from_local_datetime(&ndt).earliest();
    }
    None
}

fn extract_subject(title: &str, categories: &str) -> Option<String> {
    let re_code = Regex::new(r"([A-Z]{2,4})-(\d{3})").unwrap();
    if let Some(cap) = re_code.captures(categories) {
        let full = format!("{}-{}", &cap[1], &cap[2]);
        if let Some(t) = subjects::translate(&full) {
            return Some(t);
        }
    }
    if let Some(cap) = re_code.captures(title) {
        let full = format!("{}-{}", &cap[1], &cap[2]);
        if let Some(t) = subjects::translate(&full) {
            return Some(t);
        }
    }
    if title.to_lowercase().contains("asistencia") {
        if let Some(cap) = re_code.captures(categories) {
            let full = format!("{}-{}", &cap[1], &cap[2]);
            if let Some(t) = subjects::translate(&full) {
                return Some(format!("{} - Asistencia", t));
            }
        }
        return Some("Asistencia".to_string());
    }
    None
}

fn extract_url_from_desc(desc: &str) -> Option<String> {
    let re = Regex::new(r#"https?://[^\s<>"']+"#).ok()?;
    for cap in re.find_iter(desc) {
        let u = cap.as_str();
        if u.contains("view.php") || u.contains("mod/") {
            return Some(u.to_string());
        }
    }
    if let Some(cap) = re.find_iter(desc).next() {
        return Some(cap.as_str().to_string());
    }
    None
}

pub fn fetch_and_parse(url: &str) -> Result<Vec<IcsTask>> {
    let client = Client::builder().user_agent("folray/0.1").build()?;
    let body = client.get(url).send()?.text()?;
    let mut tasks = Vec::new();
    let mut cal = ical::IcalParser::new(body.as_bytes());
    while let Some(Ok(calendar)) = cal.next() {
        for event in calendar.events {
            let mut uid = String::new();
            let mut title = String::new();
            let mut description = String::new();
            let mut url_ev = None;
            let mut due: Option<DateTime<Local>> = None;
            let mut categories = String::new();
            for prop in event.properties {
                match prop.name.as_str() {
                    "UID" => if let Some(v) = &prop.value { uid = v.clone(); }
                    "SUMMARY" => if let Some(v) = &prop.value { title = v.clone(); }
                    "DESCRIPTION" => if let Some(v) = &prop.value { description = v.replace("\\n", "\n").replace("\\,", ","); }
                    "URL" => if let Some(v) = &prop.value { url_ev = Some(v.clone()); }
                    "CATEGORIES" => if let Some(v) = &prop.value { categories = v.clone(); }
                    "DUE" => {
                        if let Some(v) = &prop.value {
                            if let Some(dt) = parse_ical_datetime(v) { due = Some(dt); }
                        }
                    }
                    "DTEND" => {
                        if due.is_none() {
                            if let Some(v) = &prop.value {
                                if let Some(dt) = parse_ical_datetime(v) { due = Some(dt); }
                            }
                        }
                    }
                    "DTSTART" => {
                        if due.is_none() {
                            if let Some(v) = &prop.value {
                                if let Some(dt) = parse_ical_datetime(v) { due = Some(dt); }
                            }
                        }
                    }
                    _ => {}
                }
            }
            if due.is_some() && !title.is_empty() {
                let subject = extract_subject(&title, &categories);
                let clean_title = if title.to_lowercase().contains("asistencia") { "Asistencia".to_string() } else { title.clone() };
                if url_ev.is_none() {
                    url_ev = extract_url_from_desc(&description);
                }
                tasks.push(IcsTask { 
                    uid: if uid.is_empty() { format!("{}{}", clean_title, categories) } else { uid }, 
                    title: clean_title, 
                    subject, 
                    description: description.clone(), 
                    url: url_ev.clone(), 
                    due: due.unwrap() 
                });
            }
        }
    }
    Ok(tasks)
}
