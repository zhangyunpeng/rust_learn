use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn func_log(_ : TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as ItemFn);
    let func_name = &func.sig.ident;
    let func_block = &func.block;
    let outout = quote!(
        {
            println!("{} start", stringify!(#func_name));
            let result = {
                #func_block
            };
            println!("{} end", stringify!(#func_name));
            result
        }
    );
    func.block = syn::parse2(outout).unwrap();
    quote!{#func}.into()
}

