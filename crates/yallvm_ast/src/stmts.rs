use crate::{exprs::Expr, Span, classes::ClassStmt};
use yallvm_macros::Ast;

#[derive(Clone, Ast)]
pub enum Stmt {
	Expr(ExprStmt),
	Class(ClassStmt),
	Block(BlockStmt)
}

#[derive(Clone, Ast)]
pub struct ExprStmt {
	pub span: Span,
	pub expr: Box<Expr>,
}

#[derive(Clone, Ast)]
pub struct BlockStmt {
	pub span: Span,
	pub stmts: Vec<Box<Stmt>>
}