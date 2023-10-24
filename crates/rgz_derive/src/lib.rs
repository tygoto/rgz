use proc_macro::TokenStream;
use quote::quote;
use regex::Regex;
use syn;

#[proc_macro_derive(GzMessage)]
pub fn gz_message_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_gz_message_macro(&ast)
}

fn impl_gz_message_macro(ast: &syn::DeriveInput) -> TokenStream {
    // let name = &ast.ident;
    let name = &ast.ident;
    let name_str = name.to_string();
    let re = Regex::new(r"(V)$").unwrap();
    let replace_name = re.replace(&name_str, "_V").to_string();
    let type_name = format!("gz.msgs.{}", replace_name);

    let gen = quote! {
        impl crate::GzMessage for #name {
            const TYPE_NAME: &'static str = #type_name;
        }
    };
    gen.into()
}
