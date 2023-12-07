use proc_macro;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    return impl_hello_macro(&ast);
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Procedural Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    return gen.into();
}
