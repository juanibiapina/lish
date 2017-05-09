use std::ops::{Range, RangeTo, RangeFrom, RangeFull};
use std::iter::Enumerate;
use std::fmt;

use nom::{AsChar, Slice, InputIter, InputLength};

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Word(String),
    Illegal(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Word(ref s) => {
                write!(f, "{}", s)
            },
            Token::Illegal(ref s) => {
                write!(f, "{}", s)
            },
        }
    }
}

// Needed for nom to work with non u8 slices: https://github.com/Geal/nom/issues/380
// Copied from: https://github.com/Rydgel/monkey-rust/blob/3712c778694179925a6a9dc314f6a80f1c21acd6/lib/lexer/token.rs
#[derive(Clone,Copy,PartialEq,Debug)]
pub struct Tokens<'a> {
    pub tok: &'a[Token],
    pub start: usize,
    pub end: usize,
}

impl<'a> Tokens<'a> {
    pub fn new(vec: &'a Vec<Token>) -> Self {
        Tokens {
            tok: vec.as_slice(),
            start: 0,
            end: vec.len(),
        }
    }
}

impl<'a> AsChar for &'a Token {
    #[inline]
    fn as_char(self) -> char {
        '\0'
    }

    #[inline]
    fn is_alpha(self) -> bool {
        false
    }

    #[inline]
    fn is_alphanum(self) -> bool {
        false
    }

    #[inline]
    fn is_dec_digit(self) -> bool {
        false
    }

    #[inline]
    fn is_hex_digit(self) -> bool {
        false
    }

    #[inline]
    fn is_oct_digit(self) -> bool {
        false
    }
}

impl AsChar for Token {
    #[inline]
    fn as_char(self) -> char {
        '\0'
    }

    #[inline]
    fn is_alpha(self) -> bool {
        false
    }

    #[inline]
    fn is_alphanum(self) -> bool {
        false
    }

    #[inline]
    fn is_dec_digit(self) -> bool {
        false
    }

    #[inline]
    fn is_hex_digit(self) -> bool {
        false
    }

    #[inline]
    fn is_oct_digit(self) -> bool {
        false
    }
}

impl<'a> InputLength for Tokens<'a> {
    #[inline]
    fn input_len(&self) -> usize {
        self.tok.len()
    }
}

impl InputLength for Token {
    #[inline]
    fn input_len(&self) -> usize {
        1
    }
}

impl<'a> Slice<Range<usize>> for Tokens<'a> {
    #[inline]
    fn slice(&self, range: Range<usize>) -> Self {
        Tokens {
            tok: self.tok.slice(range.clone()),
            start: self.start + range.start,
            end: self.start + range.end,
        }
    }
}

impl<'a> Slice<RangeTo<usize>> for Tokens<'a> {
    #[inline]
    fn slice(&self, range: RangeTo<usize>) -> Self {
        self.slice(0..range.end)
    }
}

impl<'a> Slice<RangeFrom<usize>> for Tokens<'a> {
    #[inline]
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        self.slice(range.start..self.end - self.start)
    }
}

impl<'a> Slice<RangeFull> for Tokens<'a> {
    #[inline]
    fn slice(&self, _: RangeFull) -> Self {
        Tokens {
            tok: self.tok,
            start: self.start,
            end: self.end,
        }
    }
}

impl<'a> InputIter for Tokens<'a> {
    type Item     = &'a Token;
    type RawItem  = Token;
    type Iter     = Enumerate<::std::slice::Iter<'a, Token>>;
    type IterElem = ::std::slice::Iter<'a, Token>;

    #[inline]
    fn iter_indices(&self) -> Enumerate<::std::slice::Iter<'a, Token>> {
        self.tok.iter().enumerate()
    }
    #[inline]
    fn iter_elements(&self) -> ::std::slice::Iter<'a, Token> {
        self.tok.iter()
    }
    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize> where P: Fn(Self::RawItem) -> bool {
        self.tok.iter().position(|b| predicate(b.clone()))
    }
    #[inline]
    fn slice_index(&self, count:usize) -> Option<usize> {
        if self.tok.len() >= count {
            Some(count)
        } else {
            None
        }
    }
}
