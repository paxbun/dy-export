use proc_macro::*;
use quote::*;
use syn::*;

#[proc_macro_attribute]
pub fn dy_export(_: TokenStream, body: TokenStream) -> TokenStream {
    let mut rtn = proc_macro2::TokenStream::new();
    let input = parse_macro_input!(body as ItemFn);

    // Copy attributes
    for attr in input.attrs.iter() {
        attr.to_tokens(&mut rtn);
    }

    let name = input.sig.ident;
    let impl_name = syn::Ident::new(&format!("__impl_{}", name), proc_macro2::Span::call_site());

    let inputs = input.sig.inputs;
    let output = input.sig.output;
    let block = input.block;

    let body = quote::quote! {
        #[no_mangle]
        pub unsafe extern "C" fn #name(__a: *const dy::ValuePtr, __n: usize) -> dy::ValuePtr {
            pub fn #impl_name(#inputs) #output #block

            let __slice = std::slice::from_raw_parts(__a, __n);
            let __rtn = #impl_name(
                __slice
                    .iter()
                    .map(|__ptr| dy::Borrowed::from_ptr(*__ptr))
                    .collect(),
            );
            __rtn.into_ptr()
        }
    };
    body.to_tokens(&mut rtn);
    TokenStream::from(rtn)
}
