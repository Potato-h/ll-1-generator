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
    use std::str::FromStr;
    enum Token {
        Tok_15975982353842843148,
        Tok_13743468659553110316,
        Tok_15461786420412564008,
        Tok_14924153705535855226,
        Tok_7874756943448743542,
        Tok_13536687847573022133,
        Tok_4104316355815137153,
    }
    lazy_static! {
        static ref RE_Tok_4104316355815137153: Regex = Regex::new("\\A[0-9]+").unwrap();
    }
    fn parse_Tok_15975982353842843148<'a>(parser: &mut ParserState<'a>) -> Option<&'a str> {
        parser.expect("(")
    }
    fn check_Tok_15975982353842843148(parser: &mut ParserState) -> bool {
        parser.is_prefix("(")
    }
    fn parse_Tok_13743468659553110316<'a>(parser: &mut ParserState<'a>) -> Option<&'a str> {
        parser.expect(")")
    }
    fn check_Tok_13743468659553110316(parser: &mut ParserState) -> bool {
        parser.is_prefix(")")
    }
    fn parse_Tok_15461786420412564008<'a>(parser: &mut ParserState<'a>) -> Option<&'a str> {
        parser.expect("*")
    }
    fn check_Tok_15461786420412564008(parser: &mut ParserState) -> bool {
        parser.is_prefix("*")
    }
    fn parse_Tok_14924153705535855226<'a>(parser: &mut ParserState<'a>) -> Option<&'a str> {
        parser.expect("/")
    }
    fn check_Tok_14924153705535855226(parser: &mut ParserState) -> bool {
        parser.is_prefix("/")
    }
    fn parse_Tok_7874756943448743542<'a>(parser: &mut ParserState<'a>) -> Option<&'a str> {
        parser.expect("+")
    }
    fn check_Tok_7874756943448743542(parser: &mut ParserState) -> bool {
        parser.is_prefix("+")
    }
    fn parse_Tok_13536687847573022133<'a>(parser: &mut ParserState<'a>) -> Option<&'a str> {
        parser.expect("-")
    }
    fn check_Tok_13536687847573022133(parser: &mut ParserState) -> bool {
        parser.is_prefix("-")
    }
    fn parse_Tok_4104316355815137153<'a>(parser: &mut ParserState<'a>) -> Option<&'a str> {
        parser.expect_re(&RE_Tok_4104316355815137153)
    }
    fn check_Tok_4104316355815137153(parser: &mut ParserState) -> bool {
        parser.is_prefix_re(&RE_Tok_4104316355815137153)
    }
    impl ParserState<'_> {
        fn token(&mut self) -> Option<Token> {
            if check_Tok_15975982353842843148(self) {
                return Some(Token::Tok_15975982353842843148);
            }
            if check_Tok_13743468659553110316(self) {
                return Some(Token::Tok_13743468659553110316);
            }
            if check_Tok_15461786420412564008(self) {
                return Some(Token::Tok_15461786420412564008);
            }
            if check_Tok_14924153705535855226(self) {
                return Some(Token::Tok_14924153705535855226);
            }
            if check_Tok_7874756943448743542(self) {
                return Some(Token::Tok_7874756943448743542);
            }
            if check_Tok_13536687847573022133(self) {
                return Some(Token::Tok_13536687847573022133);
            }
            if check_Tok_4104316355815137153(self) {
                return Some(Token::Tok_4104316355815137153);
            }
            None
        }
    }
    fn parse_atom(parser: &mut ParserState) -> Option<i32> {
        match parser.token() {
            Some(Token::Tok_15975982353842843148) => {
                let __ = parse_Tok_15975982353842843148(parser)?;
                let value = parse_expr(parser)?;
                let __ = parse_Tok_13743468659553110316(parser)?;
                Some({ value })
            }
            Some(Token::Tok_4104316355815137153) => {
                let n = parse_Tok_4104316355815137153(parser)?;
                Some({ i32::from_str(n).unwrap() })
            }
            _ => None,
        }
    }
    fn parse_prod_cont(parser: &mut ParserState, acc: i32) -> Option<i32> {
        match parser.token() {
            Some(Token::Tok_15461786420412564008) => {
                let __ = parse_Tok_15461786420412564008(parser)?;
                let expr = parse_atom(parser)?;
                let cont = parse_prod_cont(parser, acc * expr)?;
                Some({ cont })
            }
            Some(Token::Tok_14924153705535855226) => {
                let __ = parse_Tok_14924153705535855226(parser)?;
                let expr = parse_atom(parser)?;
                let cont = parse_prod_cont(parser, acc / expr)?;
                Some({ cont })
            }
            _ => Some({ acc }),
            _ => None,
        }
    }
    fn parse_prod(parser: &mut ParserState) -> Option<i32> {
        match parser.token() {
            Some(Token::Tok_4104316355815137153) | Some(Token::Tok_15975982353842843148) => {
                let expr = parse_atom(parser)?;
                let cont = parse_prod_cont(parser, expr)?;
                Some({ cont })
            }
            _ => None,
        }
    }
    fn parse_expr_cont(parser: &mut ParserState, acc: i32) -> Option<i32> {
        match parser.token() {
            Some(Token::Tok_7874756943448743542) => {
                let __ = parse_Tok_7874756943448743542(parser)?;
                let expr = parse_prod(parser)?;
                let cont = parse_expr_cont(parser, acc + expr)?;
                Some({ cont })
            }
            Some(Token::Tok_13536687847573022133) => {
                let __ = parse_Tok_13536687847573022133(parser)?;
                let expr = parse_prod(parser)?;
                let cont = parse_expr_cont(parser, acc - expr)?;
                Some({ cont })
            }
            _ => Some({ acc }),
            _ => None,
        }
    }
    pub fn parse_expr(parser: &mut ParserState) -> Option<i32> {
        match parser.token() {
            Some(Token::Tok_4104316355815137153) | Some(Token::Tok_15975982353842843148) => {
                let expr = parse_prod(parser)?;
                let cont = parse_expr_cont(parser, expr)?;
                Some({ cont })
            }
            _ => None,
        }
    }
}

fn main() {
    let source = "10 + 20 * (10 * (20 - 10)) - 50";
    let mut parser = parser::ParserState::new(source);
    println!("{:?}", parser::parse_expr(&mut parser));
}
