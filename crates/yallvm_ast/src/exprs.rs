use yallvm_macros::Ast;

use crate::{stmts::Ident, Span};

#[derive(Clone)]
pub enum Expr {
	StrLit(StrLitExpr),
	IntLit(IntLitExpr),
	FloatLit(FloatLitExpr),
	BoolLit(BoolLitExpr),

	// Expr members that aren't defined in this file (idents and the like)
	Ident(Ident),
}

#[derive(Clone, Ast)]
pub struct StrLitExpr {
	pub span: Span,
	pub value: String,
}

#[derive(Clone, Ast)]
pub struct IntLitExpr {
	pub span: Span,
	pub value: usize,
}

#[derive(Clone, Ast)]
pub struct FloatLitExpr {
	pub span: Span,
	pub value: f64,
}

#[derive(Clone, Ast)]
pub struct BoolLitExpr {
	pub span: Span,
	pub value: bool,
}
