pub fn set(cmd: &str, val: &str) -> String {
    format!("\\let\\TMP{cmd}\\{cmd}\\def\\{cmd}{val}",)
}

pub fn unset(cmd: &str) -> String {
    format!("\\let\\{cmd}\\TMP{cmd}")
}

pub fn transform(raw_line: &str) -> Option<String> {
    let mut line = raw_line;
    line = line.trim_start();
    if !line.starts_with("%%") {
        return None;
    }
    line = &line[2..].trim_start();
    let words = line.split(' ').collect::<Vec<&str>>();
    let mut res = String::new();
    let mut slice = words.as_slice();

    while !slice.is_empty() {
        let current = slice[0];
        if current == "set" && slice.len() >= 3 {
            res.push_str(&set(slice[1], slice[2]));
            slice = &slice[3..];
            continue;
        }
        if current == "unset" {
            slice = &slice[1..];
            while !slice.is_empty() {
                res.push_str(&unset(slice[0]));
                slice = &slice[1..];
            }
            continue;
        }
        slice = &slice[1..];
    }
    Some(res)
}
