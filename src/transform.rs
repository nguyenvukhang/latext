const NAMESPACER: &str = "LatexTransformTMP";
pub fn set(cmd: &str, val: &str) -> String {
    format!("\\let\\{NAMESPACER}{cmd}\\{cmd}\\def\\{cmd}{val}",)
}

pub fn unset(cmd: &str) -> String {
    format!("\\let\\{cmd}\\{NAMESPACER}{cmd}")
}

pub fn transform(mut line: &str) -> Option<String> {
    line = line.trim_start();
    if !line.starts_with("%%") {
        return None;
    }
    line = &line[2..].trim_start();
    let words = line.split(' ').collect::<Vec<&str>>();
    let mut buffer = String::new();
    let mut s = words.as_slice();

    while !s.is_empty() {
        let current = s[0];
        if current == "set" && s.len() >= 3 {
            buffer.push_str(&set(s[1], s[2]));
            s = &s[3..];
            continue;
        }
        if current == "unset" {
            s = &s[1..];
            while !s.is_empty() {
                buffer.push_str(&unset(s[0]));
                s = &s[1..];
            }
            continue;
        }
        s = &s[1..];
    }
    Some(buffer)
}
