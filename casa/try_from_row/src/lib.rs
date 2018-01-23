extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;
use quote::ToTokens;

#[proc_macro_derive(TryFromRow)]
pub fn from_row(token_input: TokenStream) -> TokenStream {
    let source = token_input.to_string();
    from_row_string(source)
}

fn from_row_string(source: String) -> TokenStream {
    let input: DeriveInput = syn::parse(source).unwrap();
    let mut body_string = String::new();
    match input.data {
        syn::Data::Struct(struct_data) => {
            let struct_name = input.ident.clone().into_tokens().to_string();
            let mut struct_declaration = format!("Ok({} {{\n", struct_name);
            for field in struct_data.fields.iter() {
                if struct_declaration.len() > 0 {
                    struct_declaration.push_str(",\n");
                }
                let column_name = get_column_name(&field);
                let field_name = field.ident.unwrap().clone().into_tokens().to_string();
                let type_name = get_rust_type(&field);
                let variable_declaration = get_var_declaration(&field_name, &column_name,
                    &type_name);
                body_string.push_str("\n");
                body_string.push_str(&variable_declaration);
                struct_declaration.push_str(&format!("{}: {}", field_name, field_name));
            }
            struct_declaration.push_str("\n})");
            syn::parse(format!("impl TryFromRow for {} {{
                fn try_from_row(row: postgres::rows::Row) -> Result<Self, TryFromRowError> {{
                    // first get the variables
                    {}

                    // then return the crap
                }}
            }}", body_string, struct_declaration)).unwrap()
        },
        _ => {
            panic!("TryFromRow custom Derive can only be used on Structs");
        }
    }
}

const TRY_FROM_PATH: &'static str = "try_from_column";

fn get_column_name(field: &syn::Field) -> String {
    for attribute in field.attrs.iter() {
        let path_string = attribute.path.clone().into_tokens().to_string();
        if path_string == TRY_FROM_PATH {
            return attribute.tts.to_string();
        }
    }
    return field.ident.unwrap().to_string();
}

fn get_rust_type(field: &syn::Field) -> String {
    return field.ty.clone().into_tokens().to_string()
    //panic!("Not implemented!");
}

fn get_var_declaration(field_name: &str, column_name: &str, type_name: &str) -> String {
    format!("let {0}_match: Option<{2}> = row.get(\"{1}\");
        let {0}: {2};
        if {0}_match.is_none() {{
            return Err(TryFromRowError::new(\"Could not get field '{0}' from column '{1}'\"));
        }}
        {0} = {0}_match.unwrap();", field_name, column_name, type_name)
}

#[test]
fn test() {
}
