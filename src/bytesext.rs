use async_std::io::{BufReader, BufWriter, Read, Result, Write};
use async_std::prelude::*;
use async_trait::async_trait;
use byteorder::ByteOrder;

#[async_trait]
pub trait AsyncReadBytesExt: Read {
    async fn read_u8(&mut self) -> Result<u8>
    where
        Self: Unpin,
    {
        let mut buf = [0; 1];
        self.read_exact(&mut buf).await?;
        Ok(buf[0])
    }

    async fn read_u16<T: ByteOrder>(&mut self) -> Result<u16>
    where
        Self: Unpin,
    {
        let mut buf = [0; 2];
        self.read_exact(&mut buf).await?;
        Ok(T::read_u16(&buf))
    }

    async fn read_u32<T: ByteOrder>(&mut self) -> Result<u32>
    where
        Self: Unpin,
    {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).await?;
        Ok(T::read_u32(&buf))
    }

    async fn read_u64<T: ByteOrder>(&mut self) -> Result<u64>
    where
        Self: Unpin,
    {
        let mut buf = [0; 8];
        self.read_exact(&mut buf).await?;
        Ok(T::read_u64(&buf))
    }
}

impl<R: Read> AsyncReadBytesExt for BufReader<R> {}

#[async_trait]
pub trait AsyncWriteBytesExt: Write {
    async fn write_u8(&mut self, n: u8) -> Result<()>
    where
        Self: Unpin,
    {
        self.write_all(&[n]).await
    }

    async fn write_u16<T: ByteOrder>(&mut self, n: u16) -> Result<()>
    where
        Self: Unpin,
    {
        let mut buf = [0; 2];
        T::write_u16(&mut buf, n);
        self.write_all(&buf).await
    }

    async fn write_u32<T: ByteOrder>(&mut self, n: u32) -> Result<()>
    where
        Self: Unpin,
    {
        let mut buf = [0; 4];
        T::write_u32(&mut buf, n);
        self.write_all(&buf).await
    }

    async fn write_u64<T: ByteOrder>(&mut self, n: u64) -> Result<()>
    where
        Self: Unpin,
    {
        let mut buf = [0; 8];
        T::write_u64(&mut buf, n);
        self.write_all(&buf).await
    }



   
}

impl<R: Write> AsyncWriteBytesExt for BufWriter<R> {}
