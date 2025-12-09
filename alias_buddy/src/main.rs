use dialoguer::{theme::ColorfulTheme, Select};
use std::env;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use regex::Regex;
use std::process::Command;

fn main() {
    let re = Regex::new(r"(?:function{0,1} |)([A-Za-z-_]{2,})(?:\(\))|(?:alias\W(.*)(?:=))").unwrap();
    let aliases = parse_aliases(re);
    if aliases.is_empty() {
        println!("No aliases are found, please ensure your aliases are present in ~/.bash_aliases");
        return;
    }


    let selections = &aliases;
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an alias to execute")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    let selection_value = &selections[selection];
    let child = Command::new("bash")
        .args(["-c", "-i", selection_value])
        // .arg(selection_value)
        .spawn()
        .expect("Failed to spawn a child process for the selected alias.");
    println!("Spawned new process for the selected alias, PID: {}", child.id());
}


fn get_home_dir() -> PathBuf {
    if let Ok(home_dir) = env::var("HOME") {
        return PathBuf::from(home_dir);
    }
    return PathBuf::from("~"); // This will likely fail
}

fn get_bash_aliases_path() -> PathBuf {
    let home_dir = get_home_dir();
    return home_dir.join( ".bash_aliases");
}


fn read_file(path: PathBuf) -> io::Result<String> {
    let mut file: File = File::open(path)?;
    let mut buffer: String = String::new();
    let _: usize = io::Read::read_to_string(&mut file, &mut buffer)?;
    Ok(buffer)
}

fn parse_aliases(re: Regex) -> Vec<String>{
    let bash_aliases_content: String = read_file(get_bash_aliases_path()).unwrap_or_default();
    // println!("{bash_aliases_content}");
    let mut results: Vec<String> = vec![];

    for [path] in re.captures_iter(&bash_aliases_content).map(|c| c.extract().1) {
        results.push(path.to_owned());
    }
    return results;
}