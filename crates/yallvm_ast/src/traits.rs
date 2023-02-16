use crate::{exprs::Expr, stmts::Stmt};

pub trait AstNode where Self : Sized {
	fn to_box(self) -> Box<Self> {
    Box::new(self)
  }
}

pub trait StmtNode {
	fn to_enum(self) -> Stmt;
}

pub trait ExprNode {
	fn to_enum(self) -> Expr;
}
