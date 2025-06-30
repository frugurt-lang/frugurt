use proc_macro::{TokenStream, TokenTree};
use std::hash::{DefaultHasher, Hash, Hasher};

use quote::{quote, ToTokens};
use syn::{parse_macro_input, ImplItem, ItemImpl, LitStr};

#[proc_macro]
pub fn static_ident(input: TokenStream) -> TokenStream {
    let ast: LitStr = syn::parse(input).unwrap();

    let ident = ast.value();

    let mut hasher = DefaultHasher::new();
    ident.hash(&mut hasher);
    let hashed_ident = hasher.finish();

    let gen = quote! {
        {
            #[ctor::ctor]
            fn ident_ctor() {
                Identifier::new(#ident);
            }
            Identifier::new_unchecked(#hashed_ident)
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn derive_nat(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemImpl);

    for i in attrs {
        if let TokenTree::Ident(ident) = i {
            match ident.to_string().as_str() {
                "as_any" => item.items.push(syn::parse_quote! {
                    fn as_any(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
                        self
                    }
                }),

                "get_uid" => item.items.push(syn::parse_quote! {
                    fn get_uid(&self) -> crate::common::IdOfObject {
                        self.uid
                    }
                }),

                "get_type_uid" => item.items.push(syn::parse_quote! {
                    fn get_type_uid(&self) -> crate::common::IdOfObject {
                        crate::static_uid!()
                    }
                }),

                unexpected => panic!("Unknown attribute: {}", unexpected),
            }
        }
    }

    item.items.sort_by_key(|y| match y {
        ImplItem::Fn(s) => match s.sig.ident.to_string().as_str() {
            "as_any" => 1,
            "get_uid" => 2,
            "get_type_uid" => 3,
            "call" => 4,
            "index" => 5,
            "get_prop" => 6,
            "set_prop" => 7,
            "let_prop" => 8,

            u => unreachable!("{}", u),
        },
        u => unreachable!("{}", u.to_token_stream().to_string()),
    });

    item.to_token_stream().into()
}
