use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn external_refcounted(attr: TokenStream, item: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(item as syn::DeriveInput);
    let name = &derive_input.ident;
    let (ty_generics, generics_args, where_clause) = derive_input.generics.split_for_impl();
    let mut args =
        parse_macro_input!(attr with syn::punctuated::Punctuated<syn::Path, syn::Token![,]>::parse_terminated)
            .into_iter();
    let retained_fn = args.next().expect("requires retained fn as first");
    let release_fn = args.next().expect("requires release fn as second");

    let derive = quote! {
        impl #ty_generics crate::ExternalRefcounted for #name #generics_args #where_clause {
            unsafe fn retain(p: *mut Self) -> *mut Self {
                #retained_fn(p)
            }

            unsafe fn release(p: *mut Self) {
                #release_fn(p)
            }
        }
    };

    quote! { #derive_input #derive }.into()
}
