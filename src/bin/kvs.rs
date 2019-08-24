use structopt::StructOpt;

extern crate kvs;

// TODO: Move this enum to the lib? and re-use it here
#[derive(StructOpt, Debug)]
enum KVS {
    #[structopt(name = "get")]
    Get { key: String },
    #[structopt(name = "set")]
    Set { key: String, value: String },
    #[structopt(name = "rm")]
    Rm { key: String },
}

fn main() -> kvs::CustomResult<()> {
    let config = KVS::from_args();
    println!("{:#?}", config);
    let mut kv_store = kvs::KvStore::open()?;
    match config {
        KVS::Get { key } => {
            unimplemented!("unimplemented")
        }
        KVS::Set { key, value } => {
            kv_store.set(key, value);
        }
        KVS::Rm { key } => {
            unimplemented!("unimplemented")
        }
    }
    Ok(())
}
