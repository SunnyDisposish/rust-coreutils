#[macro_use]
extern crate clap;
use clap::App;


fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let filenames: Vec<&str> = matches.values_of("INPUT").unwrap().collect();
    println!("{:?}", filenames)
}
