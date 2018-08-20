use errors::*;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use store::{DataInput, DataOutput, KVDataInput, KVDataOutput};

const CHUNK_SIZE: usize = 8 * 1024;

pub(crate) struct FsDataInput {
    _path: PathBuf,
    pub reader: BufReader<File>,
    bytes_read: usize,
}

impl FsDataInput {
    pub fn open_input<T: AsRef<Path>>(path: &T) -> Result<FsDataInput> {
        if !Path::is_file(path.as_ref()) {
            bail!(ErrorKind::FileNotFound(
                path.as_ref().to_str().unwrap_or("").to_string()
            ));
        }
        let file = File::open(path)?;
        Ok(FsDataInput {
            _path: path.as_ref().to_path_buf(),
            reader: BufReader::with_capacity(CHUNK_SIZE, file),
            bytes_read: 0,
        })
    }

    pub fn is_end(&self) -> Result<bool> {
        Ok(self.reader.get_ref().metadata()?.len() <= self.bytes_read as u64)
    }
}

impl Read for FsDataInput {
    fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
        match self.reader.read(buf) {
            Ok(s) => {
                self.bytes_read += s;
                Ok(s)
            }
            Err(e) => Err(e),
        }
    }
}

impl DataInput for FsDataInput {}
impl KVDataInput for FsDataInput {}

pub(crate) struct FsDataOutput {
    path: String,
    writer: BufWriter<File>,
    bytes_written: usize,
}

impl FsDataOutput {
    pub fn open_exist<T: AsRef<Path>>(name: &T) -> Result<FsDataOutput> {
        Self::new(name, OpenOptions::new().append(true).create(false))
    }

    pub fn open_new<T: AsRef<Path>>(path: &T) -> Result<FsDataOutput> {
        Self::new(path, OpenOptions::new().create(true).write(true))
    }

    fn new<T: AsRef<Path>>(path: &T, options: &OpenOptions) -> Result<FsDataOutput> {
        let file = options.open(path)?;
        Ok(FsDataOutput {
            path: path.as_ref().to_str().unwrap().to_owned(),
            writer: BufWriter::with_capacity(CHUNK_SIZE, file),
            bytes_written: 0,
        })
    }
}

impl Write for FsDataOutput {
    fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
        let count = self.writer.write(buf)?;
        self.bytes_written += count;
        Ok(count)
    }

    fn flush(&mut self) -> ::std::io::Result<()> {
        self.writer.flush()
    }
}

impl Drop for FsDataOutput {
    fn drop(&mut self) {
        if let Err(ref desc) = self.flush() {
            error!("Oops, failed to flush {}, errmsg: {}", self.path, desc);
        }
        self.bytes_written = 0;
    }
}

impl DataOutput for FsDataOutput {}

impl KVDataOutput for FsDataOutput {}
