extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn debug_print(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn = parse_macro_input!(item as ItemFn);
    let ident = &item_fn.sig.ident;
    item_fn.block.stmts.insert(
        0,
        syn::parse_quote!(println!("entering the function: {}", stringify!(#ident));
        ),
    );
    TokenStream::from(quote! {
        #item_fn
    })
}

// macro_rules! gcd {
//     ($a: expr, $b: expr) => {{
//         let mut m = $a;
//         let mut n = $b;

//         while m != 0 {
//             if m < n {
//                 let t = m;
//                 m = n;
//                 n = t;
//             }
//             m = m % n;
//         }
//         n
//     }};
// }

// fn main() {
//     println!("{}", gcd!(14, 15));
// }
