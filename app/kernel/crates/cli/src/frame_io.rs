use craft::jsonrpc::{LineError, read_line};
use std::io::{self, BufRead, Read};

pub(crate) const MAX_FRAME_BYTES: usize = 1_048_576;

#[derive(Debug)]
pub(crate) enum FrameReadError {
    Io(io::Error),
    TooLong,
}

#[derive(Debug)]
pub(crate) struct Frame {
    pub bytes: Vec<u8>,
    pub end_of_stream: bool,
}

pub(crate) fn read_frame<R: BufRead>(reader: &mut R) -> Result<Option<Frame>, FrameReadError> {
    let mut physical = PhysicalFrame { reader, consumed: 0 };
    let frame = read_line(&mut physical, MAX_FRAME_BYTES).map_err(|error| match error {
        LineError::Io(error) => FrameReadError::Io(error),
        LineError::TooLong => FrameReadError::TooLong,
    })?;
    if physical.consumed > MAX_FRAME_BYTES {
        return Err(FrameReadError::TooLong);
    }
    Ok(frame.map(|bytes| Frame {
        end_of_stream: physical.consumed == bytes.len(),
        bytes,
    }))
}

// The shared reader excludes LF from the payload limit. NS2 limits the whole wire frame.
struct PhysicalFrame<'a, R> {
    reader: &'a mut R,
    consumed: usize,
}

impl<R: BufRead> Read for PhysicalFrame<'_, R> {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        let count = self.reader.read(bytes)?;
        self.consumed += count;
        Ok(count)
    }
}

impl<R: BufRead> BufRead for PhysicalFrame<'_, R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.reader.fill_buf()
    }

    fn consume(&mut self, count: usize) {
        self.consumed += count;
        self.reader.consume(count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, BufReader, Cursor, Read};

    #[test]
    fn counts_lf_in_physical_limit() {
        let mut wire = vec![b'x'; MAX_FRAME_BYTES];
        wire.push(b'\n');
        assert!(
            matches!(read_frame(&mut Cursor::new(wire)), Err(FrameReadError::TooLong)),
            "a maximum-length payload plus LF exceeds the physical frame limit"
        );
    }

    #[test]
    fn accepts_exact_limit_after_chunking_with_or_without_lf() {
        for ends_with_lf in [false, true] {
            let mut wire = vec![b'x'; MAX_FRAME_BYTES];
            if ends_with_lf {
                wire[MAX_FRAME_BYTES - 1] = b'\n';
            }
            let mut input = BufReader::with_capacity(467, Cursor::new(wire));
            let frame = read_frame(&mut input).unwrap().unwrap();
            assert_eq!(frame.bytes.len(), MAX_FRAME_BYTES - usize::from(ends_with_lf));
            assert_eq!(frame.end_of_stream, !ends_with_lf);
            assert!(read_frame(&mut input).unwrap().is_none());
        }
    }

    #[test]
    fn preserves_empty_lines_and_cr_bytes() {
        let mut input = BufReader::with_capacity(2, Cursor::new(b"\nexample\r\nlast"));
        assert!(read_frame(&mut input).unwrap().unwrap().bytes.is_empty());
        assert_eq!(read_frame(&mut input).unwrap().unwrap().bytes, b"example\r");
        let last = read_frame(&mut input).unwrap().unwrap();
        assert_eq!(last.bytes, b"last");
        assert!(last.end_of_stream);
        assert!(read_frame(&mut input).unwrap().is_none());
    }

    struct FailingRead(Cursor<&'static [u8]>);

    impl Read for FailingRead {
        fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
            let read = self.0.read(bytes)?;
            if read == 0 {
                return Err(io::Error::new(io::ErrorKind::BrokenPipe, "injected stdin failure"));
            }
            Ok(read)
        }
    }

    #[test]
    fn reports_io_after_a_partial_frame() {
        let mut input = BufReader::with_capacity(2, FailingRead(Cursor::new(b"partial")));
        let Err(FrameReadError::Io(error)) = read_frame(&mut input) else {
            panic!("a partial frame must fail when the underlying read fails");
        };
        assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
    }
}
