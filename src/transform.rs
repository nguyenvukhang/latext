const NAMESPACER: &str = "LatexTransformTMP";

fn set(cmd: &str, val: &str) -> String {
    format!("\\let\\{NAMESPACER}{cmd}\\{cmd}\\def\\{cmd}{val}",)
}

fn unset(cmd: &str) -> String {
    format!("\\let\\{cmd}\\{NAMESPACER}{cmd}")
}

pub fn transform(mut line: &str) -> Option<String> {
    line = line.trim_start();
    if !line.starts_with("%%") {
        return None;
    }
    let words = line[2..].trim_start().split(' ').collect::<Vec<&str>>();
    let (mut buffer, mut s) = (String::new(), words.as_slice());

    while !s.is_empty() {
        match s[0] {
            "set" if s.len() >= 3 => {
                buffer.push_str(&set(s[1], s[2]));
                s = &s[3..];
                continue;
            }
            "unset" => {
                (1..s.len()).for_each(|i| buffer.push_str(&unset(s[i])));
                return Some(buffer);
            }
            _ => s = &s[1..],
        }
    }
    Some(buffer)
}

pub fn replace(line: &str) -> String {
    use aho_corasick::AhoCorasick as AC;
    let from = &["×"];
    let to = &["\\times"];
    AC::new(from).unwrap().replace_all(line, to)
}
