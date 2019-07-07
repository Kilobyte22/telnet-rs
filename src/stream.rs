use std::time::Duration;
use std::io::{self, Read, Write, Result};
use std::net::TcpStream;
use std::result;

/// An error occured while trying to split the Stream.
pub struct SplittingError<S: SplittableStream>(pub S, pub io::Error);

impl <S> Into<io::Error> for SplittingError<S> {
    fn into(self) -> io::Error {
        self.1
    }
}

pub trait ReadingStream: Read {
    fn set_nonblocking(&self, nonblocking: bool) -> Result<()>;
    fn set_read_timeout(&self, dur: Option<Duration>) -> Result<()>;
}

pub trait WritingStream: Write {

}

pub trait SplittableStream: ReadingStream + WritingStream {
    type R: ReadingStream;
    type W: WritingStream;

    /// Splits this Stream into two halves, a reading and a writing one.
    fn split(self) -> result::Result<(Self::R, Self::W), (Self, io::Error)>;
}

impl ReadingStream for TcpStream {
    fn set_nonblocking(&self, nonblocking: bool) -> Result<()> {
        TcpStream::set_nonblocking(self, nonblocking)
    }

    fn set_read_timeout(&self, dur: Option<Duration>) -> Result<()> {
        TcpStream::set_read_timeout(self, dur)
    }
}

impl WritingStream for TcpStream {}

impl SplittableStream for TcpStream {
    type R = TcpStream;
    type W = TcpStream;

    fn split(self) -> result::Result<(Self::R, Self::W), SplittingError<Self>> {
        match self.try_clone() {
            Ok(clone) => Ok((self, clone)),
            Err(e) => Err(SplittingError(self, e))
        }
    }
}