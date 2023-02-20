extern crate proc_macro;

use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

const ENUMS: [(&str, &str); 3] = [("Stmt", "stmts"), ("Expr", "exprs"), ("Member", "classes")];

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
		#[automatically_derived]
		impl crate::traits::AstToBox for #name {}
	};

	// implement enums and add onto result
	let name_str = name.to_string();

	for (enum_name, submod_name) in ENUMS {
		if !name_str.ends_with(enum_name) {
			continue;
		}

		let trimmed_name = remove_last_chars(enum_name.len(), &name_str);
		let trimmed_name = match trimmed_name {
			Some(s) => s,
			None => continue,
		};

		if trimmed_name.is_empty() {
			continue;
		}

		let trimmed_name = Ident::new(&trimmed_name.as_str(), name.span());

		let enum_name = Ident::new(enum_name, Span::call_site().into());
		let submod_name = Ident::new(submod_name, Span::call_site().into());

		result.extend(quote! {
			#[automatically_derived]
			impl crate::traits::AstToEnum<crate::#submod_name::#enum_name> for #name {
				fn to_enum(self) -> crate::#submod_name::#enum_name {
					crate::#submod_name::#enum_name::#trimmed_name(self)
				}
			}
		});
	}

	result.into()
}
