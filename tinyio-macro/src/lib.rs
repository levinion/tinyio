use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemFn);
    let asyncness = input
        .sig
        .asyncness
        .unwrap_or_else(|| panic!("fn main needs to be async fn"));
    let stmts = input.block.stmts;
    let expend = quote! {
        fn main(){
            tinyio::init();
            tinyio::spawn({
                #asyncness move {
                    #(#stmts)*
                }
            });
            tinyio::run();
        }
    };
    expend.into()
}
