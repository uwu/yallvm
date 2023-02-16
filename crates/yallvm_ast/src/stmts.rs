use crate::{exprs::Expr, Span};
use yallvm_macros::Ast;

#[derive(Clone)]
pub enum Stmt {
	Expr(ExprStmt),
}

#[derive(Clone, Ast)]
pub struct ExprStmt {
	pub span: Span,
	pub expr: Box<Expr>,
}

#[derive(Clone, Ast)]
pub struct Ident {
	pub span: Span,
	pub value: String,
}