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
    
    let mut id_field_str = String::new();
    let mut id_field_ident = syn::Ident::new_raw("id", proc_macro2::Span::call_site());

    for ele in fields.named {
        let ident = ele.ident.unwrap();
        let mut table_field_name = ident.to_string();
        let ty = ele.ty.to_token_stream();

        let attrs = ele.attrs.iter().map(|x| x.parse_meta().unwrap()).collect::<Vec<_>>();
        for ele in attrs {
            if let syn::Meta::List(path) = &ele{
                let seg = path.path.segments.clone().into_token_stream().to_string();
                let id = path.nested.clone().into_token_stream().to_string();
                if seg == "orm_mysql" && id.trim() == "id"{
                    id_field_str = table_field_name.clone();
                    id_field_ident = ident.clone();
                }
            }
        }

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

    // if id_field_str.is_empty(){
    //     panic!("Please configure #[orm_mysql(id)]");
    // }

    let mut query_quest = vec![];
    for _ in &table_fields_ident {
        query_quest.push("?");
    }

    let query_quest = query_quest.join(",");
    let table_fields_str = table_fields_ident.join(",");
    
    let table_fields_update_str = table_fields_ident.join("=?,");
    let table_fields_update_str = table_fields_update_str.trim_end_matches(",");

    let code = quote::quote! {
    use mysql_async::prelude::*;
    // use orm_mysql::mysql::con_value::*;

    impl From<#struct_name> for mysql_async::Params{
        fn from(#struct_name{ #(#fields_ident_init),* }: #struct_name) -> Self{
            mysql_async::Params::Positional(vec![#(#fields_ident_init .to_value()),*])
        }
    }

    impl mysql_async::prelude::FromRow for #struct_name {
        fn from_row_opt(row: mysql_async::Row) -> Result<Self, mysql_async::FromRowError>
        where Self: Sized,
        {
            let err = mysql_async::FromRowError(row.clone());
            Ok(#struct_name {
                #(#fields_ident_init : row[#table_fields_ident].try_into().map_err(|_|err.clone())? ),*
            })
        }
    }

    #[orm_mysql::async_trait::async_trait]
    impl orm_mysql::mysql::OrmMySqlTrait for #struct_name {

        async fn query<C>(
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

            let r = comm.query(sql).await?;
            Ok(r)
        }

        async fn query_first<C>(
            comm: &mut C,
            where_sql: &str,
        ) -> common_uu::IResult<Option<Self>>
        where
            Self: Sized,
            C: Queryable + Send + Sync,
        {
            let table_name_var = #table_name;
            let mut sql = format!("select {select_sql} from {table_name_var} {where_sql} limit 1",
                select_sql = #table_fields_str,
                table_name_var = table_name_var,
                where_sql = where_sql,
            );
            let r = comm.query_first(sql).await?;
            Ok(r)
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

        async fn update<C>(self, conn: &mut C) -> common_uu::IResult<Option<i64>>
        where
            Self: Sized,
            C: Queryable + Send + Sync
        {
            let sql = format!("update {table_name_var} set ({table_fields})",
                table_name_var = #table_name,
                table_fields = #table_fields_update_str, 
            );
            let r: Option<(i64, )> = conn.exec_first(sql, self).await?;
            let r = r.map(|v|v.0);
            Ok(r)
        }

        async fn delete<C>(&self, conn: &mut C) -> common_uu::IResult<Option<i64>>
        where
            Self: Sized,
            C: Queryable + Send + Sync
        {
            let sql = format!("delete from {table_name_var} where {where_var}={val}",
                table_name_var = #table_name,
                where_var = #id_field_str,
                val = serde_json::json!(self.#id_field_ident).to_string()
            );
            let r: Option<(i64, )> = conn.exec_first(sql, ()).await?;
            let r = r.map(|v|v.0);
            Ok(r)
        }
    }

    };

    // println!("code: {}", code);

    code.into()
}
