use async_std::net::ToSocketAddrs;
use async_std::sync::{Arc, Mutex};
use async_std::task;

use conhash::{ConsistentHash, Node};

use crate::errors::Result;
use crate::protocol;

#[derive(Debug, Clone)]
struct ClonableProtocol {
    connection: Arc<Mutex<protocol::Protocol>>,
}

impl Node for ClonableProtocol {
    fn name(&self) -> String {
        let protocol = self.clone();
        // necessary to implement expected sync trait
        let connection_info =  task::block_on(async{
            protocol.connection.lock().await.connection_info()
        });
        connection_info
    }
}

/// Struct that holds all connections and proxy commands to the right server based on the key
pub struct MemcachedClient {
    connections: ConsistentHash<ClonableProtocol>,
}

impl MemcachedClient {
    pub async fn new<A: ToSocketAddrs>(
        addrs: Vec<A>,
        connections_per_addr: u8,
    ) -> Result<MemcachedClient> {
        let mut ch = ConsistentHash::new();
        for addr in &addrs {
            for _ in 0..connections_per_addr {
                let protocol = protocol::Protocol::connect(addr).await?;
                ch.add(
                    &ClonableProtocol { connection: Arc::new(Mutex::new(protocol)) },
                    1,
                );
            }
        }
        Ok(MemcachedClient { connections: ch })
    }

    pub async fn set<K, V>(&self, key: K, value: V, time: u32) -> Result<()>
    where
        K: AsRef<[u8]>,
        V: protocol::ToMemcached,
    {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().await;
        protocol.set(key, value, time).await
    }

    pub async fn add<K, V>(&self, key: K, value: V, time: u32) -> Result<()>
    where
        K: AsRef<[u8]>,
        V: protocol::ToMemcached,
    {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().await;
        protocol.add(key, value, time).await
    }

    pub async fn replace<K, V>(&self, key: K, value: V, time: u32) -> Result<()>
    where
        K: AsRef<[u8]>,
        V: protocol::ToMemcached,
    {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().await;
        protocol.replace(key, value, time).await
    }

    pub async fn get<K, V>(&self, key: K) -> Result<V>
    where
        K: AsRef<[u8]>,
        V: protocol::FromMemcached,
    {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().await;
        protocol.get(key).await
    }

    pub async fn delete<K>(&self, key: K) -> Result<()>
    where
        K: AsRef<[u8]>,
    {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().await;
        protocol.delete(key).await
    }

    pub async fn increment<K>(
        &self,
        key: K,
        amount: u64,
        initial: u64,
        time: u32,
    ) -> Result<u64>
    where
        K: AsRef<[u8]>,
    {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().await;
        protocol.increment(key, amount, initial, time).await
    }

    pub async fn decrement<K>(
        &self,
        key: K,
        amount: u64,
        initial: u64,
        time: u32,
    ) -> Result<u64>
    where
        K: AsRef<[u8]>,
    {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().await;
        protocol.decrement(key, amount, initial, time).await
    }
}
