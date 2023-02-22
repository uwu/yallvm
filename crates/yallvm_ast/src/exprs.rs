use yallvm_macros::Ast;

use crate::{BinOp, Ident, Span, TypeName, UnaryOp};

#[derive(Debug, Clone, Ast)]
pub enum Expr {
	StrLit(StrLitExpr),
	IntLit(IntLitExpr),
	HexLit(HexLitExpr),
	FloatLit(FloatLitExpr),
	BoolLit(BoolLitExpr),
	SymbolLit(SymbolLitExpr),
	CollectionLit(CollectionLitExpr),
	Assignment(AssignmentExpr),
	Call(CallExpr),
	Binary(BinaryExpr),
	Unary(UnaryExpr),

	// Expr members that aren't defined in this file (idents and the like)
	Ident(Ident),
	TypeName(TypeName),
}

#[derive(Debug, Clone, Ast)]
pub struct StrLitExpr {
	pub span: Span,
	pub value: Vec<StrLitPart>,
}

#[derive(Debug, Clone, Ast)]
pub enum StrLitPart {
	Str(String),
	Interpolation(Box<Expr>),
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
	pub exponent: Option<i32>,
}

impl FloatLitExpr {
	pub fn resolve(&self) -> f64 {
		if let Some(exp) = self.exponent {
			self.value * (10_f64.powi(exp))
		} else {
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
pub struct SymbolLitExpr {
	pub span: Span,
	pub value: Ident,
}

#[derive(Debug, Clone, Ast)]
pub struct CollectionLitExpr {
	pub span: Span,
	pub type_: Option<TypeName>,
	pub collection_type: CollectionLitType,
	pub elements: Vec<CollectionItem>,
}

#[derive(Debug, Clone, Copy, Ast)]
pub enum CollectionLitType {
	List,
	Set,
	Map,
}

#[derive(Debug, Clone, Ast)]
pub enum CollectionItem {
	/// not valid in maps
	Expr(Box<Expr>),
	/// only valid in maps
	Mapping(MappingCollectionItem),
	Spread(SpreadCollectionItem),
	If(IfCollectionItem),
	For(ForCollectionItem),
}

#[derive(Debug, Clone, Ast)]
pub struct MappingCollectionItem {
	pub span: Span,
	pub key: Box<Expr>,
	pub value: Box<Expr>,
}

#[derive(Debug, Clone, Ast)]
pub struct SpreadCollectionItem {
	pub span: Span,
	pub item: Box<Expr>,
	pub null_aware: bool,
}

#[derive(Debug, Clone, Ast)]
pub struct IfCollectionItem {
	pub span: Span,
	pub condition: Box<Expr>,
	// box else CollectionItem would be infinitely sized
	pub result: Box<CollectionItem>,
}

#[derive(Debug, Clone, Ast)]
pub struct ForCollectionItem {
	pub span: Span,
	// TODO: common enum for iteration stuff
	// box else CollectionItem would be infinitely sized
	pub result: Box<CollectionItem>,
}

#[derive(Debug, Clone, Ast)]
pub struct AssignmentExpr {
	pub span: Span,
	pub target: Ident, // TODO member exprs etc
	pub value: Box<Expr>,
	pub compound: Option<BinOp>,
}

#[derive(Debug, Clone, Ast)]
pub struct CallExpr {
	pub span: Span,
	pub target: Box<Expr>,
	pub params: Vec<Box<Expr>>,
}

#[derive(Debug, Clone, Ast)]
pub struct BinaryExpr {
	pub span: Span,
	pub left: Box<Expr>,
	pub right: Box<Expr>,
	pub op: BinOp,
}

#[derive(Debug, Clone, Ast)]
pub struct UnaryExpr {
	pub span: Span,
	pub target: Box<Expr>,
	pub op: UnaryOp,
}
