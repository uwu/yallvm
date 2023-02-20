use crate::{exprs::Expr, Span, classes::ClassStmt, TypeName, Ident};
use yallvm_macros::Ast;

#[derive(Debug, Clone, Ast)]
pub enum Stmt {
	Expr(ExprStmt),
	Class(ClassStmt),
	Block(BlockStmt),
	Decl(DeclStmt)
}

#[derive(Debug, Clone, Ast)]
pub struct ExprStmt {
	pub span: Span,
	pub expr: Box<Expr>,
}

#[derive(Debug, Clone, Ast)]
pub struct BlockStmt {
	pub span: Span,
	pub stmts: Vec<Box<Stmt>>
}

#[derive(Debug, Clone, Ast)]
pub struct DeclStmt {
	pub span: Span,
	pub type_: Option<TypeName>,
	pub value_: Option<Box<Expr>>,
	pub name: Ident,
	pub nullable: bool,
	pub final_: bool,
	pub late: bool,
	pub const_: bool
}