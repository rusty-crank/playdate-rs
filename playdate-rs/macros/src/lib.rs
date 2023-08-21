use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn app(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let name = &input.ident;
    let result = quote! {
        #input
        ::playdate_rs::register_playdate_app!(#name);
    };
    result.into()
}
