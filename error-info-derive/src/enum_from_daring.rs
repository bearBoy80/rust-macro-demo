use darling::{
    ast::{Data, Fields, Style},
    FromDeriveInput, FromField, FromVariant,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;
#[derive(Debug, FromDeriveInput)]
struct EnumFromDaring {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<EnumVariants, ()>,
}
#[derive(Debug, FromVariant)]
struct EnumVariants {
    ident: syn::Ident,
    fields: Fields<EnumVariantFields>,
}
#[derive(Debug, FromField)]
struct EnumVariantFields {
    ty: syn::Type,
}

pub(crate) fn process_from_daring(input: DeriveInput) -> TokenStream {
    let EnumFromDaring {
        ident,
        generics,
        data: Data::Enum(data),
    } = EnumFromDaring::from_derive_input(&input).expect("can not parse input")
    else {
        panic!("EnumFromDarling only works on enums");
    };
    let from_impls = data.iter().map(|variant| {
        let var = &variant.ident;
        let style = variant.fields.style;
        match style {
            Style::Tuple if variant.fields.len() == 1 => {
                let field = variant.fields.iter().next().expect("should have 1 field");
                let ty = &field.ty;
                quote! {
                    impl #generics From<#ty> for #ident #generics{
                        fn from(v: #ty) -> Self{
                            #ident::#var(v)
                        }
                    }
                }
            }
            _ => quote! {},
        }
    });
    quote! {
        #(#from_impls)*
    }
}
