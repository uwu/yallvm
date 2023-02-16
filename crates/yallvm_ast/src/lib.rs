use stmts::Stmt;
use yallvm_macros::Ast;

pub mod stmts;
pub mod exprs;
pub mod traits;

#[derive(Default, Clone)]
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

#[derive(Clone, Ast)]
pub struct Program {
	pub span: Span,
	pub stmts: Vec<Box<Stmt>>
}