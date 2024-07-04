#![allow(unused)]
use proc_macro::TokenStream;
use quote::TokenStreamExt;
use syn::{Attribute, Data, DeriveInput, Fields, Meta};
fn grab_format(attr: &Attribute) -> proc_macro2::TokenStream {
    let &Attribute { 
        pound_token: _, 
        style: _, 
        bracket_token: _, 
        meta 
    } = &attr;
    match meta {
        Meta::List(val) => {
            return val.tokens.clone();
        }
        _ => {
            panic!("Try using a list")
        }
    }
}
fn attribute_tokens(fields: Fields) -> proc_macro2::TokenStream {
    let mut token_start = quote::quote! {
        let mut offset: u32 = 0;
        let mut attributes = Vec::new();
    };
    let tokens = fields.iter().enumerate().map(|(i, field)|{
        let ty = &field.ty;
        let attrs = &field.attrs[0];
        let format = grab_format(attrs);
        quote::quote! {
            attributes.push(
                ash::vk::VertexInputAttributeDescription {
                    binding,
                    location: #i as u32,
                    offset: offset,
                    format: #format,
                });
                offset += std::mem::size_of::<#ty>() as u32;
        }
    }).fold(proc_macro2::TokenStream::new(), |mut a, b|{ 
        a.append_all(b);
        a
    });
    token_start.append_all(tokens);
    token_start
}
fn impl_vertex_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = if let Data::Struct(structure) = &ast.data {
        structure.fields.clone()
    } else {
        panic!("Not a Struct")
    };
    let token_start = attribute_tokens(fields);
    
    let gen = quote::quote! {
        impl Vertex for #name {
            fn binding(binding: u32, rate: ash::vk::VertexInputRate) -> ash::vk::VertexInputBindingDescription {
                ash::vk::VertexInputBindingDescription {
                    binding,
                    input_rate: rate,
                    stride: std::mem::size_of::<Self>() as u32,
                }
            }
            fn attributes(binding: u32) -> Vec<ash::vk::VertexInputAttributeDescription> {
                // #token_start
                #token_start
                attributes
            }
        }
    };
    gen.into()
}
#[proc_macro_derive(Vertex, attributes(format))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_vertex_macro(&ast)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
