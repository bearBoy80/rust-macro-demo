use darling::{ast::{Data, Fields, Style}, util, FromDeriveInput, FromVariant};
use proc_macro2::TokenStream;
use syn::DeriveInput;
use quote::quote;


#[derive(Debug,FromDeriveInput)]
#[darling(attributes(error_info))]
#[allow(dead_code)]
struct ErrData{
    ident:syn::Ident,
    generics:syn::Generics,
    app_type:syn::Type,
    prefix:String,
    data:Data<EnumVariants, ()>,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(error_info))]
#[allow(dead_code)]
struct EnumVariants{
    ident: syn::Ident,
    fields:Fields<util::Ignored>,
    code:String,
    app_code:String,
    #[darling(default)]
    client_msg:String,
}
pub(crate) fn process_error_info(input: DeriveInput) -> TokenStream {
    let ErrData {
        ident:name,
        generics,
        data: Data::Enum(data),
        app_type,
        prefix
    } = ErrData::from_derive_input(&input).expect("Can not parse input")
    else{
        panic!("Only enum is supported")
    };
    let code = data.iter().map(|v|{
        let EnumVariants {
            ident,
            fields,
            code,
            app_code,
            client_msg,
        } = v; 
        let code = format!("{}{}", prefix, code);
        let varint_code = match fields.style {
            Style::Struct => quote! { #name::#ident { .. } },
            Style::Tuple => quote! { #name::#ident(_) },
            Style::Unit => quote! { #name::#ident },
        };
        quote! {
            #varint_code =>{
                ErrInfo::new(
                    #app_code,
                    #code,
                    #client_msg,
                    self
                )
            }
        }
    }).collect::<Vec<_>>();
    quote! {
        use error_info::ErrInfo;
        impl #generics ToErrorInfo for #name #generics{
            type T = #app_type;
            fn to_error_info(&self) -> ErrInfo<Self::T> {
                match self {
                    #(#code),*
                }
            }
        } 
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_process_error_info(){
        let input = r#"
        #[derive(thiserror::Error, ToErrorInfo)]
        #[error_info(app_type="http::StatusCode", prefix="01")]
        pub enum MyError {
        #[error("Invalid command: {0}")]
        #[error_info(code="IC", app_code="400")]
        InvalidCommand(String),

        #[error("Invalid argument: {0}")]
        #[error_info(code="IA", app_code="400", client_msg="friendly msg")]
        InvalidArgument(String),

        #[error("{0}")]
        #[error_info(code="RE", app_code="500")]
        RespError(#[from] RespError),
        }
        "#;
        let parsed = syn::parse_str(input).unwrap();
        let info = ErrData::from_derive_input(&parsed).unwrap();
        println!("{:#?}", info);

        assert_eq!(info.ident.to_string(), "MyError");
        assert_eq!(info.prefix, "01");

        let code = process_error_info(parsed);
        println!("{}", code);
    }
}