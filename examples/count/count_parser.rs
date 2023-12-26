mod parser {
    #![allow(non_camel_case_types)]
    #![allow(non_upper_case_globals)]
    #![allow(dead_code)]
    #![allow(non_snake_case)]
    #![allow(unused_braces)]
    #![allow(unreachable_patterns)]
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
    enum Token {
        Tok_8186225505942432243,
    }
    lazy_static! {}
    fn parse_Tok_8186225505942432243<'a>(parser: &mut ParserState<'a>) -> Option<&'a str> {
        parser.expect("a")
    }
    fn check_Tok_8186225505942432243(parser: &mut ParserState) -> bool {
        parser.is_prefix("a")
    }
    impl ParserState<'_> {
        fn token(&mut self) -> Option<Token> {
            if check_Tok_8186225505942432243(self) {
                return Some(Token::Tok_8186225505942432243);
            }
            None
        }
    }
    fn parse_count(parser: &mut ParserState, acc: i32) -> Option<i32> {
        match parser.token() {
            None => Some({ acc }),
            Some(Token::Tok_8186225505942432243) => {
                let __ = parse_Tok_8186225505942432243(parser)?;
                let cont = parse_count(parser, acc + 1)?;
                Some({ cont })
            }
            _ => None,
        }
    }
    pub fn parse_expr(parser: &mut ParserState) -> Option<i32> {
        match parser.token() {
            Some(Token::Tok_8186225505942432243) | None => {
                let cnt = parse_count(parser, 0)?;
                Some({ cnt })
            }
            _ => None,
        }
    }
}
