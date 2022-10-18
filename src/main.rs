use dirs::config_dir;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    // TODO: Allow user to specify file location
    // TODO: Display better error message if file isn't found

    let mut path = config_dir().expect("No config dir found");
    path.push("i3");
    path.push("config");
    //dbg!(path);

    let cfg = std::fs::read_to_string(path.as_os_str()).expect("i3 config not found.");

    let re = Regex::new(r#"(exec (-.*? )?"?|\$\()(?P<name>.*?)( |"|$)"#).unwrap();

    let mut execs = HashSet::new();
    cfg.split("\n").for_each(|line| {
        for caps in re.captures_iter(line) {
            execs.insert(caps["name"].to_string());
        }
    });

    println!("{}", execs.into_iter().collect::<Vec<String>>().join("\n"));
}
