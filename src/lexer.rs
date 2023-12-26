use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

#[derive(Debug, Clone)]
pub enum TokDesc {
    Token(String),
    Regex(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Term(pub String);

impl Term {
    pub fn token_name(&self) -> String {
        let hash = {
            let mut hasher = DefaultHasher::new();
            self.0.hash(&mut hasher);
            hasher.finish()
        };

        format!("Tok_{hash}")
    }

    pub fn check_fn(&self) -> Ident {
        Ident::new(&format!("check_{}", self.token_name()), Span::call_site())
    }

    pub fn parse_fn(&self) -> Ident {
        Ident::new(&format!("parse_{}", self.token_name()), Span::call_site())
    }

    pub fn re_name(&self) -> Ident {
        Ident::new(&format!("RE_{}", self.token_name()), Span::call_site())
    }
}

impl ToTokens for Term {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = Ident::new(&self.token_name(), Span::call_site());
        ident.to_tokens(tokens);
    }
}

#[derive(Debug, Clone)]
pub struct Tokens {
    pub mapping: Vec<(Term, TokDesc)>,
}

impl Tokens {
    fn terms(&self) -> impl Iterator<Item = &Term> {
        self.mapping.iter().map(|desc| &desc.0)
    }

    fn token_definition(&self) -> TokenStream {
        let decls = self.terms();
        let names = self.terms();
        let vals = self.terms().map(|term| &term.0);

        quote! {
            #[derive(Debug)]
            pub enum Token {
                #(#decls),*
            }

            impl fmt::Display for Token {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    match self {
                        #(Token::#names => write!(f, #vals)),*
                    }
                }
            }
        }
    }

    fn token_parsers(&self) -> TokenStream {
        let res = self.mapping.iter().filter_map(|(tok, tok_def)| {
            let re_name = tok.re_name();

            if let TokDesc::Regex(re) = tok_def {
                Some(quote! {
                    static ref #re_name: Regex = Regex::new(#re).unwrap();
                })
            } else {
                None
            }
        });

        let fns = self.mapping.iter().map(|(tok, tok_def)| {
            let re_name = tok.re_name();
            let parse_fn = tok.parse_fn();
            let parse_body = match tok_def {
                TokDesc::Token(str) => quote! { parser.expect(#str) },
                TokDesc::Regex(_) => quote! { parser.expect_re(&#re_name) },
            };

            let check_fn = tok.check_fn();
            let check_body = match tok_def {
                TokDesc::Token(str) => quote! { parser.is_prefix(#str) },
                TokDesc::Regex(_) => quote! { parser.is_prefix_re(&#re_name) },
            };

            quote! {
                fn #parse_fn<'a>(parser: &mut ParserState<'a>) -> Result<&'a str, ParseError<Token>> {
                    if let Some(res) = #parse_body {
                        Ok(res)
                    } else {
                        Err(ParseError::UnexpectedToken {
                            actual: parser.token(),
                            expected: Token::#tok
                        })
                    }   
                }

                fn #check_fn(parser: &mut ParserState) -> bool {
                    #check_body
                }
            }
        });

        quote! {
            lazy_static! {
                #(#res)*
            }

            #(#fns)*
        }
    }

    fn token_method(&self) -> TokenStream {
        let branches = self.terms().map(|tok| {
            let check_fn = tok.check_fn();

            quote! {
                if #check_fn(self) {
                    return Some(Token::#tok)
                }
            }
        });

        quote! {
            impl ParserState<'_> {
                fn token(&mut self) -> Option<Token> {
                    #(#branches)*

                    None
                }
            }
        }
    }

    pub fn generate(&self) -> TokenStream {
        [
            self.token_definition(),
            self.token_parsers(),
            self.token_method(),
        ]
        .into_iter()
        .collect()
    }
}
