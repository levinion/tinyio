use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemFn);
    let stmts = input.block.stmts;
    let expend = quote! {
        fn main(){
            let (executor, spawner) = tinyio::init();
            spawner.spawn({
                async move {
                    #(#stmts)*
                }
            });
            drop(spawner);
            executor.run();
        }
    };
    expend.into()
}
