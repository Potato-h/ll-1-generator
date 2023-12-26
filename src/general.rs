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
