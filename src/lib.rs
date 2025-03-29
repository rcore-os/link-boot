use proc_macro::TokenStream;
use quote::quote;
use syn::parse_quote;

#[proc_macro_attribute]
pub fn link_boot(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(input as syn::ItemMod);

    if let Some((_brace, items)) = input.content.as_mut() {
        for item in items {
            match item {
                syn::Item::Fn(v) => {
                    v.attrs.push(parse_quote! {
                        #[unsafe(link_section = ".text.boot")]
                    });
                }
                syn::Item::Static(v) => {
                    v.attrs.push(parse_quote! {
                        #[unsafe(link_section = ".data.boot")]
                    });
                }
                syn::Item::Impl(v) => {
                    v.attrs.push(parse_quote! {
                        #[unsafe(link_section = ".text.boot")]
                    });

                    for item in v.items.iter_mut() {
                        if let syn::ImplItem::Fn(v) = item {
                            v.attrs.push(parse_quote! {
                                #[unsafe(link_section = ".text.boot")]
                            });
                        }
                    }
                }
                _ => {}
            }
        }
    }

    let items = input.content.unwrap().1;

    quote! {
      #(#items)*
    }
    .into()
}
