use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum KVS {
    #[structopt(name = "get")]
    Get { key: String },
    #[structopt(name = "set")]
    Set { key: String, value: String },
    #[structopt(name = "rm")]
    Rm { key: String },
}

fn main() {
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
