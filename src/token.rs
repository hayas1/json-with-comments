use std::io;

pub struct Tokenizer<R>
where
    R: io::Read,
{
    iter: RowColIterator<io::Bytes<R>>,
}
impl<R> Tokenizer<R>
where
    R: io::Read,
{
    pub fn new(reader: R) -> Self {
        Tokenizer { iter: RowColIterator::new(reader.bytes()) }
    }
}
impl<R> Tokenizer<R>
where
    R: io::Read,
{
    pub fn pos(&self) -> Position {
        self.iter.pos()
    }
}

pub type Position = (usize, usize);
pub struct RowColIterator<I> {
    iter: I,
    row: usize,
    col: usize,
}
impl<I> RowColIterator<I> {
    pub fn new(iter: I) -> Self {
        RowColIterator { iter, row: 0, col: 0 }
    }

    pub fn pos(&self) -> Position {
        (self.row, self.col)
    }
}
impl<I> Iterator for RowColIterator<I>
where
    I: Iterator<Item = io::Result<u8>>,
{
    type Item = (Position, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|c| {
            let pos = self.pos();
            match c {
                Ok(b'\n') => {
                    self.row += 1;
                    self.col = 0;
                }
                Ok(_) => self.col += 1,
                Err(_) => {}
            }
            (pos, c)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufReader, Read},
    };

    use super::*;

    #[test]
    fn behavior_row_col_iterator() {
        let reader = BufReader::new(File::open("tests/data/list.json").unwrap());
        let mut iter = RowColIterator::new(reader.bytes());
        assert!(matches!(iter.next(), Some(((0, 0), Ok(b'[')))));
        assert!(matches!(iter.next(), Some(((0, 1), Ok(b'\n')))));

        assert!(matches!(iter.next(), Some(((1, 0), Ok(b' ')))));
        assert!(matches!(iter.next(), Some(((1, 1), Ok(b' ')))));
        assert!(matches!(iter.next(), Some(((1, 2), Ok(b' ')))));
        assert!(matches!(iter.next(), Some(((1, 3), Ok(b' ')))));
        assert!(matches!(iter.next(), Some(((1, 4), Ok(b'"')))));
        assert!(matches!(iter.next(), Some(((1, 5), Ok(b'f')))));
        assert!(matches!(iter.next(), Some(((1, 6), Ok(b'o')))));
        assert!(matches!(iter.next(), Some(((1, 7), Ok(b'o')))));
        assert!(matches!(iter.next(), Some(((1, 8), Ok(b'"')))));
        assert!(matches!(iter.next(), Some(((1, 9), Ok(b',')))));
        assert!(matches!(iter.next(), Some(((1, 10), Ok(b'\n')))));

        assert!(matches!(iter.next(), Some(((2, 0), Ok(b' ')))));
        assert!(matches!(iter.next(), Some(((2, 1), Ok(b' ')))));
        assert!(matches!(iter.next(), Some(((2, 2), Ok(b' ')))));
        assert!(matches!(iter.next(), Some(((2, 3), Ok(b' ')))));
        assert!(matches!(iter.next(), Some(((2, 4), Ok(b'"')))));
        assert!(matches!(iter.next(), Some(((2, 5), Ok(b'b')))));
        assert!(matches!(iter.next(), Some(((2, 6), Ok(b'a')))));
        assert!(matches!(iter.next(), Some(((2, 7), Ok(b'r')))));
        assert!(matches!(iter.next(), Some(((2, 8), Ok(b'"')))));
        assert!(matches!(iter.next(), Some(((2, 9), Ok(b',')))));
        assert!(matches!(iter.next(), Some(((2, 10), Ok(b'\n')))));

        assert!(matches!(iter.next(), Some(((3, 0), Ok(b' ')))));
        assert!(matches!(iter.next(), Some(((3, 1), Ok(b' ')))));
        assert!(matches!(iter.next(), Some(((3, 2), Ok(b' ')))));
        assert!(matches!(iter.next(), Some(((3, 3), Ok(b' ')))));
        assert!(matches!(iter.next(), Some(((3, 4), Ok(b'"')))));
        assert!(matches!(iter.next(), Some(((3, 5), Ok(b'b')))));
        assert!(matches!(iter.next(), Some(((3, 6), Ok(b'a')))));
        assert!(matches!(iter.next(), Some(((3, 7), Ok(b'z')))));
        assert!(matches!(iter.next(), Some(((3, 8), Ok(b'"')))));
        assert!(matches!(iter.next(), Some(((3, 9), Ok(b'\n')))));

        assert!(matches!(iter.next(), Some(((4, 0), Ok(b']')))));
        assert!(matches!(iter.next(), Some(((4, 1), Ok(b'\n')))));

        assert!(matches!(iter.next(), None));
        assert!(matches!(iter.next(), None));
        assert!(matches!(iter.next(), None));
    }
}
