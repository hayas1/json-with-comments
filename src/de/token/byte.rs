use std::{io, iter::Peekable};

use crate::de::position::Position;

use super::{RowColIterator, Tokenizer};

pub struct ByteTokenizer<R>
where
    R: io::Read,
{
    iter: Peekable<RowColIterator<io::Bytes<R>>>,
}
impl<R> ByteTokenizer<R>
where
    R: io::Read,
{
    pub fn new(read: R) -> Self {
        ByteTokenizer { iter: RowColIterator::new(read.bytes()).peekable() }
    }
}

impl<R> Tokenizer for ByteTokenizer<R>
where
    R: io::Read,
{
    fn eat(&mut self) -> crate::Result<Option<(Position, u8)>> {
        match self.iter.next() {
            Some((pos, Ok(c))) => Ok(Some((pos, c))),
            Some((_, Err(e))) => Err(crate::Error::new(e.to_string())), // TODO handling io error
            None => Ok(None),
        }
    }

    fn find(&mut self) -> crate::Result<Option<(Position, u8)>> {
        match self.iter.peek() {
            Some(&(pos, Ok(c))) => Ok(Some((pos, c))),
            Some((_, Err(e))) => Err(crate::Error::new(e.to_string())), // TODO handling io error
            None => Ok(None),
        }
    }
}
