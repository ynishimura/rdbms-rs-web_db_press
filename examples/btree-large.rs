use anyhow::Result;
use md5::{Digest, Md5};

use rdbms_rs_web_db_press::btree::BTree;
use rdbms_rs_web_db_press::buffer::{BufferPool, BufferPoolManager};
use rdbms_rs_web_db_press::disk::DiskManager;

const NUM_PAIRS: u32 = 1_000_000;

fn main() -> Result<()> {
    let disk = DiskManager::open("large.btr")?;
    let pool = BufferPool::new(100);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let btree = BTree::create(&mut bufmgr)?;
    for i in 1u32..=NUM_PAIRS {
        let pkey = i.to_be_bytes();
        let md5 = Md5::digest(&pkey);
        btree.insert(&mut bufmgr, &md5[..], &pkey[..])?;
    }
    bufmgr.flush()?;

    Ok(())
}
