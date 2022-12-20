use common_uu::{dev_or_prod, string::StringExentd};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Data, Fields};


pub fn cache_query_macro(input: TokenStream) -> TokenStream {
    let empty = quote::quote! {};
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let syn::DeriveInput { ident, data, .. } = input;

    /* let attrs = attrs.iter().find(|v| {
        v.path
            .segments
            .iter()
            .find(|v| v.ident.to_string() == "CacheConfig")
            .is_some()
    }); */

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
    // let mut table_fields_ident = vec![];
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
        // table_fields_ident.push(table_field_name);

        tys.push(ty);
    }

    // let table_fields_str = table_fields_ident.join(",");

    let code1 = quote::quote! {
        use redis_async_pool::*;
        use orm_uu::conv_data::*;
        use redis_cluster_async::redis::*;

        impl #ident{
            /// Redis-hget
            pub async fn cache_hget<T: redis_cluster_async::redis::FromRedisValue>(
                pool: &RedisPool,
                k: impl redis_cluster_async::redis::ToRedisArgs + Send + Sync,
            ) -> common_uu::IResult<T> {
                let r = pool
                    .get()
                    .await?
                    .hget(k, (#(#get_selfs),*))
                    .await?;
                Ok(r)
            }
        }
    };

    let code = {
        let mut add = quote::quote! {};
        for i in 0..fields_ident_init.len() {
            let init_ele = &fields_ident_init[i];
            let i_str = i.to_string();
            // let ref ident_i = quote::format_ident!("{}", i);
            let ty_ele = &tys[i];
            add = quote::quote! {
                #add
                let #init_ele: #ty_ele = values[ #i_str.parse::<usize>().unwrap_or_default() ].conv_data();
            };
        }
        add
    };

    let code = quote::quote! {
        #code1

        impl redis_cluster_async::redis::FromRedisValue for #ident {
            fn from_redis_value(
                v: &redis_cluster_async::redis::Value,
            ) -> redis_cluster_async::redis::RedisResult<Self> {
                let values: Vec<Option<String>> = redis_cluster_async::redis::FromRedisValue::from_redis_value(v)?;
                debug!("FromRedisValue: {:?}", values);
                #code
                let r = #ident{
                    #(#fields_ident_init),*
                };
                Ok(r)
            }
        }
    };

    if dev_or_prod!(true, false) {
        // println!(
        //     "生成宏CacheQueryHget代码(仅开发环境打印): {}",
        //     code.to_string()
        // );
    }
    // empty.into()
    code.into()
}



pub fn cache_query_zrange_macro(input: TokenStream) -> TokenStream {
    let _empty = quote::quote! {};
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let syn::DeriveInput { ident, data, .. } = input;

    /* let attrs = attrs.iter().find(|v| {
        v.path
            .segments
            .iter()
            .find(|v| v.ident.to_string() == "CacheConfig")
            .is_some()
    }); */

    let code1 = quote::quote! {
        impl #ident{
            /// Redis-zrange
            pub async fn cache_zrange<T: redis_cluster_async::redis::FromRedisValue>(
                pool: &RedisPool,
                k: impl redis_cluster_async::redis::ToRedisArgs + Send + Sync,
                offset: isize, limit: isize) -> common_uu::IResult<T> {

                let r = pool
                    .get()
                    .await?
                    .zrange(k, offset, limit)
                    .await?;
                Ok(r)
            }
        }
    };

    let fields = match data {
        Data::Struct(d) => d.fields,
        _ => return code1.into(),
    };
    let fields = match fields {
        Fields::Named(v) => v,
        _ => return code1.into(),
    };

    let mut get_selfs = vec![];
    let mut tys = vec![];
    // let mut table_fields_ident = vec![];
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
        // table_fields_ident.push(table_field_name);

        tys.push(ty);
    }

    let code = {
        let mut add = quote::quote! {};
        for i in 0..fields_ident_init.len() {
            let init_ele = &fields_ident_init[i];
            let i_str = i.to_string();
            // let ref ident_i = quote::format_ident!("{}", i);
            let ty_ele = &tys[i];
            add = quote::quote! {
                #add
                let #init_ele: #ty_ele = values[ #i_str.parse::<usize>().unwrap_or_default() ].conv_data();
            };
        }
        add
    };

    let code = quote::quote! {
        #code1

        impl redis_cluster_async::redis::FromRedisValue for #ident {
            fn from_redis_value(
                v: &redis_cluster_async::redis::Value,
            ) -> redis_cluster_async::redis::RedisResult<Self> {
                debug!("FromRedisValue: {:?}", v);
                let values: Vec<Option<String>> = redis_cluster_async::redis::FromRedisValue::from_redis_value(v)?;
                #code
                let r = #ident{
                    #(#fields_ident_init),*
                };
                Ok(r)
            }
        }
    };
    if dev_or_prod!(true, false) {
        // println!(
        //     "生成宏CacheQueryZrange代码(仅开发环境打印): {}",
        //     code.to_string()
        // );
    }
    // empty.into()
    code.into()
}