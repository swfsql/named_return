pub mod fn_arg_ret;
use fn_arg_ret::FnArgRet;

pub type Outputs = syn::punctuated::Punctuated<FnArgRet, syn::token::Comma>;

// similar to `syn::FnDecl`
#[derive(Clone)]
pub struct Named {
    pub paren_token: syn::token::Paren,
    pub outputs: Outputs,
}

// similar to `impl Parse for ItemFn` from `syn::item.rs`
impl Named {
    pub fn parse(input: syn::parse::ParseStream, _allow_plus: bool) -> syn::Result<Self> {
        use syn::parse::Parse;
        // TODO: verify if _allow_plus is really not needed.
        let content;
        let paren_token = syn::parenthesized!(content in input);
        let outputs = content.parse_terminated(FnArgRet::parse)?;
        Ok(Named {
            paren_token,
            outputs,
        })
    }
}

// similar to `impl ReturnType::parse` from `syn::ty.rs`
impl syn::parse::Parse for Named {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        Self::parse(input, true)
    }
}
