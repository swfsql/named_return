// subset from `syn::FnArg`
#[derive(Clone)]
pub enum FnArgRet {
    Captured(syn::ArgCaptured),
    Ignored(syn::Type),
}

// copy from `syn::item::parsing::arg_captured`
fn arg_captured(input: syn::parse::ParseStream) -> syn::parse::Result<syn::ArgCaptured> {
    Ok(syn::ArgCaptured {
        pat: input.parse()?,
        colon_token: input.parse()?,
        ty: match input.parse::<syn::Token![...]>() {
            Ok(dot3) => {
                let args = vec![
                    proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(
                        '.',
                        proc_macro2::Spacing::Joint,
                    )),
                    proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(
                        '.',
                        proc_macro2::Spacing::Joint,
                    )),
                    proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(
                        '.',
                        proc_macro2::Spacing::Alone,
                    )),
                ];
                use std::iter::FromIterator;
                let tokens = proc_macro2::TokenStream::from_iter(
                    args.into_iter().zip(&dot3.spans).map(|(mut arg, span)| {
                        arg.set_span(*span);
                        arg
                    }),
                );
                syn::Type::Verbatim(syn::TypeVerbatim { tts: tokens })
            }
            Err(_) => input.parse()?,
        },
    })
}

// similar to `impl Parse for FnArg` from `syn::item.rs`
impl syn::parse::Parse for FnArgRet {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let ahead = input.fork();
        let err = match ahead.call(arg_captured) {
            Ok(_) => return input.call(arg_captured).map(FnArgRet::Captured),
            Err(err) => err,
        };

        let ahead = input.fork();
        if ahead.parse::<syn::Type>().is_ok() {
            return input.parse().map(FnArgRet::Ignored);
        }

        Err(err)
    }
}
