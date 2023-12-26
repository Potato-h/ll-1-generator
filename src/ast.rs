use std::{
    collections::{HashMap, HashSet},
    fmt,
    hash::Hash,
    str::FromStr,
};

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::lexer::{Term, Tokens};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NonTerm(pub String);

#[derive(Debug, Clone)]
pub enum Node {
    NonTerm {
        node: NonTerm,
        extract_name: Option<String>,
        args: Option<String>,
    },
    Term(Term, Option<String>),
}

pub struct Display<T>(pub T);

impl fmt::Display for Display<&'_ [Node]> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in self.0 {
            match node {
                Node::NonTerm { node, .. } => write!(f, "{} ", node.0)?,
                Node::Term(term, _) => write!(f, "\"{} \"", term.0)?,
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub nodes: Vec<Node>,
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct NonTermDef {
    pub name: NonTerm,
    pub args: Option<String>,
    pub ret_ty: String,
    pub rules: Vec<Rule>,
    pub is_pub: bool,
}

#[derive(Debug, Clone)]
pub struct Grammar {
    pub nonterms: Vec<NonTermDef>,
}

type FirstSet = HashSet<Option<Term>>;
type FirstMap = HashMap<NonTerm, FirstSet>;

fn first(nodes: &[Node], fst: &FirstMap) -> FirstSet {
    match nodes.get(0) {
        Some(Node::Term(tok, _)) => HashSet::from([Some(tok.clone())]),
        Some(Node::NonTerm { node: nt, .. }) => {
            let mut pref = fst[nt].clone();

            if pref.contains(&None) {
                pref.remove(&None);
                pref.union(&first(&nodes[1..], fst))
                    .map(Clone::clone)
                    .collect()
            } else {
                pref
            }
        }
        None => HashSet::from([None]),
    }
}

type FollowSet = HashSet<Option<Term>>;
type FollowMap = HashMap<NonTerm, FollowSet>;

fn code_or_empty(code: &Option<String>) -> TokenStream {
    if let Some(code) = code {
        TokenStream::from_str(code).unwrap()
    } else {
        quote! {}
    }
}

impl Grammar {
    fn build_first(&self) -> FirstMap {
        let mut changed = true;
        let mut fst: FirstMap = HashMap::new();

        while changed {
            changed = false;

            for def in self.nonterms.iter() {
                for rule in def.rules.iter() {
                    let new = first(&rule.nodes, &fst);

                    match fst.get_mut(&def.name) {
                        Some(val) => {
                            changed |= new.difference(val).count() > 0;
                            val.extend(new);
                        }
                        None => {
                            fst.insert(def.name.clone(), new);
                            changed = true;
                        }
                    }
                }
            }
        }

        fst
    }

    fn build_follow(&self, fst: &FirstMap) -> FollowMap {
        let mut changed = true;
        let mut flw: FollowMap = HashMap::new();

        for def in self.nonterms.iter() {
            if def.is_pub {
                flw.insert(def.name.clone(), HashSet::from([None]));
            }
        }

        while changed {
            changed = false;

            let pairs = self
                .nonterms
                .iter()
                .flat_map(|def| def.rules.iter().map(move |rule| (rule, def)))
                .flat_map(|(rule, def)| {
                    rule.nodes
                        .iter()
                        .enumerate()
                        .map(|(i, node)| (&def.name, node, &rule.nodes[i + 1..]))
                })
                .filter_map(|(name, node, rest)| match node {
                    Node::NonTerm { node, .. } => Some((name, node, rest)),
                    Node::Term(_, _) => None,
                });

            for (name, node, rest) in pairs {
                let fst = first(rest, fst);

                let to_add: FollowSet = if fst.contains(&None) {
                    flw.get(&name)
                        .into_iter()
                        .flatten()
                        .cloned()
                        .chain(fst.into_iter().filter(|nt| nt.is_some()))
                        .collect()
                } else {
                    fst.into_iter().filter(|nt| nt.is_some()).collect()
                };

                match flw.get_mut(node) {
                    Some(v) => {
                        changed |= to_add.difference(&v).count() > 0;
                        v.extend(to_add);
                    }
                    None => {
                        changed = true;
                        flw.insert(node.clone(), to_add);
                    }
                }
            }
        }

        flw
    }
}

fn get_fn_name(nt: &str) -> Ident {
    Ident::new(&format!("parse_{}", nt), Span::call_site())
}

impl NonTermDef {
    pub fn generate(&self, fst: &FirstMap, flw: &FollowMap) -> TokenStream {
        let fn_name = get_fn_name(&self.name.0);
        let ret_ty = TokenStream::from_str(&self.ret_ty).unwrap();
        let args = code_or_empty(&self.args);

        let get_name = |name: &Option<String>| {
            Ident::new(
                name.as_ref().map(|x| &x[..]).unwrap_or("__"),
                Span::call_site(),
            )
        };

        let branches = self.rules.iter().map(|rule| {
            let first_terms = first(&rule.nodes, fst);
            let mut terms: Vec<_> = first_terms
                .clone()
                .into_iter()
                .filter(|nt| nt.is_some())
                .collect();

            if first_terms.contains(&None) {
                terms.extend(flw.get(&self.name).into_iter().flatten().cloned());
            }

            // eprintln!("{:?} -> {:?}, marker: {:?}", self.name, rule, terms);

            let terms = terms.into_iter().map(|term| match term {
                Some(nt) => quote! { Some(Token::#nt) },
                None => quote! { None },
            });

            let code = TokenStream::from_str(&rule.code).unwrap();
            let subparses = rule.nodes.iter().map(|node| match node {
                Node::NonTerm {
                    node: nt,
                    extract_name,
                    args,
                } => {
                    let name = get_name(extract_name);
                    let nt = get_fn_name(&nt.0);
                    let args = code_or_empty(args);
                    quote! { let #name = #nt (parser, #args)?; }
                }
                Node::Term(t, name) => {
                    let name = get_name(name);
                    let t = get_fn_name(&t.token_name());
                    quote! { let #name = #t (parser)?; }
                }
            });

            quote! {
                #(#terms)|* => {
                    #(#subparses)*
                    Some({ #code })
                },
            }
        });

        // let eps_branches = self
        //     .rules
        //     .iter()
        //     .filter(|rule| rule.nodes.is_empty())
        //     .map(|rule| {
        //         let code = TokenStream::from_str(&rule.code).unwrap();
        //         quote! {
        //             _ => { Some({ #code }) }
        //         }
        //     });

        let vis = if self.is_pub {
            quote! { pub }
        } else {
            quote! {}
        };

        quote! {
            #vis fn #fn_name (parser: &mut ParserState, #args) -> Option<#ret_ty> {
                match parser.token() {
                    #(#branches)*
                    _ => None,
                }
            }
        }
    }
}

impl Grammar {
    pub fn generate(&self) -> TokenStream {
        let fst = self.build_first();
        let flw = self.build_follow(&fst);

        // eprintln!("first: {fst:#?}");
        // eprintln!("follow: {flw:#?}");

        self.nonterms
            .iter()
            .map(|def| def.generate(&fst, &flw))
            .collect()
    }

    pub fn check_ll1(&self) -> Result<(), (NonTerm, &[Node], &[Node])> {
        let fst = self.build_first();
        let flw = self.build_follow(&fst);

        for def in self.nonterms.iter() {
            let pairs = def
                .rules
                .iter()
                .enumerate()
                .flat_map(|(i, r1)| def.rules[i + 1..].iter().map(move |r2| (r1, r2)));

            for (r1, r2) in pairs {
                let fst1 = first(&r1.nodes, &fst);
                let fst2 = first(&r2.nodes, &fst);
                let flw_def = {
                    let mut flw_a = flw.get(&def.name).unwrap().clone();
                    flw_a.remove(&None);
                    flw_a
                };

                if fst1.intersection(&fst2).count() != 0
                    || (fst1.contains(&None) && fst2.intersection(&flw_def).count() != 0)
                    || (fst2.contains(&None) && fst1.intersection(&flw_def).count() != 0)
                {
                    return Err((def.name.clone(), &r1.nodes[..], &r2.nodes[..]));
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Language {
    pub preamble: String,
    pub lexer: Tokens,
    pub grammar: Grammar,
}

impl Language {
    pub fn generate(&self) -> TokenStream {
        let inner: TokenStream = [
            TokenStream::from_str(include_str!("general.rs")).unwrap(),
            TokenStream::from_str(&self.preamble).unwrap(),
            self.lexer.generate(),
            self.grammar.generate(),
        ]
        .into_iter()
        .collect();

        quote! {
            mod parser {
                #![allow(non_camel_case_types)]
                #![allow(non_upper_case_globals)]
                #![allow(dead_code)]
                #![allow(non_snake_case)]
                #![allow(unused_braces)]
                #![allow(unreachable_patterns)]

                #inner
            }
        }
    }
}
