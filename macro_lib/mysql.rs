use common_uu::{dev_or_prod, string::StringExentd};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Data, Fields};


pub fn db_query(input: TokenStream) -> TokenStream {
    let empty = quote::quote! {};
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let syn::DeriveInput { ident, data, .. } = input;
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

        use orm_uu::conv_data::*;

        impl #ident{

            // 查询字段
            // pub fn fields() -> Vec<String> {
            //     return vec![#(#get_selfs .to_string()),*];
            // }

            /// 数据库查询方法
            pub async fn db_query<T: ToString>(session: &mysql_async::GetConn, where_sql: String, where_in_vars: impl Into<VecInto<T>>, limit_v: Option<isize>) -> common_uu::IResult<Vec<Self>> {

                let ref where_in_vars = where_in_vars.into().0;

                // 拼接SQL
                let table = Self::table_name();
                let mut cql = format!(
                    "SELECT {fields} from {table} {where_}",
                    fields = #table_fields_str,
                    table = table,
                    where_ = where_sql
                );

                if let Some(limit_var) = limit_v{
                    cql.push_str(&format!(" limit {}", limit_var));
                }

                let mut r_rows = vec![];

                if !where_in_vars.is_empty(){
                    let mut i = 0;
                    debug!("db_query in where_in_vars.len: {}", where_in_vars.len());
                    for where_sql in where_in_vars.split_inclusive(|_| {
                        i += 1;
                        i % 100 == 0
                    }){
                        debug!("db_query in var ele.len: {}", where_sql.len());

                        // 带wherein条件的情况
                        use orm_uu::scylladb::ScyllaQuery;
                        let query = ScyllaQuery::from(cql.clone()).wherein2(where_sql);

                        debug!("cql: {}", query.contents);
                        let mut rows = session.query(query, &[]).await?.rows()?;
                        r_rows.append(&mut rows);
                    }
                }else{
                    use orm_uu::scylladb::ScyllaQuery;
                    let query = ScyllaQuery::from(cql.clone()).query;
                    debug!("cql: {}", query.contents);
                    let mut rows = session.query(query, &[]).await?.rows()?;
                    r_rows.append(&mut rows);
                }

                // 使用宏生成返回类型
                let mut r_arr = vec![];
                for item in r_rows{
                    debug!("item: {:?}", item);
                    let ( #(#fields_ident_init),* ) = match item.into_typed::<(#(#tys),*)>(){
                        Err(e) => {
                            error!("into_typed: {:?} \n", e);
                            continue;
                        }
                        Ok(v) => v,
                    };
                    r_arr.push(#ident{ #(#fields_ident_init),* });
                }

                Ok(r_arr)
            }
        }
    };
    if dev_or_prod!(true, false) {
        // println!("生成宏DbQuery代码(仅开发环境打印): {}", code.to_string());
    }
    // empty.into()
    code.into()
}