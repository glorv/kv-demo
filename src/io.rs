use std::io::{Read, Write};
use std::cmp::min;

use errors::*;

pub const MAGIC_NUMBER: i32 = 0x3FD7_6C17;

pub trait DataInput: Read {
    fn read_byte(&mut self) -> Result<u8> {
        let mut buffer = [0u8; 1];
        if self.read(&mut buffer)? != 1 {
            bail!(ErrorKind::UnexpectedEOF(
                "Reached EOF when a single byte is expected".to_owned()
            ))
        } else {
            Ok(buffer[0])
        }
    }

    fn read_bytes(&mut self, b: &mut [u8], offset: usize, length: usize) -> Result<()> {
        let end = offset + length;
        if b.len() < end {
            let msg = format!(
                "Buffer too small: wring [{}, {}) to [0, {})",
                offset,
                end,
                b.len(),
            );
            bail!(ErrorKind::IllegalArgument(msg));
        }

        let mut blob = &mut b[offset..end];

        if self.read(&mut blob)? != length {
            bail!(ErrorKind::UnexpectedEOF(format!(
                "Reached EOF when {} bytes are expected",
                length
            )))
        } else {
            Ok(())
        }
    }

    fn read_short(&mut self) -> Result<i16> {
        Ok(((i16::from(self.read_byte()?) & 0xff) << 8) | (i16::from(self.read_byte()?) & 0xff))
    }

    fn read_int(&mut self) -> Result<i32> {
        Ok(((i32::from(self.read_byte()?) & 0xff) << 24)
            | ((i32::from(self.read_byte()?) & 0xff) << 16)
            | ((i32::from(self.read_byte()?) & 0xff) << 8)
            | (i32::from(self.read_byte()?) & 0xff))
    }

    fn read_vint(&mut self) -> Result<i32> {
        let mut b = (self.read_byte()?) as i8;
        if b >= 0 {
            return Ok(i32::from(b));
        }

        let mut i = i32::from(b) & 0x7f;
        b = self.read_byte()? as i8;
        i |= (i32::from(b) & 0x7f) << 7;
        if b >= 0 {
            return Ok(i);
        }

        b = self.read_byte()? as i8;
        i |= (i32::from(b) & 0x7f) << 14;
        if b >= 0 {
            return Ok(i);
        }

        b = self.read_byte()? as i8;
        i |= (i32::from(b) & 0x7f) << 21;
        if b >= 0 {
            return Ok(i);
        }

        b = self.read_byte()? as i8;
        i |= (i32::from(b) & 0x0f) << 28;

        if (b as u8 & 0xf0) != 0 {
            bail!(ErrorKind::IllegalState("Invalid vInt detected".to_owned()));
        }

        Ok(i)
    }

    fn read_long(&mut self) -> Result<i64> {
        Ok((i64::from(self.read_int()?) << 32) | (i64::from(self.read_int()?) & 0xffff_ffff))
    }

    fn read_string(&mut self) -> Result<String> {
        const ERR_MSG: &str = "Invalid String detected";
        let length = self.read_vint()?;
        if length < 0 {
            bail!(ErrorKind::IllegalState(ERR_MSG.to_owned()));
        }

        let length = length as usize;

        let mut buffer = Vec::with_capacity(length);

        unsafe {
            buffer.set_len(length);
        };

        self.read_bytes(&mut buffer, 0, length)?;
        Ok(String::from_utf8(buffer)?)
    }

    fn skip_bytes(&mut self, count: usize) -> Result<()> {
        const SKIP_BUFFER_SIZE: usize = 1024;
        let mut skip_buffer = [0u8; SKIP_BUFFER_SIZE];
        let mut skipped = 0;

        while skipped < count {
            let step = ::std::cmp::min(SKIP_BUFFER_SIZE, count - skipped);
            self.read_bytes(&mut skip_buffer, 0, step)?;
            skipped += step;
        }
        Ok(())
    }
}

pub trait DataOutput: Write {
    fn write_byte(&mut self, b: u8) -> Result<()> {
        let buf = [b; 1];
        self.write_all(&buf)?;
        Ok(())
    }

    fn write_bytes(&mut self, b: &[u8], offset: usize, length: usize) -> Result<()> {
        let end = offset + length;
        if b.len() < end {
            bail!(ErrorKind::IllegalArgument("b.len() < end".to_owned()));
        }
        let blob = &b[offset..end];
        self.write_all(blob)?;
        Ok(())
    }

    fn write_short(&mut self, i: i16) -> Result<()> {
        self.write_byte((i >> 8) as u8)?;
        self.write_byte(i as u8)
    }

    fn write_int(&mut self, i: i32) -> Result<()> {
        self.write_byte((i >> 24) as u8)?;
        self.write_byte((i >> 16) as u8)?;
        self.write_byte((i >> 8) as u8)?;
        self.write_byte(i as u8)
    }

    fn write_vint(&mut self, i: i32) -> Result<()> {
        let mut i = i as u32;
        while (i & !0x7f_u32) != 0 {
            self.write_byte(((i & 0x7f) | 0x80) as u8)?;
            i >>= 7;
        }
        self.write_byte(i as u8)
    }

    fn write_string(&mut self, s: &str) -> Result<()> {
        let s = s.as_bytes();
        self.write_vint(s.len() as i32)?;
        self.write_bytes(s, 0, s.len())?;
        Ok(())
    }
}

pub enum KVAction {
    Put(Vec<u8>, Vec<u8>),
    Remove(Vec<u8>),
}

const ACTION_PUT: u8 = 1;
const ACTION_REMOVE: u8 = 2;

pub trait KVDataInput: DataInput {
    fn read_byte_array(&mut self, allow_empty: bool) -> Result<Vec<u8>> {
        let key_len = self.read_vint()?;
        if key_len < 0 {
            bail!("invalid data length '{}'", key_len);
        }
        if key_len == 0 {
            if !allow_empty {
                bail!("invalid data length '{}'", key_len)
            } else {
                Ok(Vec::new())
            }
        } else {
            let key_len = key_len as usize;
            let mut key = Vec::with_capacity(key_len);
            key.resize(key_len, 0);
            self.read_bytes(&mut key, 0, key_len)?;
            Ok(key)
        }
    }

    fn read_kv_action(&mut self) -> Result<KVAction> {
        let action_type = self.read_byte()?;
        if action_type != ACTION_PUT && action_type != ACTION_REMOVE {
            bail!("unknown action type '{}'", action_type);
        }

        let key = self.read_byte_array(false)?;
        if action_type == ACTION_PUT {
            let value = self.read_byte_array(true)?;
            Ok(KVAction::Put(key, value))
        } else {
            Ok(KVAction::Remove(key))
        }
    }
}

pub trait KVDataOutput: DataOutput + Send + Sync {
    fn write_byte_array(&mut self, data: &[u8]) -> Result<()> {
        let length = data.len();
        self.write_vint(length as i32)?;
        if length > 0 {
            self.write_bytes(data, 0, length)?;
        }
        Ok(())
    }

    fn write_kv_action(&mut self, action: &KVAction) -> Result<()> {
        match action {
            &KVAction::Put(ref key, ref value) => {
                self.write_byte(ACTION_PUT)?;
                self.write_byte_array(key)?;
                self.write_byte_array(value)?;
            }
            &KVAction::Remove(ref key) => {
                self.write_byte(ACTION_REMOVE);
                self.write_byte_array(key);
            }
        }
        Ok(())
    }
}

impl<'a> DataInput for &'a [u8] {}

impl<'a> DataOutput for &'a mut [u8] {}