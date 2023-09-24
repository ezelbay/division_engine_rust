use std::str::FromStr;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(VertexData, attributes(location))]
pub fn derive_vertex_data(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let locations = match input.data {
        Data::Struct(s) => gather_locations(s.fields),
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };

    let attrs_list = locations
        .into_iter()
        .map(|(index, ty_name)| (index, map_typename_to_shader_type(ty_name)))
        .map(|(index, var_ty)| {
            format!(
                "VertexAttributeDescriptor {{ 
                    location: {index}, 
                    field_type: ShaderVariableType::{var_ty} 
                }}"
            )
        })
        .collect::<Vec<String>>()
        .join(",");
    
    let data_ty_name = input.ident.to_string();
    let vertex_attr_impl = format!(
        "impl VertexData for {data_ty_name} {{
            fn vertex_attributes() -> Vec<VertexAttributeDescriptor> {{
                [{attrs_list}]
                    .into_iter()
                    .collect::<Vec<VertexAttributeDescriptor>>()
            }}
        }}"
    );

    TokenStream::from_str(&vertex_attr_impl).unwrap()
}

fn map_typename_to_shader_type(type_name: String) -> String {
    String::from(match type_name.as_str() {
        "i32" => "Integer",
        "f32" => "Float",
        "f64" => "Double",
        "Vector2" => "FVec2",
        "Vector3" => "FVec3",
        "Vector4" => "FVec4",
        "Matrix4x4" => "FMat4x4",
        _ => panic!("Unknown shader variable type"),
    })
}

fn gather_locations(fields: syn::Fields) -> Vec<(u32, String)> {
    let mut locations = Vec::new();
    for field in fields {
        for attr in field.attrs {
            let ident = attr.path().get_ident();

            if !ident.is_some_and(|i| i.to_string() == "location") {
                continue;
            }

            let v = attr
                .parse_args::<syn::LitInt>()
                .unwrap()
                .base10_parse()
                .unwrap();
            let type_ident = match field.ty {
                syn::Type::Path(ref p) => match p.path.get_ident() {
                    Some(i) => i.to_string(),
                    None => continue,
                },
                _ => continue,
            };

            locations.push((v, type_ident));
        }
    }
    locations
}

#[proc_macro_attribute]
pub fn location(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
