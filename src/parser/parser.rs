#![allow(unknown_lints)]
#![allow(clippy::all)]
#![allow(dead_code)]

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/parser/parser.rs");
