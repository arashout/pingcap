use clap::{App, Arg, SubCommand};

use structopt::StructOpt;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[derive(StructOpt, Debug)]
#[structopt(name = "kvs", about = "key value store")]
enum KVS {
    #[structopt(name = "get")]
    Get { key: String },
    #[structopt(name = "set")]
    Set { key: String, value: String },
    #[structopt(name = "rm")]
    Rm { key: String },
}

fn main() {
    let matches = App::new(APP_NAME)
        .version(APP_VERSION)
        .author(APP_AUTHORS)
        .subcommand(
            SubCommand::with_name("get")
                .arg(Arg::with_name("key").required(true).takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("set")
                .arg(Arg::with_name("key").required(true).takes_value(true))
                .arg(Arg::with_name("value").required(true).takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("rm").arg(Arg::with_name("key").required(true).takes_value(true)),
        )
        .get_matches();

    let config = KVS::from_args();
    println!("{:#?}", config);

    match config {
        KVS::Get { key } => {
            let value = key + "random";
            unimplemented!("unimplemented")
        }
        KVS::Set { key, value } => {
            println!("{} {} {}", "set", key, value);
            unimplemented!("unimplemented")
        }
        KVS::Rm { key } => {
            println!("{} {}", "rm", key);
            unimplemented!("unimplemented")
        }
    }
}
