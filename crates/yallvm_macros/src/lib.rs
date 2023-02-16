extern crate proc_macro;

use proc_macro::{TokenStream, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

fn remove_last_chars(amt: usize, s: &String) -> Option<String> {
	let len = s.chars().count();
	let mut s = s.clone();
	if amt > len {
		None
	} else {
		s.drain(len - amt..len);
		Some(s)
	}
}

#[proc_macro_derive(Ast)]
pub fn derive_ast_node(input: TokenStream) -> TokenStream {
	//! implements relevant ast node traits for the struct

	// parse rust code lmao
	let input = parse_macro_input!(input as DeriveInput);

	// get the name of the struct as an ident
	let name = input.ident;

	// construct an AstNode implementation
	let mut result = quote! {
		impl crate::traits::AstNode for #name {}
	};

	// trim off Stmt / Expr
	let name_str = name.to_string();

	let trimmed_name = remove_last_chars(4, &name_str);

	let trimmed_name = match trimmed_name {
		Some(s) => s,
		None => return result.into(),
	};

	let trimmed_name = Ident::new(&trimmed_name.as_str(), name.span());

	// add StmtNode / ExprNode implementations
	if name_str.ends_with("Stmt") || name_str.ends_with("Expr") {
		let enum_name: String = if name_str.ends_with("Stmt") { "Stmt".into() } else { "Expr".into() };
		let trait_name = enum_name.clone() + "Node";

		let enum_name = Ident::new(enum_name.as_str(), Span::call_site().into());
		let trait_name = Ident::new(trait_name.as_str(), Span::call_site().into());

		result.extend(quote! {
			impl crate::traits::#trait_name for #name {
				fn to_enum(self) -> crate::stmts::Stmt {
					crate::stmts::#enum_name::#trimmed_name(self)
				}
			}
		})
	}

	result.into()
}
