extern crate proc_macro;

use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

const ENUMS: &[(&str, &str)] = &[
	("Stmt", "stmts"),
	("Expr", "exprs"),
	("Member", "classes"),
	("CollectionItem", "exprs"),
];

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

	// get the name of the struct we are deriving on
	let name_ident = (parse_macro_input!(input as DeriveInput)).ident;
	let name_str = name_ident.to_string();

	// implement relevant enum
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

		let trimmed_name = Ident::new(&trimmed_name.as_str(), name_ident.span());

		let enum_name = Ident::new(enum_name, Span::call_site().into());
		let submod_name = Ident::new(submod_name, Span::call_site().into());

		return quote! {
			#[automatically_derived]
			impl From<#name_ident> for crate::#submod_name::#enum_name {
				#[inline(always)]
				fn from(item: #name_ident) -> crate::#submod_name::#enum_name {
					crate::#submod_name::#enum_name::#trimmed_name(item)
				}
			}

			#[automatically_derived]
			impl From<#name_ident> for Box<crate::#submod_name::#enum_name> {
				#[inline(always)]
				fn from(item: #name_ident) -> Box<crate::#submod_name::#enum_name> {
					Box::new(item.into())
				}
			}
		}
		.into();
	}

	// we didn't implement any enums, so return empty
	TokenStream::default()
}
