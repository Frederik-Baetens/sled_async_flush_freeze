use itertools::Itertools;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use tokio::runtime::Runtime;
use tokio::task::JoinSet;

fn main() {
    let rt = Runtime::new().unwrap();
    let db = sled::Config::default().temporary(true).open().unwrap();
    let prefix_chunks = thread_rng().sample_iter(Alphanumeric).take(90).chunks(9);
    let prefix_iter = prefix_chunks
        .into_iter()
        .map(|c| String::from_utf8_lossy(&c.collect::<Vec<u8>>()).to_string());
    let mut futs = JoinSet::new();

    for prefix in prefix_iter {
        let db = db.clone();
        futs.spawn_on(
            async move {
                db.insert(prefix, "elloa").unwrap();
                println!("pre-flush");
                db.flush_async().await.unwrap();
                println!("post-flush");
            },
            rt.handle(),
        );
    }
    rt.block_on(async move { while let Some(()) = futs.join_one().await.unwrap() {} });
    println!("benchmark finished");
}
