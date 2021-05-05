use anyhow::Result;

use rdbms_rs_web_db_press::btree::{BTree, SearchMode};
use rdbms_rs_web_db_press::buffer::{BufferPool, BufferPoolManager};
use rdbms_rs_web_db_press::disk::{DiskManager, PageId};

fn main() -> Result<()> {
    let disk = DiskManager::open("test.btr")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    // B+Treeを開く
    let btree = BTree::new(PageId(0));

    // 検索する
    let mut iter = btree.search(&mut bufmgr, SearchMode::Key(b"Hyogo".to_vec()))?;

    // 値を一つ取り出す
    let (key, value) = iter.next(&mut bufmgr)?.unwrap();
    println!("{:02x?} = {:02x?}", key, value);
    Ok(())
}
