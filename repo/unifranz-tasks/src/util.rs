use regex::Regex;

pub fn clean_description(s: &str) -> String {
    let re_tags = Regex::new(r"<[^>]+>").unwrap();
    let mut t = re_tags.replace_all(s, " ").to_string();
    // remove common markdown symbols
    t = t.replace("**", "").replace("__", "").replace("`", "").replace("#", "");
    // collapse whitespace
    let re_ws = Regex::new(r"\s+").unwrap();
    t = re_ws.replace_all(&t, " ").trim().to_string();
    t
}

pub fn format_time_12h(dt: &chrono::DateTime<chrono::Local>) -> String {
    dt.format("%I:%M %p").to_string()
}

pub fn format_date(dt: &chrono::DateTime<chrono::Local>) -> String {
    dt.format("%d/%m/%y").to_string()
}
