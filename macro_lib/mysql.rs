use common_uu::{dev_or_prod, string::StringExentd};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Data, Fields};

pub fn db_query(input: TokenStream) -> TokenStream {
    let empty = quote::quote! {};
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let syn::DeriveInput { ident: table_name, data, .. } = input;
    // attrs

    let fields = match data {
        Data::Struct(d) => d.fields,
        _ => return empty.into(),
    };
    let fields = match fields {
        Fields::Named(v) => v,
        _ => return empty.into(),
    };

    let mut get_selfs = vec![];
    let mut tys = vec![];
    let mut table_fields_ident = vec![];
    let mut fields_ident_init = vec![];

    for ele in fields.named {
        let mut table_field_name = ele.ident.unwrap().to_string();
        let ty = ele.ty.to_token_stream();

        let attrs = ele.attrs.iter().find(|v| {
            let path = v.path.to_token_stream().to_string();
            let serde_s = v.tokens.to_token_stream().to_string();
            path.contains("serde") && (serde_s.contains("rename") || serde_s.contains("alias"))
        });

        fields_ident_init.push(quote::format_ident!("{}", table_field_name));

        if let Some(v) = attrs {
            let r = v.tokens.to_string().split_arr(r##"""##);
            let r = r[(r.len() - 2)..(r.len() - 1)].to_vec().join("");
            table_field_name = r;
        }

        get_selfs.push(quote::quote!(#table_field_name));
        table_fields_ident.push(table_field_name);

        tys.push(ty);
    }

    let table_fields_str = table_fields_ident.join(",");

    let code = quote::quote! {
        use mysql_async::prelude::*;

        #[async_trait::async_trait]
        impl orm_uu::mysql::ORMr for #table_name {

            async fn query<'a, 't: 'a, C>(
                comm: C,
                where_sql: &'a str,
                limit: Option<usize>,
            ) -> common_uu::IResult<Vec<Self>>
            where
                Self: Sized,
                C: ToConnection<'a, 't> + 'a,
            {
                let r = where_sql
                    .with(())
                    .map(comm, |(#(#fields_ident_init),*)| Self { #(#fields_ident_init),* })
                    .await?;
                Ok(r)
            }

            async fn query_one<'a, 't: 'a, C>(
                comm: C,
                where_sql: &'a str,
            ) -> common_uu::IResult<Option<Self>>
            where
                Self: Sized,
                C: ToConnection<'a, 't> + 'a,
            {
                let mut r = Self::query(comm, where_sql, Some(1)).await?;
                match r.len(){
                    0 => return Ok(None),
                    1 => return Ok(Some(r.remove(0))),
                    _ => return Err(format!("'{where_sql}' find more row data!", where_sql = where_sql))?,
                }
            }
        }

        };
    code.into()
}
