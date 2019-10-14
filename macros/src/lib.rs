extern crate proc_macro;
use crate::proc_macro::TokenStream;

use quote::quote;

/// 結局使わずに済んだけど、記念にとってある
///
#[proc_macro_derive(TaskRepository)]
pub fn mixin_task_repository(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl MixInTaskRepository for #name {
            type TaskRepository = Self;

            fn task_repository(&self) -> &Self::TaskRepository {
                &self
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(MySqlConnection)]
pub fn mixin_mysql_connection(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl MixInMySqlConnection for #name {
            type MySqlConnection = Self;

            fn mysql_con(&self) -> &Self::MySqlConnection {
                &self
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(RedisContext)]
pub fn mixin_redis_context(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl MixInRedisContext for #name {
            type RedisContext = Self;

            fn redis_pool_ctxt(&self) -> &Self::RedisContext {
                &self
            }
        }
    };

    TokenStream::from(expanded)
}
