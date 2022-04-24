extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(ObjcObjectBase)]
pub fn derive_objc_object_base(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let ref name = ast.ident;
    let (genparams, genidents, where_clause) = ast.generics.split_for_impl();
    let fields = if let syn::Data::Struct(ref ds) = ast.data {
        &ds.fields
    } else {
        panic!("Only struct can derive from ObjcObjectBase");
    };
    let q = match fields {
        &syn::Fields::Unnamed(_) => quote! {
           impl #genparams ObjcObjectBase for #name #genidents #where_clause {
               fn objid(&self) -> &objc::runtime::Object { &self.0 }
               fn objid_mut(&mut self) -> &mut objc::runtime::Object { &mut self.0 }
           }
        },
        &syn::Fields::Named(_) => panic!("A struct that has named fields cannot derive from ObjcObjectBase"),
        &syn::Fields::Unit => panic!("Unit struct is not Objc object"),
    };

    q.into()
}
