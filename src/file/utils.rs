use std::io::{self, IoSlice, Result, Write};

/// Copied from [std::io::Write::write_all_vectored]
pub fn write_all_vectored<T: Write>(this: &mut T, mut bufs: &mut [IoSlice<'_>]) -> Result<()> {
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
