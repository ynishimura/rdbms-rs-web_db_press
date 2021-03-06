use anyhow::Result;

use rdbms_rs_web_db_press::buffer::{BufferPool, BufferPoolManager};
use rdbms_rs_web_db_press::disk::{DiskManager, PageId};
use rdbms_rs_web_db_press::query::{Filter, PlanNode, SeqScan, TupleSearchMode};
use rdbms_rs_web_db_press::tuple;

fn main() -> Result<()> {
    let disk = DiskManager::open("simple.rly")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let plan = Filter {
        cond: &|record| record[1].as_slice() < b"Dave",
        inner_plan: &SeqScan {
            table_meta_page_id: PageId(0),
            search_mode: TupleSearchMode::Key(&[b"w"]),
            while_cond: &|pkey| pkey[0].as_slice() < b"z",
        },
    };
    let mut exec = plan.start(&mut bufmgr)?;

    while let Some(record) = exec.next(&mut bufmgr)? {
        println!("{:?}", tuple::Pretty(&record));
    }
    Ok(())
}
