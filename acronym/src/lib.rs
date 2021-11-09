use regex::Regex;

pub fn abbreviate(phrase: &str) -> String {
    let re =
        Regex::new(r"((?:^|\s|-|_)(?P<letter>[[:alpha:]])|([a-z](?P<letter1>[A-Z])))").unwrap();
    let mut acr = String::new();
    for caps in re.captures_iter(phrase) {
        match caps.name("letter") {
            Some(x) => acr.push_str(x.as_str()),
            None => {}
        }
        match caps.name("letter1") {
            Some(x) => acr.push_str(x.as_str()),
            None => {}
        }
    }
    acr.to_uppercase()
}
