#![feature(proc_macro_hygiene)]

//! # Named Return
//!
//! Declares a proc-macro that enables return types to be named.  
//! Mostly re-defining structures from `syn` for parsing.
//!
//! The macro also:
//! 1. Declares the variables that were named as a prefix statement to the
//! original function's body.
//! 2. Requires that the return syntax is similar to the input parameter
//! syntax.
//!     - Requires parentheses.
//!
//! ## Example
//!
//! ```rust
//! #![feature(proc_macro_hygiene)]
//! # use named_return::named_return;
//! #
//! # #[derive(Debug, PartialEq, Eq)]
//! # pub struct A;
//! # #[derive(Debug, PartialEq, Eq)]
//! # pub struct B;
//! #
//!
//! named_return!(
//! fn f() -> (a: A, b: B) {
//!     a = A;
//!     b = B;
//!     (a, b)
//! });
//!
//! assert_eq!(f(), (A, B));
//! ```
//!
//! ## Note
//!
//! The intended syntax were to be used with a proc-macro-attr, such as:
//!
//! ```rust,ignore
//! #[named_return]
//! fn f() -> (a: A, b: B) {
//!     a = A;
//!     b = B;
//!     (a, b)
//! }
//! ```
//!
//! But it seems that Rust parses the original function syntax before
//! executing the proc-macro-attr and so it refuses the invalid syntax.
//!
//! This is a draft and is based on this suggestion:
//! https://github.com/rust-lang/rfcs/issues/2638
extern crate proc_macro;
extern crate proc_macro2;

mod resyn;

use proc_macro::TokenStream;
use quote::quote;
use syn;

/// Changes the function syntax so that return values may be named.
///
/// Also declares the variables that were named as a prefix statement to the
/// original function's body.
#[proc_macro]
pub fn named_return(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as resyn::item_fn::ItemFn);

    // replicates the input function (after the custom ItemFn parsing)
    // let replica = quote! {
    //    #input
    // };

    // extract parsed information
    let vis = &input.vis;
    let constness = &input.constness;
    let unsafety = &input.unsafety;
    let ident = &input.ident;
    let decl = &input.decl;
    let block = &input.block;
    // -------
    // TODO: also deal with those information:
    // let attrs = &input.attrs;
    // let asyncness = &input.asyncness;
    // let abi = &input.abi;

    let generics = &decl.generics;
    let inputs = &decl.inputs;
    let _output = &decl.output;
    let output_simple: syn::ReturnType = decl.output.clone().into();

    type Cp<T> = syn::punctuated::Punctuated<T, syn::token::Comma>;
    use resyn::item_fn::fn_decl::return_type::Captures;
    let captures: Option<Captures> = decl.output.clone().into();
    let (cap_names, cap_types): (Option<Cp<syn::Pat>>, Option<Cp<syn::Type>>) = match captures {
        Some(ref caps) => {
            use syn::punctuated::Pair;
            let (names, types) = caps
                .iter()
                .map(|arg_cap| {
                    let name = Pair::new(
                        arg_cap.pat.clone(),
                        Some(syn::token::Comma {
                            spans: arg_cap.colon_token.spans,
                        }),
                    );
                    let ty = Pair::new(
                        arg_cap.ty.clone(),
                        Some(syn::token::Comma {
                            spans: arg_cap.colon_token.spans,
                        }),
                    );
                    (name, ty)
                })
                .unzip();
            (Some(names), Some(types))
        }
        None => (None, None),
    };
    let block_prefix = if captures.is_some() {
        Some(quote!(let (#cap_names) : (#cap_types) ;))
    } else {
        None
    };
    let new_block = if block_prefix.is_some() {
        quote!({
            #block_prefix
            #block
        })
    } else {
        quote!(#block)
    };
    // -------
    // TODO: also deal with this information
    // let variadic = &decl.variadic;

    // creates a loosened wrapper for the replica
    // such that the loosened has only one parameter, a tuple,
    // and flatten that tuple as parameters for the replica call
    let reparsed = quote! {
        #vis #constness #unsafety fn #ident #generics ( #inputs ) #output_simple #new_block
    };
    // ------
    // TODO: test on signature-only functions

    // println!("\n ------> ident: {}", ident.to_string());
    // println!(" ==> <  {}  >", &replica);
    // println!(" ==> <  {}  >\n", &reparsed);

    let tokens = quote! {
        #reparsed
    };

    tokens.into()
}

/// Does not works because the original function syntax is parsed before the
/// proc-macro-attr.
#[proc_macro_attribute]
pub fn named_return_attr(_attr: TokenStream, item: TokenStream) -> TokenStream {
    named_return(item)
}
