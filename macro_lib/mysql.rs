use common_uu::{string::StringExentd};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Data, Fields};

use crate::utils;

pub fn db_query(input: TokenStream) -> TokenStream {
    let empty = quote::quote! {};
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let syn::DeriveInput {
        attrs,
        ident: struct_name,
        data,
        ..
    } = input;
    // attrs

    let attrs = attrs
        .iter()
        .filter(|v| {
            v.path
                .segments
                .iter()
                .find(|v| v.ident.to_string() == "orm_mysql")
                .is_some()
        })
        .collect::<Vec<_>>();

    let mut table_name = String::new();
    if let Some(attr) = attrs.get(0) {
        let attr = attr.tokens.to_string();
        let attr = attr.trim().trim_start_matches('(').trim_end_matches(')');
        let attrs = attr.split_arr(",");
        for ele in attrs {
            let (k, v) = ele.split_two("=");
            let k = k.trim().trim_end_matches("\"").trim_end_matches("\"");
            let v = v.trim().trim_end_matches("\"").trim_end_matches("\"");
            if k == "table_name" {
                table_name = v.to_string();
            } else {
                panic!("supper attr: {}", v);
            }
        }
    }

    if table_name.is_empty() {
        let name = struct_name.to_string();
        table_name = utils::to_snake_name(&name);
    }

    // println!("table_name: {table_name}");

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

    let mut query_quest = vec![];
    for _ in &table_fields_ident {
        query_quest.push("?");
    }

    let query_quest = query_quest.join(",");
    let table_fields_str = table_fields_ident.join(",");

    let code = quote::quote! {
    use mysql_async::prelude::*;

    impl From<#struct_name> for mysql_async::Params{
        fn from(#struct_name{ #(#fields_ident_init),* }: #struct_name) -> Self{
            mysql_async::Params::Positional(vec![#(#fields_ident_init .to_value()),*])
        }
    }

    #[orm_uu::async_trait::async_trait]
    impl orm_uu::mysql::OrmMySqlTrait for #struct_name {

        async fn query_list<C>(
            comm: &mut C,
            where_sql: &str,
            limit: Option<usize>,
        ) -> common_uu::IResult<Vec<Self>>
        where
            Self: Sized,
            C: Queryable + Send + Sync,
        {

            let table_name_var = #table_name;
            let mut sql = format!("select {select_sql} from {table_name_var} {where_sql}",
                select_sql = #table_fields_str,
                table_name_var = table_name_var,
                where_sql = where_sql,
            );

            if let Some(v) = limit{
                sql.push_str(&format!(" limit {}", v));
            }

            let sql = sql.as_str();

            // println!("sql: {}", sql);

            let r = comm
                .query_map(sql, |(#(#fields_ident_init),*)| Self { #(#fields_ident_init),* })
                .await?;
            Ok(r)
        }

        async fn query<C>(
            comm: &mut C,
            where_sql: &str,
        ) -> common_uu::IResult<Option<Self>>
        where
            Self: Sized,
            C: Queryable + Send + Sync,
        {
            let mut r = Self::query_list(comm, where_sql, Some(1)).await?;
            match r.len(){
                0 => return Ok(None),
                1 => return Ok(Some(r.remove(0))),
                _ => return Err(format!("'{where_sql}' find more row data!", where_sql = where_sql))?,
            }
        }

        async fn insert<C>(self, conn: &mut C) -> common_uu::IResult<Option<i64>>
        where
            Self: Sized,
            C: Queryable + Send + Sync
        {
            let sql = format!("insert into {table_name_var} ({table_fields})values({query_quest})",
                table_name_var = #table_name,
                table_fields = #table_fields_str, 
                query_quest = #query_quest,
            );
            let r: Option<(i64, )> = conn.exec_first(sql, self).await?;
            let r = r.map(|v|v.0);
            Ok(r)
        }
    }

    };
    code.into()
}
