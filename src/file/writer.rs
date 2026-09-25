use std::{
    fs::File,
    io::{self, IoSlice, Result, Seek, SeekFrom, Write},
    path::Path,
};

struct Chunk {
    offset: u64,
    data: Box<[u8]>,
}

pub struct FileWriter<T: Write + Seek> {
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

impl<T: Write + Seek> FileWriter<T> {
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
        assert!(data.len() <= self.frame_size as usize);

        let chunk = Chunk {
            offset: no * self.frame_size as u64,
            data,
        };

        let result = self
            .chunks
            .binary_search_by(|c| c.offset.cmp(&chunk.offset));

        match result {
            Ok(index) => self.chunks[index] = chunk,
            Err(index) => {
                self.total_bytes += chunk.data.len();
                self.chunks.insert(index, chunk);
            }
        }
    }

    pub fn flush(&mut self) -> Result<()> {
        let mut chunks = self.chunks.iter().peekable();

        while let Some(buf) = chunks.next() {
            let start_offset = buf.offset;
            let mut bufs = vec![IoSlice::new(&buf.data)];

            let mut expected_offset = start_offset + buf.data.len() as u64;

            while let Some(next) = chunks.next_if(|&next| next.offset == expected_offset) {
                bufs.push(IoSlice::new(&next.data));
                expected_offset += next.data.len() as u64;
            }

            self.file.seek(SeekFrom::Start(start_offset))?;
            write_all_vectored(&mut self.file, &mut bufs)?;
        }

        self.chunks.clear();
        self.total_bytes = 0;

        Ok(())
    }
}

impl<T: Write + Seek> Drop for FileWriter<T> {
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
    use std::io::Cursor;

    fn create_writer(frame_size: u16) -> FileWriter<Cursor<Vec<u8>>> {
        FileWriter::new(Cursor::new(Vec::new()), frame_size)
    }

    fn contents(writer: &FileWriter<Cursor<Vec<u8>>>) -> &[u8] {
        writer.file.get_ref()
    }

    #[test]
    fn writes_contiguous_chunks() {
        let mut writer = create_writer(2);

        writer.write(0, Box::new([0, 0]));
        writer.write(1, Box::new([1, 1]));
        writer.write(2, Box::new([2]));

        assert_eq!(writer.total_bytes(), 5);
        writer.flush().unwrap();

        assert_eq!(contents(&writer), &[0, 0, 1, 1, 2]);
        assert_eq!(writer.total_bytes(), 0);
    }

    #[test]
    fn writes_chunks_out_of_order() {
        let mut writer = create_writer(2);

        writer.write(2, Box::new([2]));
        writer.write(0, Box::new([0, 0]));
        writer.write(1, Box::new([1, 1]));

        writer.flush().unwrap();
        assert_eq!(contents(&writer), &[0, 0, 1, 1, 2]);
    }

    #[test]
    fn writes_chunks_with_a_gap() {
        let mut writer = create_writer(2);

        writer.write(0, Box::new([0, 0]));
        writer.write(2, Box::new([2, 2]));

        writer.flush().unwrap();
        assert_eq!(contents(&writer), &[0, 0, 0, 0, 2, 2]);
    }

    #[test]
    fn writes_partial_chunk() {
        let mut writer = create_writer(4);

        writer.write(0, Box::new([1, 2]));
        writer.write(1, Box::new([3, 4]));

        writer.flush().unwrap();
        assert_eq!(contents(&writer), &[1, 2, 0, 0, 3, 4]);
    }

    #[test]
    fn overwrites_existing_chunk() {
        let mut writer = create_writer(2);

        writer.write(0, Box::new([1, 1]));
        writer.write(0, Box::new([2, 2]));

        assert_eq!(writer.total_bytes(), 2);

        writer.flush().unwrap();
        assert_eq!(contents(&writer), &[2, 2]);
    }

    #[test]
    fn can_write_after_flush() {
        let mut writer = create_writer(2);

        writer.write(0, Box::new([1, 2]));
        writer.flush().unwrap();

        writer.write(2, Box::new([3, 4]));
        writer.flush().unwrap();

        assert_eq!(contents(&writer), &[1, 2, 0, 0, 3, 4]);
    }

    #[test]
    fn writing_past_eof_creates_zero_filled_gap() {
        let mut writer = create_writer(2);

        writer.write(3, Box::new([6]));
        writer.flush().unwrap();

        assert_eq!(contents(&writer), &[0, 0, 0, 0, 0, 0, 6]);
    }
}
