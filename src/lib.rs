use {
    proc_macro::TokenStream,
    proc_macro2::{Span, TokenStream as TokenStream2},
    quote::quote,
    syn::{
        DeriveInput,
        Expr,
        Path,
        Token,
        parse::{Parse, ParseStream},
        parse_macro_input,
        punctuated::Punctuated,
    },
};

struct Args {
    structs: Vec<StructWithMaybeFunction>,
}

struct StructWithMaybeFunction {
    path: Path,
    opt_fn: Option<Path>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let structs = Punctuated::<syn::Expr, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            structs: structs
                .into_iter()
                .map(|expr| {
                    Ok(match expr {
                        Expr::Path(path) => {
                            StructWithMaybeFunction {
                                path: path.path,
                                opt_fn: None,
                            }
                        },
                        Expr::Call(call) => {
                            StructWithMaybeFunction {
                                path: match *call.func {
                                    Expr::Path(expr_path) => expr_path.path,
                                    _ => unreachable!(),
                                },
                                opt_fn: Some(match call.args.into_iter().next() {
                                    Some(Expr::Path(expr_path)) => expr_path.path,
                                    None => {
                                        return Err(syn::Error::new(
                                            Span::call_site(),
                                            "Expected at least one argument between parenthesis",
                                        ));
                                    },
                                    _ => unreachable!(),
                                }),
                            }
                        },
                        _ => panic!("Unexpected."),
                    })
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[proc_macro_attribute]
pub fn catfishing(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_clone = input.clone();
    let args_parsed = parse_macro_input!(args as Args);
    let input_parsed = parse_macro_input!(input_clone as DeriveInput);

    let syn::Data::Struct(syn::DataStruct { fields, .. }) = &input_parsed.data else {
        return quote! {
            compile_error!("Only structs are supported for this macro");
        }
        .into();
    };

    let implementations: Vec<_> = args_parsed
        .structs
        .iter()
        .map(|struct_with_maybe_function| {
            let methods: Vec<_> = fields
                .into_iter()
                .map(|field| {
                    let field_name = &field.ident;
                    let field_ty = &field.ty;

                    if let Some(r#fn) = &struct_with_maybe_function.opt_fn {
                        quote! {
                            pub fn #field_name(&self) -> #field_ty {
                                #r#fn(&self.0).#field_name
                            }
                        }
                    } else {
                        quote! {
                            pub fn #field_name(&self) -> &#field_ty {
                                &self.0.#field_name
                            }
                        }
                    }
                })
                .collect();

            let struct_path = &struct_with_maybe_function.path;

            quote! {
                impl #struct_path {
                    #(#methods)*
                }
            }
        })
        .collect();

    let input2: TokenStream2 = input.into();
    quote!(
        #(#implementations)*
        #input2
    )
    .into()
}
