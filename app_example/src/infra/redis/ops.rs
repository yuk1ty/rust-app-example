use crate::infra::redis::pool::RedisPoolOps;
use crate::infra::redis::pool_ctxt::RedisContext;
use crate::infra::redis::pool_ctxt::UsesRedisContext;
use redis::{FromRedisValue, RedisResult, Value};
use std::ops::DerefMut;

pub trait RedisOpsContext {
    fn set(&self, key: impl Into<String>, val: impl Into<String>) -> Option<u8>;
}

pub trait RedisOpsContextDependencies: UsesRedisContext {}

impl<T: RedisOpsContextDependencies> RedisOpsContext for T {
    fn set(&self, key: impl Into<String>, val: impl Into<String>) -> Option<u8> {
        let mut con = self.redis_context().try_get_write_pool().retrieve_con();

        // TODO ???
        //        let set_r = redis::cmd("set")
        //            .arg(&key.into())
        //            .arg(&val.into())
        //            .query(con.deref_mut());
        //        set_r
        //            .map(|_| Some(1 as u8))
        //            .map_err(|_| Some(0 as u8))
        //            .unwrap()
        None
    }
}

pub trait UsesRedisOpsContext {
    type RedisOpsContext: RedisOpsContext;
    fn redis_ops_context(&self) -> &Self::RedisOpsContext;
}
