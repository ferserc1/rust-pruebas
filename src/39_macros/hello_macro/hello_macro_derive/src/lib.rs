use proc_macro::TokenStream;
use quote::quote;
use syn;

fn impl_hello_macro(ast: &syn::DeriveInput) ->TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hola, Macro! Mi nombre es {}", stringify!(#name));
            }
        }
    };
    gen.into()

}

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construimos una representación sintáctica del código fuente
    // que podamos manipular
    let ast = syn::parse(input).unwrap();

    // Construimos la implementación del rasgo `HelloMacro` para el tipo
    impl_hello_macro(&ast)
}