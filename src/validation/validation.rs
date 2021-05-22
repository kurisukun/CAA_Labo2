
use regex::Regex;



pub fn check_cmd_syntax(command: &str) -> bool {
    let re: Regex = Regex::new(r"^([A-Za-z]+)$|^(\d+)$").unwrap();

    re.is_match(&command)
}