#![allow(unknown_lints)]
#![allow(clippy::all)]
#![allow(dead_code)]

use lalrpop_util::lalrpop_mod;
pub use parser::*;

lalrpop_mod!(pub parser, "/parser/parser.rs");
