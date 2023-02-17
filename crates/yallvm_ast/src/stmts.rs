use crate::{exprs::Expr, Span, Ident, Type, Op};
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

#[derive(Clone, Ast)]
pub struct ClassStmt {
	pub span: Span,
	pub name: Ident,
	pub superclass: Option<Type>,
	pub interfaces: Vec<Type>,
	pub body: Vec<Member>,
	pub abstract_: bool,
}

#[derive(Clone, Ast)]
/// nodes that are members of a class
pub enum Member {
	InstVar(InstVarMember),
	Constructor(ConstructorMember),
	Method(MethodMember),
	Operator(OperatorMember),
}

#[derive(Clone, Ast)]
pub struct InstVarMember {
	pub span: Span,
	pub name: Ident,
	pub init: Option<Box<Expr>>,
	pub type_: Type,
	pub optional: bool,
	pub final_: bool,
	pub late: bool,
	pub static_: bool
}

#[derive(Clone, Ast)]
pub struct ConstructorMember {
	pub span: Span,
	pub name: Option<Ident>,
	pub static_: bool,
	pub const_: bool,
	pub factory: bool,
	// TODO: finish constructor node
}

#[derive(Clone, Ast)]
pub struct MethodMember {
	pub span: Span,
	pub name: Ident,
	pub override_: bool,
	// TODO: finish method node
}

#[derive(Clone, Ast)]
pub struct OperatorMember {
	pub span: Span,
	pub op: Op,
	pub override_: bool,
	// TODO: finish operator node
}