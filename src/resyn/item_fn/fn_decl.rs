pub mod return_type;
pub use return_type::ReturnType;

// similar to `syn::FunDecl`
#[derive(Clone)]
pub struct FnDecl {
    pub fn_token: syn::token::Fn,
    pub generics: syn::Generics,
    pub paren_token: syn::token::Paren,
    pub inputs: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
    pub variadic: Option<syn::token::Dot3>,
    pub output: ReturnType,
}
