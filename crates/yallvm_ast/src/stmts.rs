use crate::{exprs::Expr, Span, Ident, TypeName, Op, funcs::{FuncCommon, TypeGeneric}};
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
	pub superclass: Option<TypeName>,
	pub interfaces: Vec<TypeName>,
	pub body: Vec<Member>,
	pub abstract_: bool,
	pub mixins: Vec<TypeName>,
	// something something this should be a separate node type something
	pub mixin: bool,
	pub generics: Vec<TypeGeneric>
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
	pub type_: TypeName,
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
	pub func: FuncCommon,
	pub init_list: Option<ConstructorInitList>
}

#[derive(Clone, Ast)]
pub enum ConstructorInitList {
	InitList/* (Vec<Box<DeclStmt>>) */,
	Assertion(Box<Expr>),
	Redirect(ConstructorRedirect)
}

#[derive(Clone, Ast)]
pub struct ConstructorRedirect {
	pub span: Span,
	pub super_: bool,
	pub name: Option<Ident>,
	pub params: Vec<Box<Expr>>
}

#[derive(Clone, Ast)]
pub struct MethodMember {
	pub span: Span,
	pub name: Ident,
	pub override_: bool,
	pub func: FuncCommon,
}

#[derive(Clone, Ast)]
pub struct OperatorMember {
	pub span: Span,
	pub op: Op,
	pub override_: bool,
	pub func: FuncCommon,
}