use crate::tree_visualizer::Paint;

mod tree_visualizer {
    use std::fmt::Display;

    #[derive(Debug, Default)]
    pub struct Canvas {
        nodes: Vec<String>,
        edges: Vec<(u64, u64)>,
    }

    pub trait Paint {
        fn paint_on(&self, canvas: &mut Canvas) -> u64;

        fn paint(&self) -> Canvas {
            let mut canvas = Canvas::new();
            self.paint_on(&mut canvas);
            canvas
        }
    }

    impl Canvas {
        pub fn new() -> Self {
            Default::default()
        }

        pub fn node(&mut self, label: &str) -> u64 {
            let id = self.nodes.len() as u64;
            self.nodes.push(String::from(label));
            id
        }

        pub fn edge(&mut self, from: u64, to: u64) {
            self.edges.push((from, to));
        }
    }

    impl Display for Canvas {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "graph G {{")?;

            for (id, node) in self.nodes.iter().enumerate() {
                writeln!(f, "\t{id} [label=\"{node}\"];")?;
            }

            for (from, to) in self.edges.iter() {
                writeln!(f, "\t{from} -- {to};")?;
            }

            writeln!(f, "}}")
        }
    }

    impl Paint for String {
        fn paint_on(&self, canvas: &mut Canvas) -> u64 {
            canvas.node("ident")
        }
    }
}

mod ast {
    use crate::tree_visualizer::Canvas;

    use super::tree_visualizer::Paint;

    #[derive(Debug, Clone)]
    pub struct Ty(pub String);

    #[derive(Debug, Clone)]
    pub struct Arg {
        pub name: String,
        pub ty: Ty,
    }

    #[derive(Debug, Clone)]
    pub struct Signature {
        pub name: String,
        pub args: Vec<Arg>,
        pub ret_ty: Option<Ty>,
    }

    impl Paint for Ty {
        fn paint_on(&self, canvas: &mut Canvas) -> u64 {
            let ident = self.0.paint_on(canvas);
            let id = canvas.node("Ty");
            canvas.edge(id, ident);
            id
        }
    }

    impl Paint for Arg {
        fn paint_on(&self, canvas: &mut Canvas) -> u64 {
            let arg = self.name.paint_on(canvas);
            let colon = canvas.node(":");
            let ty = self.ty.paint_on(canvas);
            let id = canvas.node("Arg");
            canvas.edge(id, arg);
            canvas.edge(id, colon);
            canvas.edge(id, ty);
            id
        }
    }

    impl Paint for Signature {
        fn paint_on(&self, canvas: &mut Canvas) -> u64 {
            let fun = canvas.node("fun");
            let name = self.name.paint_on(canvas);
            let lp = canvas.node("(");
            let args = {
                let args: Vec<_> = {
                    let mut args = Vec::new();

                    for (i, arg) in self.args.iter().enumerate() {
                        args.push(arg.paint_on(canvas));
                        if i + 1 != self.args.len() {
                            args.push(canvas.node(","));
                        }
                    }

                    args
                };

                let id = canvas.node("Args");

                for arg in args {
                    canvas.edge(id, arg);
                }

                id
            };
            let rp = canvas.node(")");
            let ret_ty = match &self.ret_ty {
                Some(ty) => {
                    let colon = canvas.node(":");
                    let ty = ty.paint_on(canvas);
                    let id = canvas.node("RetTy");
                    canvas.edge(id, colon);
                    canvas.edge(id, ty);
                    id
                }
                None => {
                    let eps = canvas.node("eps");
                    let id = canvas.node("RetTy");
                    canvas.edge(id, eps);
                    id
                }
            };

            let id = canvas.node("Sig");
            canvas.edge(id, fun);
            canvas.edge(id, name);
            canvas.edge(id, lp);
            canvas.edge(id, args);
            canvas.edge(id, rp);
            canvas.edge(id, ret_ty);
            id
        }
    }
}

include!("kotlin_parser.rs");

fn main() {
    let source = "fun foo(a:Int,b:Double):Double";
    let mut parser = parser::ParserState::new(source);
    if let Some(sig) = parser::parse_sig(&mut parser) {
        println!("{}", sig.paint());
    } else {
        eprintln!("Parse error");
    }
}
