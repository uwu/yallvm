use stmts::Stmt;
use yallvm_macros::Ast;

pub mod exprs;
pub mod stmts;
pub mod funcs;
pub mod classes;

#[derive(Debug, Default, Clone)]
/// The purpose of the `Span` is to keep track of where in the input file the node is,
/// for compiler usage when outputting helpful information to the user.
pub struct Span {
	/// the byte of the input file that this ast node begins on
	pub abs_start: usize,
	/// the byte of the input file that this ast node ends on
	pub abs_end: usize,
	/// the starting line number of this ast node
	pub line_num: usize,
	/// the starting byte on this line
	pub line_start: usize,
	/// the ending byte on this line
	pub line_end: usize,
}

#[derive(Debug, Clone, Ast)]
pub struct Program {
	pub span: Span,
	pub stmts: Vec<Box<Stmt>>,
}

#[derive(Debug, Clone, Ast)]
pub struct Ident {
	pub span: Span,
	pub value: String,
}

// TODO: types will not be idents
pub type TypeName = Ident;

#[derive(Debug, Clone, Copy, Ast)]
pub enum BinOp {
	Lt,
	Gt,
	Lte,
	Gte,
	Sub,
	Add,
	Div,
	IntDiv,
	Mul,
	Mod,
	Or,
	Xor,
	And,
	LShf,
	RShf,
	RShff,
	Idx,
	IdxSet,
	Eq,
}

#[derive(Debug, Clone, Copy, Ast)]
pub enum UnaryOp {
	Pos,
	Neg,
	Complement
}