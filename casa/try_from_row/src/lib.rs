#![feature(proc_macro)]
extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::{TokenStream};
use syn::DeriveInput;
use quote::{ToTokens};
use std::str::FromStr;

use syn::punctuated::Iter;

#[proc_macro_derive(TryFromRow)]
pub fn from_row(token_input: TokenStream) -> TokenStream {
    let source: String = token_input.to_string();
    from_row_string(source)
}

fn from_row_string(source: String) -> TokenStream {
    let token_stream = TokenStream::from_str(&source).unwrap();
    let input: DeriveInput = syn::parse(token_stream).unwrap();
    let mut body_string = String::new();
    match input.data {
        syn::Data::Struct(struct_data) => {
            let struct_name = input.ident.to_string();
            let mut struct_declaration = format!("Ok({} {{\n", struct_name);
            for (i, field) in struct_data.fields.iter().enumerate() {
                if i > 0 {
                    struct_declaration.push_str(",\n");
                }
                let column_name = get_column_name(&field);
                let field_name = field.ident.unwrap().to_string();
                let type_name = get_rust_type(&field);
                let is_option = is_optional(&field);
                let variable_declaration = get_var_declaration(&field_name, &column_name,
                    &type_name, is_option);
                body_string.push_str("\n");
                body_string.push_str(&variable_declaration);
                struct_declaration.push_str(&format!("                {}: {}", field_name, field_name));
            }
            struct_declaration.push_str("\n})");
            let impl_string: String = format!("impl TryFromRow for {} {{
                fn try_from_row<'a>(row: &postgres::rows::Row<'a>) -> Result<Self, TryFromRowError> {{
                    // first get the variables
                    {}

                    // then return the crap
                    {}
                }}
            }}", struct_name, body_string, struct_declaration);
            let token_stream = TokenStream::from_str(&impl_string);
            token_stream.unwrap()
        },
        _ => {
            panic!("TryFromRow custom Derive can only be used on Structs");
        }
    }
}

const TRY_FROM_PATH: &'static str = "try_from_column";

fn get_column_name(field: &syn::Field) -> String {
    for attr in field.attrs.iter() {
        let key = attr.path.clone().into_tokens().to_string();
        if key == "column_id" {
            let value_tokens: Vec<_> = attr.tts.clone().into_iter().collect();
            if value_tokens.len() == 1 {
                match &value_tokens[0].kind {
                    &quote::__rt::TokenNode::Group(_, ref name_stream) => {
                        return name_stream.clone().into_tokens().to_string()
                    },
                    _ => panic!("Bad column format")
                }
            }
        }
    }
    for attribute in field.attrs.iter() {
        let path_string = attribute.path.clone().into_tokens().to_string();
        if path_string == TRY_FROM_PATH {
            return attribute.tts.to_string();
        }
    }
    return field.ident.unwrap().to_string();
}

fn get_rust_type(field: &syn::Field) -> String {
    let entire_type_name = field.ty.clone().into_tokens().to_string();
    entire_type_name
}

fn is_optional(field: &syn::Field) -> bool {
    //panic!("Not implemented!");
    match field.ty {
        syn::Type::Path(ref type_path) => {
            let first_segment = &type_path.path.segments[0];
            let segment_name = first_segment.ident.to_string();
            if segment_name == "Option" {
                match first_segment.arguments {
                    syn::PathArguments::AngleBracketed(ref generic_arguments) => {
                        match generic_arguments.args[0] {
                            syn::GenericArgument::Type(ref ty) => {
                                //panic!("This is optional: {}", field.ty.clone().into_tokens().to_string());
                                return true;
                            },
                            _ => false
                        }
                    },
                    _ => false
                }
            } else {
                return false;
            }
        },
        _ => false
    }
}

fn get_var_declaration(field_name: &str, column_name: &str, type_name: &str, is_option: bool) -> String {
    if type_name == "Option < DateTime < Utc > >" {
        return format!("let {0}_match: Option<NaiveDateTime> = row.get(\"{1}\");
            let {0} = {0}_match.map(|d| DateTime::from_utc(d, Utc));", 
            field_name, column_name);
    }
    if is_option {
        return format!("let {0}: {2} = row.get(\"{1}\");", field_name, column_name, type_name);
    }
    if type_name == "DateTime < Utc >" {
        return format!("
            let {0}_match: Option<NaiveDateTime> = row.get(\"{1}\");
            let {0}: DateTime<Utc>;
            if {0}_match.is_none() {{
                return Err(TryFromRowError::new(\"Could not get field '{0}' from column '{1}'\"));
            }}
            {0} = DateTime::from_utc({0}_match.unwrap(), Utc);", field_name, column_name);
    }
    format!("
        let {0}_match: Option<{2}> = row.get(\"{1}\");
        let {0}: {2};
        if {0}_match.is_none() {{
            return Err(TryFromRowError::new(\"Could not get field '{0}' from column '{1}'\"));
        }}
        {0} = {0}_match.unwrap();", field_name, column_name, type_name)
}
