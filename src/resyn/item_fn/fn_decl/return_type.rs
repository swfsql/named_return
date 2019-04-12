pub mod named;
use named::Named;

pub type Captures = syn::punctuated::Punctuated<syn::ArgCaptured, syn::token::Comma>;

// similar to `syn::ReturnType`
#[derive(Clone)]
pub enum ReturnType {
    Default, // from `syn::ReturnType::Default`
    Named(syn::token::RArrow, Box<Named>),
    Type(syn::token::RArrow, Box<syn::Type>), // from `syn::ReturnType::Type`
                                              // ---
                                              // TODO: verify the tuple case, because instead of falling into
                                              // the Type case, it will fall into the Named case.
}

// similar to `syn::ty::ReturnType` impls
impl ReturnType {
    pub fn parse(input: syn::parse::ParseStream, allow_plus: bool) -> syn::parse::Result<Self> {
        if input.peek(syn::Token![->]) {
            let arrow = input.parse()?;
            // TODO: test empty-tuple case.
            // perhaps this is an exceptional case.
            if input.peek(syn::token::Paren) {
                // FnArg-like input
                // TODO: verify if allow_plus is really not used for this case
                let named: Named = input.parse()?;
                Ok(ReturnType::Named(arrow, Box::new(named)))
            } else {
                // just a single type input
                // (except tuples, which fall in the case above)

                // TODO: the `Type::parse()` sends allow_plus=true
                // so a verification is needed for cases when
                // allow_plus=false (a direct call to parse below would be
                // invalid)
                assert!(allow_plus);
                let ty: syn::Type = input.parse()?;
                Ok(ReturnType::Type(arrow, Box::new(ty)))
            }
        } else {
            Ok(ReturnType::Default)
        }
    }
}

// copy form `impl Parse for ReturnType` from `syn::ty.rs`
impl syn::parse::Parse for ReturnType {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        Self::parse(input, true)
    }
}

// information other than the type is lost
impl Into<syn::ReturnType> for ReturnType {
    fn into(self) -> syn::ReturnType {
        match self {
            ReturnType::Default => syn::ReturnType::Default,
            ReturnType::Type(right_arrow, type_boxed) => {
                syn::ReturnType::Type(right_arrow, type_boxed)
            }
            ReturnType::Named(right_arrow, named_boxed) => {
                use named::fn_arg_ret::FnArgRet;
                let types: syn::punctuated::Punctuated<syn::Type, syn::token::Comma> = named_boxed
                    .outputs
                    .iter()
                    .cloned()
                    .map(|fn_arg_ret| match fn_arg_ret {
                        FnArgRet::Ignored(ty) => ty,
                        FnArgRet::Captured(arg_captured) => arg_captured.ty,
                    })
                    .collect();

                match types.len() {
                    // fn a() -> () {} case
                    0 => syn::ReturnType::Default,

                    // single return type
                    1 => {
                        let ty = types.first().unwrap().into_value();
                        syn::ReturnType::Type(right_arrow, Box::new(ty.clone()))
                    }

                    // multiple returns (wrap in a tupple)
                    _l => {
                        // multiple returns, wrap into a tuple
                        let ty = syn::TypeTuple {
                            paren_token: named_boxed.paren_token,
                            elems: types,
                        };
                        let ty = syn::Type::Tuple(ty);
                        syn::ReturnType::Type(right_arrow, Box::new(ty))
                    }
                }
            }
        }
    }
}

impl Into<Option<Captures>> for ReturnType {
    fn into(self) -> Option<Captures> {
        match self {
            ReturnType::Default => None,
            ReturnType::Type(_right_arrow, _type_boxed) => None,
            ReturnType::Named(_right_arrow, named_boxed) => {
                use named::fn_arg_ret::FnArgRet;
                let captureds: Captures = named_boxed
                    .outputs
                    .iter()
                    .filter_map(|fn_arg_ret| match fn_arg_ret {
                        FnArgRet::Captured(arg_captured) => Some(arg_captured),
                        FnArgRet::Ignored(_ty) => None,
                    })
                    .cloned()
                    .collect();
                if captureds.iter().len() == 0 {
                    None
                } else {
                    Some(captureds)
                }
            }
        }
    }
}
