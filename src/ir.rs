use std::boxed::Box;
use std::collections::HashMap;
use std::option::Option;
use std::vec::Vec;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Variable {
    DeBruijn(i32),
    Global(String),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Type {
    Bool,
    Int,
    Type,
    FakeLin,
    Arrow(Box<Type>, Box<Type>),
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Builtin {
    Plus,
    Times,
    Minus,
    Div,
    Arrow,
    Eq,
    Neq,
    Lt,
    Lteq,
    Gt,
    Gteq,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Value {
    Bool(bool),
    Int(i32),
    Type(Type),
    Builtin {
        captured: Vec<Value>,
        remaining: i32,
        body: Builtin,
    },
    Fun {
        captured: Vec<Value>,
        remaining: i32,
        body: Box<TExpr>,
    },
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TValue {
    pub val: Value,
    pub typ: Type,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ExprT<Typ, Expr> {
    Glob(String),
    Value(TValue),
    Let {
        name: String,
        val: Expr,
        vartyp: Option<Typ>,
        body: Expr,
    },
    Ite(Expr, Expr, Expr),
    Call(Expr, Expr),
    Builtin(Builtin),
    Lambda {
        name: String,
        argtyp: Typ,
        body: Expr,
    },
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct SourcePos {}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Expr {
    pub source: SourcePos,
    pub expr: ExprT<Box<Expr>, Box<Expr>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TExpr {
    pub typ: Type,
    pub expr: ExprT<Type, Box<TExpr>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Declaration {
    Definition {
        name: String,
        typ: Expr,
        args: Vec<String>,
        body: Expr,
    },
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Program(HashMap<String, Declaration>);

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TProgram(HashMap<String, TValue>);
