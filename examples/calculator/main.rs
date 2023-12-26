include!("calc_parser.rs");

fn main() {
    for arg in std::env::args().skip(1) {
        let mut parser = parser::ParserState::new(&arg);
        match parser::parse_expr(&mut parser) {
            Ok(v) => println!("{v}"),
            Err(e) => println!("failed: {e}"),
        }
    }
}
