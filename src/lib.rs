/*!
This is a binary memcached protocol implemented only in rust with support of traits to send
and receive `T` and consistent hashing to select connections from a pool to distribute data.
# Example
```rust!
use async_std::sync::Arc;
use async_std::task;

use bmemcached::MemcachedClient;

#[async_std::main]
async fn main() {
    // Use arc for threading support
    //let client = Arc::new(MemcachedClient::new(vec!["127.0.0.1:11211"], 5).await.unwrap());

    // Traits examples
    //let value = "value";
    //client.set("string", value, 1000);
    //let rv: String = client.get("string").await.unwrap();
    //assert_eq!(rv, "value");

    //client.set("integer", 10 as u8, 1000);
    //let rv: u8 = client.get("integer").await.unwrap();
    //assert_eq!(rv, 10 as u8);

    // Threads example
    //let mut handles = vec![];
    //for i in 0..4 {
    //    let client = client.clone();
    //    handles.push(task::spawn(async move {
    //        let data = format!("data_n{}", i);
    //        client.set(&data, &data, 100).await.unwrap();
    //        let val: String = client.get(&data).await.unwrap();
    //        client.delete(&data).await.unwrap();
    //        val
    //    }));
    //}
    //for (i, handle) in handles.into_iter().enumerate() {
    //    let result = handle.await;
    //    assert_eq!(result, format!("data_n{}", i));
    //}
}
```
*/
#![forbid(unsafe_code)]
#[macro_use]
extern crate bitflags;
extern crate byteorder;
extern crate conhash;
#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate log;
extern crate num;

mod client;
pub mod constants;
pub mod errors;
pub mod protocol;
pub mod bytesext;

pub use protocol::{FromMemcached, Status, ToMemcached};
#[macro_use]
extern crate error_chain;

pub use client::MemcachedClient;
pub use constants::StoredType;
