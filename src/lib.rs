use {
    proc_macro::TokenStream,
    proc_macro2::TokenStream as TokenStream2,
    quote::quote,
    std::collections::HashSet,
    syn::{
        DeriveInput,
        Ident,
        Token,
        parse::{Parse, ParseStream},
        parse_macro_input,
        punctuated::Punctuated,
    },
};

struct Args {
    structs: HashSet<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let structs = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            structs: structs.into_iter().collect(),
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

    let methods: Vec<_> = fields
        .into_iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_ty = &field.ty;

            quote! {
                fn #field_name(&self) -> &#field_ty {
                    &self.0.#field_name
                }
            }
        })
        .collect();

    let implementations: Vec<_> = args_parsed
        .structs
        .iter()
        .map(|r#struct| {
            quote! {
                impl #r#struct {
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
