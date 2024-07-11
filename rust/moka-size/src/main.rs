use moka::sync::Cache;

fn main() {
    let cache = Cache::new(100);

    for i in 0..100u64 {
        cache.insert(i, i + 1);
    }

    cache.run_pending_tasks();
    println!("{}", cache.entry_count());

    cache.invalidate(&0);
    cache.insert(101, 102);
    cache.run_pending_tasks();
    println!("{}", cache.entry_count());

    println!("{:?}", cache.get(&101));
}
