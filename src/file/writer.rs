use std::{
    fs::File,
    io::{self, IoSlice, Result, Write},
    path::Path,
};

struct Chunk {
    offset: u64,
    data: Box<[u8]>,
}

pub struct FileWriter<T: Write> {
    file: T,
    total_bytes: usize,
    chunks: Vec<Chunk>,
    frame_size: u16,
}

impl FileWriter<File> {
    pub fn open(path: impl AsRef<Path>, frame_size: u16) -> Result<Self> {
        let file = File::options().create(true).write(true).open(path)?;

        Ok(FileWriter::new(file, frame_size))
    }
}

impl<T: Write> FileWriter<T> {
    pub fn new(file: T, frame_size: u16) -> Self {
        Self {
            file,
            frame_size,
            total_bytes: 0,
            chunks: vec![],
        }
    }

    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    pub fn write(&mut self, no: u64, data: Box<[u8]>) {
        assert!(data.len() <= self.total_bytes);

        self.total_bytes += data.len();

        let chunk = Chunk {
            offset: no * self.frame_size as u64,
            data,
        };

        let result = self
            .chunks
            .binary_search_by(|c| c.offset.cmp(&chunk.offset));

        match result {
            Ok(index) => self.chunks[index] = chunk,
            Err(index) => self.chunks.insert(index, chunk),
        }
    }

    pub fn flush(&mut self) -> Result<()> {
        if self.chunks.is_empty() {
            return Ok(());
        }

        write_all_vectored(&mut self.file, &mut [])?;

        Ok(())
    }
}

impl<T: Write> Drop for FileWriter<T> {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

/// Copied from [std::io::Write::write_all_vectored]
fn write_all_vectored<T: Write>(this: &mut T, mut bufs: &mut [IoSlice<'_>]) -> Result<()> {
    IoSlice::advance_slices(&mut bufs, 0);
    while !bufs.is_empty() {
        match this.write_vectored(bufs) {
            Ok(0) => {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "failed to write whole buffer",
                ));
            }
            Ok(n) => IoSlice::advance_slices(&mut bufs, n),
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}
