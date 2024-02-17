use std::io;

pub struct Tokenizer<R> {
    iter: RowColIterator<R>,
}
impl<R> Tokenizer<R> {
    pub fn new(iter: RowColIterator<R>) -> Self {
        Tokenizer { iter }
    }
}
impl<R> Tokenizer<R>
where
    R: io::Read,
{
    pub fn pos(&self) -> (usize, usize) {
        self.iter.pos()
    }
}

pub struct RowColIterator<I> {
    iter: I,
    row: usize,
    col: usize,
}
impl<I> RowColIterator<I> {
    pub fn new(iter: I) -> Self {
        RowColIterator { iter, row: 0, col: 0 }
    }

    pub fn pos(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}
impl<I> Iterator for RowColIterator<I>
where
    I: Iterator<Item = io::Result<u8>>,
{
    type Item = io::Result<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|c| {
            match c {
                Ok(b'\n') => {
                    self.row += 1;
                    self.col = 0;
                }
                Ok(_) => self.col += 1,
                Err(_) => {}
            }
            c
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
        assert!(matches!((iter.pos(), iter.next()), ((0, 0), Some(Ok(b'[')))));
        assert!(matches!((iter.pos(), iter.next()), ((0, 1), Some(Ok(b'\n')))));

        assert!(matches!((iter.pos(), iter.next()), ((1, 0), Some(Ok(b' ')))));
        assert!(matches!((iter.pos(), iter.next()), ((1, 1), Some(Ok(b' ')))));
        assert!(matches!((iter.pos(), iter.next()), ((1, 2), Some(Ok(b' ')))));
        assert!(matches!((iter.pos(), iter.next()), ((1, 3), Some(Ok(b' ')))));
        assert!(matches!((iter.pos(), iter.next()), ((1, 4), Some(Ok(b'"')))));
        assert!(matches!((iter.pos(), iter.next()), ((1, 5), Some(Ok(b'f')))));
        assert!(matches!((iter.pos(), iter.next()), ((1, 6), Some(Ok(b'o')))));
        assert!(matches!((iter.pos(), iter.next()), ((1, 7), Some(Ok(b'o')))));
        assert!(matches!((iter.pos(), iter.next()), ((1, 8), Some(Ok(b'"')))));
        assert!(matches!((iter.pos(), iter.next()), ((1, 9), Some(Ok(b',')))));
        assert!(matches!((iter.pos(), iter.next()), ((1, 10), Some(Ok(b'\n')))));

        assert!(matches!((iter.pos(), iter.next()), ((2, 0), Some(Ok(b' ')))));
        assert!(matches!((iter.pos(), iter.next()), ((2, 1), Some(Ok(b' ')))));
        assert!(matches!((iter.pos(), iter.next()), ((2, 2), Some(Ok(b' ')))));
        assert!(matches!((iter.pos(), iter.next()), ((2, 3), Some(Ok(b' ')))));
        assert!(matches!((iter.pos(), iter.next()), ((2, 4), Some(Ok(b'"')))));
        assert!(matches!((iter.pos(), iter.next()), ((2, 5), Some(Ok(b'b')))));
        assert!(matches!((iter.pos(), iter.next()), ((2, 6), Some(Ok(b'a')))));
        assert!(matches!((iter.pos(), iter.next()), ((2, 7), Some(Ok(b'r')))));
        assert!(matches!((iter.pos(), iter.next()), ((2, 8), Some(Ok(b'"')))));
        assert!(matches!((iter.pos(), iter.next()), ((2, 9), Some(Ok(b',')))));
        assert!(matches!((iter.pos(), iter.next()), ((2, 10), Some(Ok(b'\n')))));

        assert!(matches!((iter.pos(), iter.next()), ((3, 0), Some(Ok(b' ')))));
        assert!(matches!((iter.pos(), iter.next()), ((3, 1), Some(Ok(b' ')))));
        assert!(matches!((iter.pos(), iter.next()), ((3, 2), Some(Ok(b' ')))));
        assert!(matches!((iter.pos(), iter.next()), ((3, 3), Some(Ok(b' ')))));
        assert!(matches!((iter.pos(), iter.next()), ((3, 4), Some(Ok(b'"')))));
        assert!(matches!((iter.pos(), iter.next()), ((3, 5), Some(Ok(b'b')))));
        assert!(matches!((iter.pos(), iter.next()), ((3, 6), Some(Ok(b'a')))));
        assert!(matches!((iter.pos(), iter.next()), ((3, 7), Some(Ok(b'z')))));
        assert!(matches!((iter.pos(), iter.next()), ((3, 8), Some(Ok(b'"')))));
        assert!(matches!((iter.pos(), iter.next()), ((3, 9), Some(Ok(b'\n')))));

        assert!(matches!((iter.pos(), iter.next()), ((4, 0), Some(Ok(b']')))));
        assert!(matches!((iter.pos(), iter.next()), ((4, 1), Some(Ok(b'\n')))));

        assert!(matches!((iter.pos(), iter.next()), ((5, 0), None)));
        assert!(matches!((iter.pos(), iter.next()), ((5, 0), None)));
        assert!(matches!((iter.pos(), iter.next()), ((5, 0), None)));
    }
}
