#![recursion_limit="256"]
extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;
use proc_macro::TokenStream;


#[proc_macro_derive(SqlId)]
pub fn sql_id_derive(input: TokenStream) -> TokenStream {
    let input_string = input.to_string();
    let ast = syn::parse_derive_input(&input_string).unwrap();
    let impl_tokens = impl_sql_traits(&ast);
    impl_tokens.parse().unwrap()
}

fn impl_sql_traits(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        use serde;
        use serde::ser::Serialize;
        use serde::de::Deserialize;

        type ToSqlResult = Result<IsNull, Box<std::error::Error + 'static + Send + Sync>>;

        impl ToSql for #name {
            fn to_sql(&self, ty: &Type, out: &mut Vec<u8>) -> ToSqlResult {
                self.0.to_sql(ty, out)
            }

            fn accepts(ty: &Type) -> bool
            where
                Self: Sized,
            {
                true
            }

            fn to_sql_checked(&self, ty: &Type, out: &mut Vec<u8>) -> ToSqlResult {
                self.0.to_sql_checked(ty, out)
            }
        }

        impl FromSql for #name {
            fn from_sql(
                ty: &Type,
                raw: &[u8],
            ) -> Result<Self, Box<std::error::Error + 'static + Send + Sync>> {
                let id = try!(i32::from_sql(ty, raw));
                Ok(#name(id))
            }

            fn accepts(ty: &Type) -> bool {
                true
            }

            fn from_sql_null(ty: &Type) -> Result<Self, Box<std::error::Error + 'static + Send + Sync>> {
                let id = try!(i32::from_sql_null(ty));
                Ok(#name(id))
            }

            fn from_sql_nullable(
                ty: &Type,
                raw: Option<&[u8]>,
            ) -> Result<Self, Box<std::error::Error + 'static + Send + Sync>> {
                let id = try!(i32::from_sql_nullable(ty, raw));
                Ok(#name(id))
            }
        }

        use serde::Serializer;
        impl Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
                serializer.serialize_str(&self.0.to_string())
            }
        }

        use serde::Deserializer;
        impl<'de> Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
                let int_val = i32::deserialize(deserializer).unwrap();
                Ok(#name(int_val))
            }
        }
    }
}
