use anyhow::Result;

use rdbms_rs_web_db_press::btree::{BTree, SearchMode};
use rdbms_rs_web_db_press::buffer::{BufferPool, BufferPoolManager};
use rdbms_rs_web_db_press::disk::{DiskManager, PageId};

fn main() -> Result<()> {
    let disk = DiskManager::open("test.btr")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let btree = BTree::new(PageId(0));
    let mut iter = btree.search(&mut bufmgr, SearchMode::Start)?;

    while let Some((key, value)) = iter.next(&mut bufmgr)? {
        println!("{:02x?} = {:02x?}", key, value);
    }
    Ok(())
}
