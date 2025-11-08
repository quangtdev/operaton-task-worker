extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, AttributeArgs, ItemFn, Meta, NestedMeta, Lit};

/// Attribute macro to register an external task handler function with a name (activityId/topic).
/// Usage: `#[task_handler(name = "example_echo")]`
#[proc_macro_attribute]
pub fn task_handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input_fn = parse_macro_input!(item as ItemFn);

    // Extract name = "..."
    let mut name_value: Option<String> = None;
    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
            if nv.path.is_ident("name") {
                if let Lit::Str(s) = nv.lit {
                    name_value = Some(s.value());
                }
            }
        }
    }

    let name_value = match name_value {
        Some(v) => v,
        None => panic!("#[task_handler] requires `name = \"...\"`"),
    };

    let fn_ident = input_fn.sig.ident.clone();

    // Emit original function unchanged + inventory registration in this crate's context
    let expanded = quote! {
        #input_fn
        
        const _: () = {
            // Ensure `inventory` is linked
            inventory::submit! {
                crate::registry::Handler {
                    name: #name_value,
                    func: #fn_ident,
                }
            }
        };
    };

    TokenStream::from(expanded)
}
