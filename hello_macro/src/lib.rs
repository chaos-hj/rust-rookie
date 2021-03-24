extern crate proc_macro;

use crate::proc_macro::TokenStream;
// use proc_macro::TokenStream;
use syn::{Ident, parse};
use quote::__rt::Span;


#[proc_macro]
pub fn my_proc_macro(ident: TokenStream) -> TokenStream {
    let new_func_name = format!("test_{}", ident.to_string());
    let concated_ident = Ident::new(&new_func_name, Span::call_site()); // 创建新的ident，函数名

    let expanded = quote! {
        // 不能直接这样写trait bound，T: Debug
        fn #concated_ident<T: std::fmt::Debug>(t: T) {
            println!("{:?}", t);
        }
    };
    expanded.into()
}

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", #name);
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Show)]
pub fn derive_show(item: TokenStream) -> TokenStream {
    // 解析整个token tree
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident; // 结构体名字

    // 提取结构体里的字段
    let expanded = match input.data {
        Data::Struct(DataStruct{ref fields,..}) => {
            if let Fields::Named(ref fields_name) = fields {
                // 结构体中可能是多个字段
                let get_selfs: Vec<_> = fields_name.named.iter().map(|field| {
                    let field_name = field.ident.as_ref().unwrap(); // 字段名字
                    quote! {
                        &self.#field_name
                    }
                }).collect();

            let implemented_show = quote! {
                // 下面就是Display trait的定义了
                // use std::fmt; // 不要这样import，因为std::fmt是全局的，无法做到卫生性(hygiene)
                // 编译器会报错重复import fmt当你多次使用Show之后
                impl std::fmt::Display for #struct_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        // #(#get_self),*，这是多重匹配，生成的样子大概是这样：&self.a, &self.b, &self.c, ...
                        // 用法和标准宏有点像，关于多个匹配，可以看这个文档
                        // https://docs.rs/quote/1.0.0/quote/macro.quote.html
                        write!(f, "{} {:?}", stringify!(#struct_name), (#(#get_selfs),*))
                    }
                }
            };
            implemented_show
            
            } else {
                panic!("sorry, may it's a complicated struct.");
            }
        }
        _ => panic!("sorry, Show is not implemented for union or enum type.")
    };
    expanded.into()
}