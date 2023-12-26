use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;

pub struct ParserState<'input> {
    stream: &'input str,
}

impl<'input> ParserState<'input> {
    pub fn new(stream: &'input str) -> Self {
        ParserState { stream }
    }

    pub fn remainder(&self) -> &'input str {
        self.stream
    }

    pub fn push_spaces(&mut self) {
        self.stream = self.stream.trim_start()
    }

    pub fn is_prefix(&mut self, prefix: &str) -> bool {
        self.push_spaces();
        self.stream.starts_with(prefix)
    }

    pub fn is_prefix_re(&mut self, regex: &Regex) -> bool {
        self.push_spaces();
        regex.find(self.stream).is_some_and(|m| m.start() == 0)
    }

    pub fn expect(&mut self, prefix: &str) -> Option<&'input str> {
        self.push_spaces();

        if self.stream.starts_with(prefix) {
            let res = Some(&self.stream[..prefix.len()]);
            self.bump(prefix.len());
            return res;
        } else {
            None
        }
    }

    pub fn expect_re(&mut self, re: &Regex) -> Option<&'input str> {
        self.push_spaces();

        re.find(self.stream).filter(|m| m.start() == 0).map(|m| {
            let res = &self.stream[m.range()];
            self.bump(m.len());
            res
        })
    }

    pub fn bump(&mut self, bytes: usize) {
        self.stream = &self.stream[bytes..];
    }
}

#[derive(Debug, Clone)]
pub enum ParseError<T> {
    UnexpectedToken { actual: Option<T>, expected: T },
    NoRuleFound(&'static str),
}

impl<T: fmt::Display> fmt::Display for ParseError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken { actual, expected } => match actual {
                Some(actual) => {
                    write!(f, "Unexpected token: Expect {expected}, but found {actual}")
                }
                None => write!(f, "Unexpected token: Expect {expected}, but found None"),
            },
            ParseError::NoRuleFound(state) => write!(f, "while parsing {state}, found no rules"),
        }
    }
}
