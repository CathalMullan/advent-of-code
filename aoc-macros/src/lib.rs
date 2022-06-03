use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::Token;
use syn::{parse_macro_input, FnArg, Ident, ItemFn, Signature};

#[proc_macro_attribute]
pub fn aoc(attr: TokenStream, input: TokenStream) -> TokenStream {
    let title = attr.to_string();
    let title = &title[1..title.len() - 1];

    let func = parse_macro_input!(input as ItemFn);
    let ItemFn { attrs, vis, sig, .. }: &ItemFn = &func;
    let func_name = &sig.ident;

    let renamed_func_name = format_ident!("{}_timer", func_name);
    let renamed_func = ItemFn {
        sig: Signature {
            ident: Ident::new(&renamed_func_name.to_string(), sig.ident.span()),
            ..func.sig.clone()
        },
        ..func.clone()
    };

    let wrapper_sig = Signature {
        ident: Ident::new(&func_name.to_string(), sig.ident.span()),
        ..sig.clone()
    };

    let args: Punctuated<_, Token![,]> = sig
        .inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Typed(pattern_type) => &pattern_type.pat,
            FnArg::Receiver(_) => unreachable!(),
        })
        .collect();

    let result = quote! {
        #renamed_func

        #(#attrs)*
        #vis #wrapper_sig {
            use std::time::{Duration, Instant};

            let timer = Instant::now();
            let result = #renamed_func_name(#args);
            let elapsed = timer.elapsed();

            println!("{}", #title);
            println!("Solution: {}", result);
            println!("Time: {:#?}", elapsed);

            result
        }
    };

    result.into()
}
