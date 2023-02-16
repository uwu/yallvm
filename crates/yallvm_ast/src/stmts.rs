use yallvm_macros::Ast;
use crate::Span;

#[derive(Clone)]
pub enum Stmt {
	Expr(ExprStmt)
}

#[derive(Clone, Ast)]
pub struct ExprStmt {
	pub span: Span,
}
