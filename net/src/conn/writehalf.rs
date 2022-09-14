mod writer;
use crate::{encoding::EncodedData, packing::Compression};
use futures_lite::AsyncWrite;
use std::io;
use writer::Writer;

pub struct WriteHalf<W> {
    writer: Writer<W>,
    compression: Option<Compression>,
    compress_buf: Vec<u8>,
}

const DEFAULT_COMPRESS_BUF_CAPACITY: usize = 4096;

impl<W> WriteHalf<W> {
    pub fn new(inner: W) -> WriteHalf<W> {
        WriteHalf {
            writer: Writer::new(inner),
            compression: None,
            compress_buf: Vec::with_capacity(DEFAULT_COMPRESS_BUF_CAPACITY),
        }
    }
    pub fn new_with_capacity(inner: W, capacity: u32) -> WriteHalf<W> {
        WriteHalf {
            writer: Writer::new(inner),
            compression: None,
            compress_buf: Vec::with_capacity(capacity as usize),
        }
    }
    pub fn enable_encryption(&mut self, encryptor: cfb8::Encryptor<aes::Aes128>) {
        self.writer.enable_encryption(encryptor)
    }
}

impl<W> WriteHalf<W>
where
    W: AsyncWrite + Unpin,
{
    pub async fn write<'encoded>(&mut self, encoded: EncodedData<'encoded>) -> io::Result<()> {
        let packed = encoded.split_pack(self.compression.as_mut(), &mut self.compress_buf);
        self.writer.write(packed).await
    }
}
