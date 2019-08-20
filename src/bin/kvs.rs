use clap::{Arg, App, SubCommand};

const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
fn main() {
    let matches = App::new(APP_NAME)
        .version(APP_VERSION)
        .author(APP_AUTHORS)
        .subcommand(SubCommand::with_name("get")
            .arg(Arg::with_name("key")
            .required(true)
            .takes_value(true)))
        .subcommand(SubCommand::with_name("set")
            .arg(Arg::with_name("key")
            .required(true)
            .takes_value(true))
            .arg(Arg::with_name("value")
            .required(true)
            .takes_value(true))
        )
        .subcommand(SubCommand::with_name("rm")
            .arg(Arg::with_name("key")
            .required(true)
            .takes_value(true)))
        .get_matches();
    
    match matches.subcommand_name() {
        Some("get") => {
            let value = matches.subcommand_matches("get").unwrap().value_of("key").unwrap();
            println!("{} {}", "get", value);
            unimplemented!("unimplemented")
        },
        Some("set") => {
            let sm = matches.subcommand_matches("set").unwrap();
            let key = sm.value_of("key").unwrap();
            let value = sm.value_of("value").unwrap();
            println!("{} {} {}", "set", key, value);
            unimplemented!("unimplemented")
        }
        Some("rm") => {
            let sm = matches.subcommand_matches("rm").unwrap();
            let key = sm.value_of("key").unwrap();
            println!("{} {}", "rm", key);
            unimplemented!("unimplemented")
        }
        None        => panic!(),
        _           => println!("Some other subcommand was used"),
    }
}