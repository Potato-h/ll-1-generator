use std::io;

use generator::{ast::Display, notation};
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar, "/notation/grammar.rs");

fn main() -> io::Result<()> {
    for arg in std::env::args().skip(1) {
        let source = std::fs::read_to_string(arg)?;
        let lexer = notation::lexer::Lexer::new(&source[..]);
        let parser = grammar::DescriptionParser::new();

        let ast = parser.parse(lexer).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to parse grammar: {e:?}"),
            )
        })?;

        if let Err((def, arm1, arm2)) = ast.grammar.check_ll1() {
            eprintln!("Check for ll(1) failed:");
            eprintln!("In rules for {} conflicted has founded:", def.0);
            eprintln!("Arm1: {}", Display(arm1));
            eprintln!("Arm2: {}", Display(arm2));
            return Err(io::Error::new(io::ErrorKind::Other, "Conflict in grammar"));
        }

        println!("{}", ast.generate());
    }

    Ok(())
}
