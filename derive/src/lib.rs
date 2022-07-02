use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, Parser};
use syn::{self, parse_macro_input, Block, Expr, FnArg, Ident, ItemFn, Pat, Signature};

#[proc_macro_attribute]
pub fn foreign_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item: ItemFn = parse_macro_input!(item as ItemFn);
    let offset = parse_macro_input!(attr as Expr);
    let Signature { inputs, output, .. } = &item.sig;

    let names: syn::Result<Vec<Ident>> = inputs
        .iter()
        .map(|arg| {
            if let FnArg::Typed(pt) = arg {
                if let Pat::Ident(id) = pt.pat.as_ref() {
                    Ok(id.ident.clone())
                } else {
                    Err(syn::Error::new_spanned(arg, "only identifiers allowed"))
                }
            } else {
                Err(syn::Error::new_spanned(arg, "self parameter not allowed"))
            }
        })
        .collect();

    match names {
        Ok(names) => {
            let block = quote! {{
                    let addr = memhack::resolve_rva(#offset);
                    let func: extern "C" fn(#inputs) #output = unsafe { std::mem::transmute(addr) };
                    func(#(#names),*)
            }};

            item.block = Block::parse.parse2(block).unwrap().into();
            item.into_token_stream().into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}
