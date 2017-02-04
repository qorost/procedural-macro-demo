extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloWorld)]
pub fn hello_world(input : TokenStream) -> TokenStream {

    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();

    let gen = impl_hello_world(&ast);


    gen.parse().unwrap()

}


fn impl_hello_world(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
    
        impl HelloWorld for #name {
            fn hello_world() {
                println!("Hello, world , my name is {]",stringify!(#name) );
            }
            
        }
    }

}





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
