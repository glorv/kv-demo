use errors::Result;
use store::{DataInput, DataOutput};

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
        Ok(self.clone().into())
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        match String::from_utf8(bytes.into()) {
            Ok(s) => Ok(s),
            Err(e) => bail!("error: {}", e),
        }
    }
}

macro_rules! impl_for_num {
    ($t:ty, $sz:expr, $write_fn:expr, $read_fn:expr) => {
        impl BytesSerializer for $t {
            fn to_bytes(&self) -> Result<Vec<u8>> {
                let mut res = Vec::with_capacity($sz);
                res.resize($sz, 0u8);
                {
                    let mut slice: &mut [u8] = &mut res;
                    $write_fn(&mut slice, &self)?;
                }
                Ok(res)
            }

            fn from_bytes(bytes: &[u8]) -> Result<Self> {
                if bytes.len() < $sz {
                    bail!("the bytes length is too small!");
                }
                let data_copy = bytes[0..$sz].to_vec();
                let slice: &[u8] = &data_copy;
                $read_fn(slice)
            }
        }
    };
}

impl_for_num!(
    i32,
    4,
    |slice, v: &i32| DataOutput::write_int(slice, *v),
    |mut s| DataInput::read_int(&mut s)
);
impl_for_num!(
    u32,
    4,
    |slice, v: &u32| DataOutput::write_int(slice, *v as i32),
    |mut s| match DataInput::read_int(&mut s) {
        Ok(s) => Ok(s as u32),
        Err(e) => Err(e),
    }
);

//impl BytesSerializer for i32 {
//    fn to_bytes(&self) -> Result<Vec<u8>> {
//        let mut res = Vec::with_capacity(4);
//        res.resize(4, 0u8);
//        {
//            let mut slice: &mut [u8] = &mut res;
//            DataOutput::write_int(&mut slice, *self);
//        }
//        Ok(res)
//    }
//
//    fn from_bytes(bytes: &[u8]) -> Result<Self> {
//        if bytes.len() < 4 {
//            bail!("the bytes length is too small!");
//        }
//        let data_copy = bytes[0..4].to_vec();
//        let mut slice: &[u8] = &data_copy;
//        DataInput::read_int(&mut slice)
//    }
//}
