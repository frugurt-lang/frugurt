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

    let mut get_set_op_flag = false;

    for i in attrs {
        if let TokenTree::Ident(ident) = i {
            match ident.to_string().as_str() {
                "as_any" => item.items.push(syn::parse_quote! {
                    fn as_any(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
                        self
                    }
                }),

                "get_uid" => item.items.push(syn::parse_quote! {
                    fn get_uid(&self) -> uid::Id<crate::interpreter::value::native_object::OfObject> {
                        crate::static_uid!()
                    }
                }),

                "get_type" => item.items.push(syn::parse_quote! {
                    fn get_type(&self) -> FruValue {
                        crate::stdlib::builtins::builtin_type_type::BuiltinTypeType::get_value()
                    }
                }),

                "get_set_op" => {
                    get_set_op_flag = true;
                    item.items.push(syn::parse_quote! {
                        fn get_operator(
                            &self,
                            ident: crate::interpreter::identifier::OperatorIdentifier,
                        ) -> Option<crate::interpreter::value::operator::AnyOperator> {
                            OPERATORS.lock().unwrap().get(&ident).cloned()
                        }
                    });
                    item.items.push(syn::parse_quote! {
                        fn set_operator(
                            &self,
                            ident: crate::interpreter::identifier::OperatorIdentifier,
                            value: crate::interpreter::value::operator::AnyOperator,
                        ) -> Result<(), crate::interpreter::error::FruError> {
                            match OPERATORS.lock().unwrap().entry(ident) {
                                std::collections::hash_map::Entry::Occupied(_) => {
                                    crate::interpreter::error::FruError::new_res(format!("operator `{:?}` is already set", ident.op))
                                }
                                std::collections::hash_map::Entry::Vacant(entry) => {
                                    entry.insert(value);
                                    Ok(())
                                }
                            }
                        }
                    });
                }

                "fru_clone" => item.items.push(syn::parse_quote! {
                    fn fru_clone(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn INativeObject> {
                        self
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
            "get_type" => 3,
            "call" => 4,
            "instantiate" => 5,
            "get_prop" => 6,
            "set_prop" => 7,
            "get_operator" => 8,
            "set_operator" => 9,
            "fru_clone" => 10,
            u => unreachable!("{}", u),
        },
        u => unreachable!("{}", u.to_token_stream().to_string()),
    });

    let mut item = item.to_token_stream();

    if get_set_op_flag {
        item.extend(quote! {
            static OPERATORS: once_cell::sync::Lazy<
                std::sync::Mutex<
                    std::collections::HashMap<
                        crate::interpreter::identifier::OperatorIdentifier,
                        crate::interpreter::value::operator::AnyOperator,
                    >,
                >,
            > = once_cell::sync::Lazy::new(Default::default);
        });
    }

    item.into()
}
