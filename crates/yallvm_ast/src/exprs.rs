use yallvm_macros::Ast;

use crate::{Ident, Span, TypeName, BinOp, UnaryOp};

#[derive(Debug, Clone, Ast)]
pub enum Expr {
	StrLit(StrLitExpr),
	IntLit(IntLitExpr),
	HexLit(HexLitExpr),
	FloatLit(FloatLitExpr),
	BoolLit(BoolLitExpr),
	Assignment(AssignmentExpr),
	Call(CallExpr),
	Binary(BinaryExpr),
	Unary(UnaryExpr),

	// Expr members that aren't defined in this file (idents and the like)
	Ident(Ident),
	TypeName(TypeName)
}

#[derive(Debug, Clone, Ast)]
pub struct StrLitExpr {
	pub span: Span,
	pub value: Vec<StrLitPart>,
}

#[derive(Debug, Clone, Ast)]
pub enum StrLitPart {
	Str(String),
	Interpolation(Box<Expr>)
}

#[derive(Debug, Clone, Ast)]
pub struct IntLitExpr {
	pub span: Span,
	pub value: i64,
}

#[derive(Debug, Clone, Ast)]
pub struct HexLitExpr {
	pub span: Span,
	pub value: String,
}

impl HexLitExpr {
	pub fn resolve(&self) -> i64 {
		i64::from_str_radix(&self.value, 16).unwrap()
	}
}

#[derive(Debug, Clone, Ast)]
pub struct FloatLitExpr {
	pub span: Span,
	pub value: f64,
	pub exponent: Option<i32>
}

impl FloatLitExpr {
	pub fn resolve(&self) -> f64 {
		if let Some(exp) = self.exponent {
			self.value * (10_f64.powi(exp))
		}
		else {
			self.value
		}
	}
}

#[derive(Debug, Clone, Ast)]
pub struct BoolLitExpr {
	pub span: Span,
	pub value: bool,
}

#[derive(Debug, Clone, Ast)]
pub struct AssignmentExpr {
	pub span: Span,
	pub target: Ident, // TODO member exprs etc
	pub value: Box<Expr>,
	pub compound: Option<BinOp>
}

#[derive(Debug, Clone, Ast)]
pub struct CallExpr {
	pub span: Span,
	pub target: Box<Expr>,
	pub params: Vec<Box<Expr>>
}

#[derive(Debug, Clone, Ast)]
pub struct BinaryExpr {
	pub span: Span,
	pub left: Box<Expr>,
	pub right: Box<Expr>,
	pub op: BinOp
}

#[derive(Debug, Clone, Ast)]
pub struct UnaryExpr {
	pub span: Span,
	pub target: Box<Expr>,
	pub op: UnaryOp
}