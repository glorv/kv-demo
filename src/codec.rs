use errors::Result;
use io::{DataOutput, DataInput};

use std::io::Read;

pub trait BytesSerializer: Sized {
    fn to_bytes(&self) -> Result<Vec<u8>>;

    fn from_bytes(bytes: &[u8]) -> Result<Self>;
}

impl BytesSerializer for Vec<u8> {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.clone())
    }

    fn from_bytes(bytes: &[u8]) -> Result<Vec<u8>> {
        Ok(bytes.to_vec())
    }
}

impl BytesSerializer for String {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.into_bytes())
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        match String::from_utf8(bytes.into()) {
            Ok(s) => Ok(s),
            Err(e) => bail!("error: {}", e),
        }
    }
}



impl BytesSerializer for i32 {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut res = Vec::with_capacity(4);
        res.resize(4, 0u8);

        let slice: &mut [u8] = &mut res;
        DataOutput::write_int(&mut slice, *self);
        Ok(res)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 4 {
            bail!("the bytes length is too small!");
        }
        let mut data_copy = bytes[0..4].to_vec();
        let mut slice: &[u8] = &data_copy;
        slice.read_int()
    }
}