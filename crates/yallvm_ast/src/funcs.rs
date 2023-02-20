use yallvm_macros::Ast;

use crate::{exprs::Expr, stmts::BlockStmt, Ident, Span, TypeName};

#[derive(Debug, Clone, Ast)]
pub enum FuncBody {
	Expr(Box<Expr>),
	Block(Box<BlockStmt>),
	/// used for abstract methods and constructors
	EmptyBody,
}

#[derive(Debug, Clone, Ast)]
pub struct FuncCommon {
	pub span: Span,
	pub body: FuncBody,
	pub params: Vec<FuncParam>,
	pub rettype: TypeName,
	pub generics: Vec<TypeGeneric>
}

#[derive(Debug, Clone, Ast)]
pub struct FuncParam {
	pub span: Span,
	pub type_: TypeName,
	pub name: Ident,
	pub default: Option<Box<Expr>>,
	/// useful in constructors
	pub prefix: Option<FuncParamPrefix>,
}

#[derive(Debug, Clone, Copy, Ast)]
pub enum FuncParamPrefix {
	Super,
	This,
}

#[derive(Debug, Clone, Ast)]
pub struct TypeGeneric {
	pub span: Span,
	pub name: Ident,
	pub superclassses: Vec<TypeName>
}