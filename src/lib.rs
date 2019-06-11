extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index,
};

#[proc_macro_derive(GraphLeaf)]
pub fn derive_graph_leaf(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics GraphExec for #name #ty_generics #where_clause {
            fn exec(&mut self, context: &mut dyn SchedContext, children: &mut dyn ChildExec) -> bool {
                self.exec_leaf(context);
                true
            }

            #[cfg(feature = "std")]
            fn children_max(&self) -> ChildCount {
                ChildCount::None
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(GraphNode)]
pub fn derive_graph_node(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics GraphExec for #name #ty_generics #where_clause {
            fn exec(&mut self, context: &mut dyn SchedContext, children: &mut dyn ChildExec) -> bool {
                self.exec_node(context, children);
                true
            }

            #[cfg(feature = "std")]
            fn children_max(&self) -> ChildCount {
                ChildCount::Inf
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}
