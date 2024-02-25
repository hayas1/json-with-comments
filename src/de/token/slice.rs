use std::iter::Peekable;

use crate::{
    de::position::{Position, RowColIterator},
    error::NeverFail,
};

use super::Tokenizer;

pub struct SliceTokenizer<'a, I>
where
    I: Iterator<Item = Result<u8, ()>>,
{
    slice: &'a [u8],
    pos: usize,
    iter: Peekable<RowColIterator<I>>,
}
impl<'a, I> SliceTokenizer<'a, I>
where
    I: Iterator<Item = Result<u8, ()>>,
{
    pub fn new(slice: &'a [u8]) -> Self {
        let (pos, iter) = (0, RowColIterator::new(slice.iter().cloned().map(Ok)).peekable());
        SliceTokenizer { slice, pos, iter }
    }
}

impl<'a, I> Tokenizer for SliceTokenizer<'a, I>
where
    I: Iterator<Item = Result<u8, ()>>,
{
    fn eat(&mut self) -> crate::Result<Option<(Position, u8)>> {
        match self.iter.next() {
            Some((pos, Ok(c))) => Ok(Some((pos, c))),
            Some((_, Err(e))) => Err(NeverFail::EmptyError)?,
            None => Ok(None),
        }
    }

    fn find(&mut self) -> crate::Result<Option<(Position, u8)>> {
        match self.iter.peek() {
            Some(&(pos, Ok(c))) => Ok(Some((pos, c))),
            Some((_, Err(e))) => Err(NeverFail::EmptyError)?,
            None => Ok(None),
        }
    }
}
