use proc_macro::TokenStream;
use std::hash::{DefaultHasher, Hash, Hasher};

use quote::quote;
use syn::LitStr;

#[proc_macro]
pub fn static_ident(input: TokenStream) -> TokenStream {
    let ast: LitStr = syn::parse(input).unwrap();

    let ident = ast.value();

    let mut hasher = DefaultHasher::new();
    ident.hash(&mut hasher);
    let hashed_ident = hasher.finish();

    quote! {
        {
            #[ctor::ctor]
            fn ident_ctor() {
                Identifier::new(#ident);
            }
            Identifier::new_unchecked(#hashed_ident)
        }
    }
    .into()
}
