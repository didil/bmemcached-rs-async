# bmemcached-rs-async

Experimental port of [bmemcached-rs](https://github.com/jaysonsantos/bmemcached-rs) to async/await via async-std


# Usage
```rust
use async_std::sync::Arc;
use async_std::task;

use bmemcached::MemcachedClient;

fn main() {
    task::block_on(async{
        let client = Arc::new(MemcachedClient::new(vec!["127.0.0.1:11211"], 5).await.unwrap());

        // Traits examples
        let value = "value";
        client.set("string", value, 1000).await;
        let rv: String = client.get("string").await.unwrap();
        assert_eq!(rv, "value");

        client.set("integer", 10 as u8, 1000).await.unwrap();
        let rv: u8 = client.get("integer").await.unwrap();
        assert_eq!(rv, 10 as u8);

        // Tasks example
        let mut handles = vec![];
        for i in 0..4 {
            let client = client.clone();
            handles.push(task::spawn(async move {
                let data = format!("data_n{}", i);
                client.set(&data, &data, 100).await.unwrap();
                let val: String = client.get(&data).await.unwrap();
                client.delete(&data).await.unwrap();
                val
            }));
        }
        for (i, handle) in handles.into_iter().enumerate() {
            let result = handle.await;
            assert_eq!(result, format!("data_n{}", i));
        }
        });
}
```
