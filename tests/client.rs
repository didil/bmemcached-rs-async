use bmemcached_async::errors::{Error, ErrorKind};
use bmemcached_async::{MemcachedClient, Status};
use async_std::task;
use async_std::sync::Arc;

#[async_std::test]
async fn multiple_tasks() {
    let _ = env_logger::try_init();
    let mut handles = vec![];
    let client = Arc::new(MemcachedClient::new(vec!["127.0.0.1:11211"], 5).await.unwrap());
    for i in 0..4 {
        let client = client.clone();
        println!("Starting thread {}", i);
        handles.push(task::spawn(async move {
            println!("Started {}", i);
            let data = format!("data_n{}", i);
            client.set(&data, &data, 100).await.unwrap();
            let val: String = client.get(&data).await.unwrap();
            client.delete(&data).await.unwrap();
            println!("Finished {}", i);
            val
        }));
    }
    for (i, handle) in handles.into_iter().enumerate() {
        let result = handle.await;
        assert_eq!(result, format!("data_n{}", i));
    }
}

#[async_std::test]
async fn get_set_delete() {
    let _ = env_logger::try_init();
    let client = MemcachedClient::new(vec!["127.0.0.1:11211"], 1).await.unwrap();
    let key = "Hello Get, Set, Delete Client";
    let value = "World";
    client.set(key, value, 1000).await.unwrap();
    let rv: String = client.get(key).await.unwrap();
    assert_eq!(rv, value);
    client.delete(key).await.unwrap();
}

#[async_std::test]
async fn get_set_u8() {
    let _ = env_logger::try_init();
    let client = MemcachedClient::new(vec!["127.0.0.1:11211"], 1).await.unwrap();
    let key = "Hello u8";
    let value = 1 as u8;
    client.set(key, value, 1000).await.unwrap();

    let rv: u8 = client.get(key).await.unwrap();
    assert_eq!(rv, value);
    client.delete(key).await.unwrap();
}

#[async_std::test]
async fn get_set_u16() {
    let _ = env_logger::try_init();
    let client = MemcachedClient::new(vec!["127.0.0.1:11211"], 1).await.unwrap();
    let key = "Hello u16";
    let value = 1 as u16;
    client.set(key, value, 1000).await.unwrap();

    let rv: u16 = client.get(key).await.unwrap();
    assert_eq!(rv, value);
    client.delete(key).await.unwrap();
}

#[async_std::test]
async fn get_set_u32() {
    let _ = env_logger::try_init();
    let client = MemcachedClient::new(vec!["127.0.0.1:11211"], 1).await.unwrap();
    let key = "Hello u32";
    let value = 1 as u32;
    client.set(key, value, 1000).await.unwrap();

    let rv: u32 = client.get(key).await.unwrap();
    assert_eq!(rv, value);
    client.delete(key).await.unwrap();
}

#[async_std::test]
async fn get_set_u64() {
    let _ = env_logger::try_init();
    let client = MemcachedClient::new(vec!["127.0.0.1:11211"], 1).await.unwrap();
    let key = "Hello u64";
    let value = 1 as u64;
    client.set(key, value, 1000).await.unwrap();

    let rv: u64 = client.get(key).await.unwrap();
    assert_eq!(rv, value);
    client.delete(key).await.unwrap();
}

#[async_std::test]
async fn add() {
    let _ = env_logger::try_init();
    let client = MemcachedClient::new(vec!["127.0.0.1:11211"], 1).await.unwrap();
    let key = "Hello Add Client";
    let value = "World";
    client.add(key, value, 1000).await.unwrap();
    let rv: String = client.get(key).await.unwrap();
    assert_eq!(rv, value);
    match client.add(key, value, 1000).await {
        Err(Error(ErrorKind::Status(Status::KeyExists), _)) => (),
        e => panic!("Wrong status returned {:?}", e),
    }
    client.delete(key).await.unwrap();
}

#[async_std::test]
async fn replace() {
    let _ = env_logger::try_init();
    let client = MemcachedClient::new(vec!["127.0.0.1:11211"], 1).await.unwrap();
    let key = "Hello Replace Client";
    let value = "World";
    client.add(key, value, 1000).await.unwrap();

    let rv: String = client.get(key).await.unwrap();
    assert_eq!(rv, value);

    client.replace(key, "New value", 100).await.unwrap();
    let rv: String = client.get(key).await.unwrap();
    assert_eq!(rv, "New value");
    client.delete(key).await.unwrap();
}

#[async_std::test]
async fn increment() {
    let _ = env_logger::try_init();
    let client = MemcachedClient::new(vec!["127.0.0.1:11211"], 1).await.unwrap();
    let key = "Hello Increment Client";
    assert_eq!(client.increment(key, 1, 0, 1000).await.unwrap(), 0);
    assert_eq!(client.increment(key, 1, 1, 1000).await.unwrap(), 1);
    client.delete(key).await.unwrap();
}

#[async_std::test]
async fn decrement() {
    let _ = env_logger::try_init();
    let client = MemcachedClient::new(vec!["127.0.0.1:11211"], 1).await.unwrap();
    let key = "Hello Decrement Client";
    assert_eq!(client.decrement(key, 1, 10, 1000).await.unwrap(), 10);
    assert_eq!(client.decrement(key, 1, 1, 1000).await.unwrap(), 9);
    client.delete(key).await.unwrap();
}
