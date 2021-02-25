use clap::{App, Arg, SubCommand};
use std::path::Path;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

mod recieve;
fn main() {
    let recieve_command = SubCommand::with_name("recieve")
        .about("allows the mode to be set to recieve")
        .version(VERSION)
        .author(AUTHORS)
        .arg(
            Arg::with_name("auto")
                .short("a")
                .help("Automatically accept and save all drops"),
        );

    let drop_command = SubCommand::with_name("drop")
        .about("allows the mode to be set to drop")
        .version(VERSION)
        .author(AUTHORS)
        .arg(
            Arg::with_name("file")
                .short("f")
                .help("Select the file to send.")
                .takes_value(true)
                .required(true),
        );

    let matches = App::new("Rust Drop")
        .version(VERSION)
        .author(AUTHORS)
        .about("An attempt at a rusty AirDrop like LAN file share app!")
        .arg(
            Arg::with_name("version")
                .short("v")
                .help("Display the version of Rust Drop"),
        )
        .subcommand(drop_command)
        .subcommand(recieve_command)
        .get_matches();
    let rdrop = RDrop {};
    // Print Version info if requested
    if matches.is_present("v") {
        println!("Version: {:?}", VERSION);
    } else {
    }

    // Switch to match statement
    if let Some(matches) = matches.subcommand_matches("drop") {
        if matches.is_present("file") {
            rdrop.drop(matches.value_of("file").unwrap());
        } else {
            println!("Printing normally...");
        }
    }
    if let Some(_matches) = matches.subcommand_matches("recieve") {
        rdrop.recieve();
    }
}

#[derive(Copy, Clone)]
struct RDrop {}
impl RDrop {
    pub fn drop(self, file_path: &str) {
        if Path::new(&file_path).exists() {
            //Validate the file exists
            println!("File found! Dropping {:?}", &file_path);
        } else {
            println!("File {:?} not found", &file_path);
        }
    }
    fn recieve(self) {
        // NOTE(Able): Yeah good luck buddy
        recieve::register(); // loops (like it should)
    }
}
