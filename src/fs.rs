use std::fs::{File, OpenOptions, read_dir, DirEntry};
use std::path::{Path, PathBuf};
use std::io::{BufReader, BufWriter};
use std::io::{Read, Write};
use errors::*;

use io::{DataInput, KVDataInput, KVDataOutput, DataOutput};

const CHUNK_SIZE: usize = 8 * 1024;

pub(crate) struct FsDataInput {
    path: PathBuf,
    reader: BufReader<File>,
    bytes_read: usize
}

impl FsDataInput {
    pub fn open_input<T: AsRef<Path>>(path: &T) -> Result<FsDataInput> {
        if !Path::is_file(path.as_ref()) {
            bail!(ErorKind::FileNotFound(path.to_str().unwrap_or("")));
        }
        let file = File::open(path)?;
        Ok(FsDataInput{
            path: path.as_ref().to_owned(),
            reader: BufReader::with_capacity(CHUNK_SIZE, file),
            bytes_read: 0,
        })
    }
}

impl Read for FsDataInput {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reader.read(buf)
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
        let opt = OpenOptions::new().append(true).create(false);
        Self::new(opt, name)
    }

    pub fn open_new<T: AsRef<Path>>(path: &T) -> Result<FsDataOutput> {
        let options = OpenOptions::new().create(true).write(true);
        Self::new(options, path)
    }

    fn new<T: AsRef<Path>>(path: &T, options: &OpenOptions) -> Result<FsDataOutput> {
        let file = options.open(path)?;
        Ok(FSIndexOutput {
            path: name.as_ref().to_str().unwrap().to_owned(),
            writer: BufWriter::with_capacity(CHUNK_SIZE, file),
            bytes_written: 0,
        })
    }
}

impl Write for FSIndexOutput {
    fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
        let count = self.writer.write(buf)?;
        self.bytes_written += count;
        Ok(count)
    }

    fn flush(&mut self) -> ::std::io::Result<()> {
        self.writer.flush()
    }
}

impl Drop for FSIndexOutput {
    fn drop(&mut self) {
        if let Err(ref desc) = self.writer.flush() {
            println!("Oops, failed to flush {}, errmsg: {}", self.name, desc);
        }
        self.bytes_written = 0;
    }
}

impl DataOutput for FsDataOutput {}

impl KVDataOutput for FsDataOutput {}

pub fn walk_dir_entries<T: AsRef<Path>, F: Fn(&DirEntry) -> bool>(path: &T, func: F) -> Result<()> {
    assert!(Path::is_dir(path));
    let mut result = Vec::new();
    for entry in read_dir(path)? {
        let entry = entry?;
        if func(&entry) {
            break;
        }
    }
    Ok(())
}
