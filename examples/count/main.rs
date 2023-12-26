include!("count_parser.rs");

fn main() {
    for arg in std::env::args().skip(1) {
        let mut parser = parser::ParserState::new(&arg);
        println!("{:?}", parser::parse_expr(&mut parser));
    }
}
