use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(GzMessage)]
pub fn gz_message_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_gz_message_macro(&ast)
}

fn impl_gz_message_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl crate::GzMessage for #name {
            const TYPE_NAME: &'static str = concat!("gz.msgs.", stringify!(#name));
        }
    };
    gen.into()
}

