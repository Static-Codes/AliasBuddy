use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};
use std::collections::HashMap;
use std::env;
use std::fs::{File};
use std::io::{self};
use std::path::{Path, PathBuf};

fn main() { //-> Result<()> {
    // let bash_rc_path = get_bash_rc_path();
    let bash_aliases_path = get_bash_aliases_path();
    // let bash_rc_path_str = bash_rc_path.to_str().unwrap_or_default();
    let bash_aliases_str = bash_aliases_path.to_str().unwrap_or_default();

    // println!("{bash_rc_path_str}");
    // println!("{bash_aliases_str}");
    parse_aliases();
    // color_eyre::install()?;
    // let terminal = ratatui::init();
    // let result = run(terminal);
    // ratatui::restore();
    // result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
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

// fn get_bash_rc_path() -> PathBuf {
//     let home_dir: PathBuf = get_home_dir();
//     return home_dir.join(".bashrc");
// }

fn read_file(path: PathBuf) -> io::Result<Vec<u8>> {
    let mut file: File = File::open(path)?;
    let mut buffer: Vec<u8> = vec![];
    let _: usize = io::Read::read_to_end(&mut file, &mut buffer)?;
    Ok(buffer)
}

fn parse_aliases() {
    // let bash_rc_vector: Vec<u8> = read_file(get_bash_rc_path()).unwrap_or_default();
    let bash_aliases_vector: Vec<u8> = read_file(get_bash_aliases_path()).unwrap_or_default();

    // let bash_rc_contents: &str =  str::from_utf8(&bash_rc_vector).unwrap_or_default();
    let bash_aliases_content: &str = str::from_utf8(&bash_aliases_vector).unwrap_or_default();
    // let use_rc: bool = !bash_rc_contents.is_empty();
    // let use_aliases: bool = !bash_aliases_content.is_empty();
    // println!("{bash_rc_contents}");
    println!("{bash_aliases_content}");
}

fn create_aliases_hashmap() {

}

// Commands will go here
fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}