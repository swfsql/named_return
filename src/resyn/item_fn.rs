pub mod fn_decl;
use fn_decl::{FnDecl, ReturnType};

// similar to `syn::ItemFn`
pub struct ItemFn {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub constness: Option<syn::token::Const>,
    pub unsafety: Option<syn::token::Unsafe>,
    pub asyncness: Option<syn::token::Async>,
    pub abi: Option<syn::Abi>,
    pub ident: syn::Ident,
    pub decl: Box<FnDecl>,
    pub block: Box<syn::Block>,
}

// from `impl syn::parse::Parse for syn::ItemFn`.
impl syn::parse::Parse for ItemFn {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let outer_attrs = input.call(syn::Attribute::parse_outer)?;
        let vis: syn::Visibility = input.parse()?;
        let constness: Option<syn::Token![const]> = input.parse()?;
        let unsafety: Option<syn::Token![unsafe]> = input.parse()?;
        // let unsafety: syn::Token![unsafe] = input.parse()?;
        // let unsafety = Some(unsafety);
        let asyncness: Option<syn::Token![async]> = input.parse()?;
        let abi: Option<syn::Abi> = input.parse()?;
        let fn_token: syn::Token![fn] = input.parse()?;
        let ident: syn::Ident = input.parse()?;
        let generics: syn::Generics = input.parse()?;

        let content;
        let paren_token = syn::parenthesized!(content in input);
        let inputs = content.parse_terminated(syn::FnArg::parse)?;
        let variadic: Option<syn::Token![...]> = match inputs.last() {
            Some(syn::punctuated::Pair::End(&syn::FnArg::Captured(syn::ArgCaptured {
                ty: syn::Type::Verbatim(syn::TypeVerbatim { ref tts }),
                ..
            }))) => syn::parse2(tts.clone()).ok(),
            _ => None,
        };

        let output: ReturnType = input.parse()?;
        let where_clause: Option<syn::WhereClause> = input.parse()?;

        let content;
        let brace_token = syn::braced!(content in input);
        let inner_attrs = content.call(syn::Attribute::parse_inner)?;
        let stmts = content.call(syn::Block::parse_within)?;

        Ok(ItemFn {
            attrs: private::attrs(outer_attrs, inner_attrs),
            vis: vis,
            constness: constness,
            unsafety: unsafety,
            asyncness: asyncness,
            abi: abi,
            ident: ident,
            decl: Box::new(FnDecl {
                fn_token: fn_token,
                paren_token: paren_token,
                inputs: inputs,
                output: output,
                variadic: variadic,
                generics: syn::Generics {
                    where_clause: where_clause,
                    ..generics
                },
            }),
            block: Box::new(syn::Block {
                brace_token: brace_token,
                stmts: stmts,
            }),
        })
    }
}

// from `impl quote::ToTokens for syn::ItemFn`.
impl quote::ToTokens for ItemFn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use attr::FilterAttrs as _;
        use quote::TokenStreamExt as _;
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.constness.to_tokens(tokens);
        self.unsafety.to_tokens(tokens);
        self.asyncness.to_tokens(tokens);
        self.abi.to_tokens(tokens);
        NamedDecl(&self.decl, &self.ident).to_tokens(tokens);
        self.block.brace_token.surround(tokens, |tokens| {
            tokens.append_all(self.attrs.inner());
            tokens.append_all(&self.block.stmts);
        });
    }
}

// from `syn::private`, impl from `syn::attr.rs`
mod private {
    pub fn attrs(outer: Vec<syn::Attribute>, inner: Vec<syn::Attribute>) -> Vec<syn::Attribute> {
        let mut attrs = outer;
        attrs.extend(inner);
        attrs
    }
}

// from `syn::attr` (private mod)
mod attr {
    pub trait FilterAttrs<'a> {
        type Ret: Iterator<Item = &'a syn::Attribute>;

        fn outer(self) -> Self::Ret;
        fn inner(self) -> Self::Ret;
    }

    impl<'a, T> FilterAttrs<'a> for T
    where
        T: IntoIterator<Item = &'a syn::Attribute>,
    {
        type Ret = std::iter::Filter<T::IntoIter, fn(&&syn::Attribute) -> bool>;

        fn outer(self) -> Self::Ret {
            #[cfg_attr(feature = "cargo-clippy", allow(trivially_copy_pass_by_ref))]
            fn is_outer(attr: &&syn::Attribute) -> bool {
                match attr.style {
                    syn::AttrStyle::Outer => true,
                    _ => false,
                }
            }
            self.into_iter().filter(is_outer)
        }

        fn inner(self) -> Self::Ret {
            #[cfg_attr(feature = "cargo-clippy", allow(trivially_copy_pass_by_ref))]
            fn is_inner(attr: &&syn::Attribute) -> bool {
                match attr.style {
                    syn::AttrStyle::Inner(_) => true,
                    _ => false,
                }
            }
            self.into_iter().filter(is_inner)
        }
    }
}

// from `syn::item::NamedDecl` (private structure)
struct NamedDecl<'a>(&'a FnDecl, &'a syn::Ident);
impl<'a> quote::ToTokens for NamedDecl<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.fn_token.to_tokens(tokens);
        self.1.to_tokens(tokens);
        self.0.generics.to_tokens(tokens);
        self.0.paren_token.surround(tokens, |tokens| {
            self.0.inputs.to_tokens(tokens);
            if self.0.variadic.is_some() && !self.0.inputs.empty_or_trailing() {
                <syn::Token![,]>::default().to_tokens(tokens);
            }
            self.0.variadic.to_tokens(tokens);
        });
        let output: syn::ReturnType = self.0.output.clone().into();
        output.to_tokens(tokens);
        self.0.generics.where_clause.to_tokens(tokens);
    }
}
