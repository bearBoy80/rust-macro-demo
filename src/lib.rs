mod enum_from_daring;
use enum_from_daring::process_from_daring;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    //将enum token转换成语法树
    let input = syn::parse_macro_input!(input as DeriveInput);
    println!("{:#?}", input);
    //获取要实现ident
    let ident = input.ident;
    //获取Data信息，里面包含enum成员变量相关的信息,里面有一个variants数组
    //variants 里面成员有fields,我们需要解析Fields::Unnamed里面的ty里面的信息,
    //通过这个信息来组装 from traint相关代码
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom only works on enums"),
    };
    let from_impls = variants.iter().map(|variant| {
        let val = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                //针对不为1的处理
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().expect("should have 1 field");
                    let ty = &field.ty;
                    quote! {
                        impl From <#ty> for #ident {
                            fn from(v: #ty) -> Self{
                                #ident::#val(v)
                            }
                        }
                    }
                }
            }
            syn::Fields::Unit => quote! {},
            syn::Fields::Named(_field) => quote! {},
        }
    });

    quote! {
        #(#from_impls)*
    }
    .into()
}
#[proc_macro_derive(EnumFromDaring)]
pub fn derive_enum_from_daring(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    process_from_daring(input).into()
}
