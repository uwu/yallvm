use yallvm_macros::Ast;

use crate::{exprs::Expr, stmts::BlockStmt, Ident, Span, Type};

#[derive(Clone, Ast)]
pub enum FuncBody {
	Expr(Box<Expr>),
	Block(Box<BlockStmt>),
	/// used for abstract methods and constructors
	EmptyBody,
}

#[derive(Clone, Ast)]
pub struct FuncCommon {
	pub span: Span,
	pub body: FuncBody,
	pub params: Vec<FuncParam>,
	pub rettype: Type,
}

#[derive(Clone, Ast)]
pub struct FuncParam {
	pub span: Span,
	pub type_: Type,
	pub name: Ident,
	pub default: Option<Box<Expr>>,
	/// useful in constructors
	pub prefix: Option<FuncParamPrefix>,
}

#[derive(Clone, Copy, Ast)]
pub enum FuncParamPrefix {
	Super,
	This,
}
