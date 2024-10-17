use reqwest::Error;
use std::cmp::Reverse;
use std::collections::HashMap;

#[macro_export]
macro_rules! prod_or_fast {
    ($prod:expr, $test:expr) => {
        if cfg!(feature = "fast-runtime") {
            $test
        } else {
            $prod
        }
    };
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    Ok(())
}

#[subxt::subxt(runtime_metadata_path = "../metadata.scale")] // relace path accordingly 
pub mod voice_ban {}


#[cfg(test)]
mod tests {
    use super::*;
    use rust_ipfs::{
        unixfs::{AddOpt, UnixfsStatus},
        Block, Ipfs, IpfsPath, MessageId, Multiaddr, PeerId, PublicKey, SubscriptionStream,
    };
    // #[test]
    // fn test_div() {
    //     assert_eq!(divide(4, 2), Ok(2));
    // }
    // #[test]
    // fn test_add() {
    //     assert_eq!(add(1, 2), 3);
    // }
    // #[tokio::test]
    // async fn test1() {
    //     assert_eq!(4, 4);
    // }

    async fn setup_ipfs() -> rust_ipfs::Ipfs {
        let ipfs = rust_ipfs::UninitializedIpfsNoop::new().with_default();
        let y = ipfs.with_mdns().start().await.unwrap();
        y.default_bootstrap().await.unwrap();
        y
    }
    async fn ipfs_get(ipfs: &Ipfs, path: IpfsPath) -> Result<Vec<u8>, rust_ipfs::Error> {
        let path_copy = path.clone();
        let stream_result = ipfs.cat_unixfs(path).await;
        let default_error_msg = "Unable to cat file: ".to_string() + &path_copy.to_string();

        let _default_error = rust_ipfs::Error::msg(default_error_msg);

        match stream_result {
            Ok(stream) => {
                let mut data = Vec::<u8>::new();
                data.append(&mut stream.to_vec());

                Ok(data)
            }
            Err(e) => Err(rust_ipfs::Error::msg(format!(
                "Unable to traverse file with error message: {:?}",
                e
            ))),
        }
    }
    #[tokio::test]
    async fn test_ipfs_get() {
        let node = spawn_node("--alice", "random-tmp-path",9945, 30337);
        let second_ipfs = setup_ipfs().await;
        //let cid = cid produced by the node; you can get it using subxt like this:
        let client = OnlineClient::<PolkadotConfig>::from_url(ws).await;
        let query = voice_ban::storage().ipfs_address().ipfs_cids();
        let storage = client.storage().at_latest().await.unwrap();
        let cids = storage.fetch(&query).await.unwrap().unwrap();
        let cid = cids[0].clone()
        //let cids = storage.fetch(&query).await.unwrap().unwrap();
        let retrieved_data = ipfs_get(&second_ipfs, cid).await.unwrap();

        dbg!(retrieved_data);
        //assert_eq!(data, retrieved_data);
    }

    fn spawn(ar: &str, path: &str, port: u16, porty: u16) -> Child {
    let mut cmd =
        Command::new("/Users/kofiotuo/CLionProjects/VoiceBanBackend/target/release/vban-node"); // replace with the node path
        .arg(format!("--port={}", porty))
        .arg(format!("--rpc-port={}", port))
        .arg(format!("--base-path=/tmp/2uas13{}", path))
        .arg(ar);
    cmd.spawn().unwrap()
}

}
