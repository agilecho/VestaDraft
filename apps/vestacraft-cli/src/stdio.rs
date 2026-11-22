use crate::frame_io::{FrameReadError, read_frame};
use crate::session_v2::Session;
use std::io::{self, BufRead, Write};
use std::sync::mpsc;
use std::time::Duration;
use vesta_model::{Error, Result};
fn emit(values: Vec<serde_json::Value>) -> Result<()> {
    let mut out = io::stdout().lock();
    for v in values {
        writeln!(out, "{v}").map_err(|e| Error::new("E_IO", e.to_string()))?;
    }
    out.flush().map_err(|e| Error::new("E_IO", e.to_string()))
}
fn forward_frames(
    mut input: impl BufRead,
    tx: mpsc::SyncSender<std::result::Result<Vec<u8>, &'static str>>,
) {
    loop {
        match read_frame(&mut input) {
            Ok(Some(frame)) => {
                if tx.send(Ok(frame.bytes)).is_err() || frame.end_of_stream {
                    return;
                }
            }
            Ok(None) => return,
            Err(FrameReadError::Io(_error)) => return,
            Err(FrameReadError::TooLong) => {
                let _ = tx.send(Err("frame exceeds 1048576 bytes"));
                return;
            }
        }
    }
}

pub fn serve(mut session: Session) -> Result<()> {
    let (tx, rx) = mpsc::sync_channel(8);
    std::thread::spawn(move || forward_frames(io::stdin().lock(), tx));
    loop {
        let next = if session.has_pending() {
            match rx.recv_timeout(Duration::from_millis(1)) {
                Ok(v) => Some(v),
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    emit(session.advance())?;
                    continue;
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    emit(session.advance())?;
                    None
                }
            }
        } else {
            rx.recv().ok()
        };
        let Some(frame) = next else {
            if session.has_unsettled() {
                return Err(Error::new(
                    "E_IO",
                    "EOF with unconfirmed engineering snapshots; artifacts may exist, Saved not granted",
                ));
            }
            return Ok(());
        };
        match frame {
            Err(s) => {
                emit(vec![session.malformed("N_FRAME", s)])?;
                return Err(Error::new("E_SCHEMA", s));
            }
            Ok(frame) => {
                if std::str::from_utf8(&frame).is_err() {
                    emit(vec![session.malformed("N_UTF8", "strict UTF8 required")])?;
                    return Err(Error::new("E_SCHEMA", "UTF8"));
                }
                emit(session.handle(&frame))?;
                if session.shutdown {
                    return Ok(());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Cursor, Read};

    struct FailsAtEnd(Cursor<&'static [u8]>);

    impl Read for FailsAtEnd {
        fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
            match self.0.read(bytes)? {
                0 => Err(io::Error::other("injected stdin failure")),
                count => Ok(count),
            }
        }
    }

    #[test]
    fn stdin_error_discards_only_the_unfinished_frame() {
        let input = BufReader::with_capacity(3, FailsAtEnd(Cursor::new(b"first\nunfinished")));
        let (tx, rx) = mpsc::sync_channel(2);
        forward_frames(input, tx);
        assert_eq!(rx.into_iter().collect::<Vec<_>>(), [Ok(b"first".to_vec())]);
    }

    #[test]
    fn stdin_is_never_polled_again_after_unterminated_frame_at_eof() {
        struct EofOnce(Cursor<&'static [u8]>, bool);
        impl Read for EofOnce {
            fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
                assert!(!self.1, "stdin was read again after EOF");
                let count = self.0.read(bytes)?;
                self.1 = count == 0;
                Ok(count)
            }
        }
        let (tx, rx) = mpsc::sync_channel(1);
        let reader = BufReader::new(EofOnce(Cursor::new(b"last"), false));
        forward_frames(reader, tx);
        assert_eq!(rx.into_iter().collect::<Vec<_>>(), [Ok(b"last".to_vec())]);
    }
}
