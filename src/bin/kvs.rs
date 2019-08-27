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

fn main() -> kvs::Result<()> {
    let config = KVS::from_args();
    let path = std::path::Path::new(kvs::LOG_FILE);
    let mut kv_store = kvs::KvStore::open(path)?;
    match config {
        KVS::Get { key } => {
            if let Ok(vr) = kv_store.get(key) {
                if let Some(v) = vr {
                    println!("{}", v);
                }
            }
        },
        KVS::Set { key, value } => {
            kv_store.set(key, value).expect("Somehow failed set");
        }
        KVS::Rm { key } => {
            kv_store.remove(key).expect("Somehow failed rm");
        },
    }
    Ok(())
}
