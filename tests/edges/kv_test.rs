use kv::Error;

#[test]
fn simple() -> Result<(), Error> {
    use kv::{Config, Error, Manager, ValueRef};
    // First step create a manager, this ensured that each LMDB environment will only be
    // accessed once per process
    let mut mgr = Manager::new();

    // Next configure a database
    let mut cfg = Config::default("/tmp/rust-kv");

    // Add a bucket named `test`
    cfg.bucket("test", None);

    // Get a Store handle
    let handle = mgr.open(cfg)?;

    // Get read-write access to the underlying store
    let store = handle.write()?;

    // A Bucket provides typed access to an LMDB database
    let bucket = store.bucket::<&str, &str>(Some("test"))?;

    {
        // Finally, a transaction is needed, they can be read-write or readonly, here we will use a
        // write transaction to add data
        let mut txn = store.write_txn()?;

        // To set a value
        let () = txn.set(&bucket, "testing", "abc123")?;

        // Make sure to commit the transaction. There is also an `abort` function to abandon
        // the transaction
        txn.commit()?;
    }

    {
        // This time a readonly transaction
        let txn = store.read_txn()?;

        // Getting a value is easy once everything is set up
        let val = txn.get(&bucket, "testing")?;
        println!("testing => {}", val);
    }

    Ok(())
}