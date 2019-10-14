use std::sync::{Once, ONCE_INIT};

use r2d2_redis::r2d2::Pool;
use r2d2_redis::RedisConnectionManager;

use crate::infra::redis::pool::read::RedisReadPool;
use crate::infra::redis::pool::write::RedisWritePool;
use crate::infra::redis::pool::RedisPoolOps;

pub trait RedisContext {
    /// Create the connection pool for reading Redis commands (such as get).
    fn try_get_read_pool(&self) -> RedisReadPool;

    /// Create the connection pool for writing Redis commands (such as set).
    fn try_get_write_pool(&self) -> RedisWritePool;
}

pub trait RedisContextDependencies {}

impl<T> RedisContext for T
where
    T: RedisContextDependencies,
{
    fn try_get_read_pool(&self) -> RedisReadPool {
        unsafe {
            static mut REDIS_POOL: Option<Box<Pool<RedisConnectionManager>>> = None;
            static ONCE: Once = ONCE_INIT;
            ONCE.call_once(|| {
                println!("Start to read-connect Redis!");
                let p = Pool::builder()
                    .build(
                        RedisConnectionManager::new(
                            format!("{}{}", "redis://", "localhost").as_ref(),
                        )
                        .expect("Failed to connect Redis!"),
                    )
                    .expect("Failed to connect Redis!");
                REDIS_POOL = Some(Box::new(p));
            });
            RedisReadPool::new(&REDIS_POOL)
        }
    }

    fn try_get_write_pool(&self) -> RedisWritePool {
        unsafe {
            static mut REDIS_POOL: Option<Box<Pool<RedisConnectionManager>>> = None;
            static ONCE: Once = ONCE_INIT;
            ONCE.call_once(|| {
                println!("Start to write-connect Redis!");
                let p = Pool::builder()
                    .build(
                        RedisConnectionManager::new(
                            format!("{}{}", "redis://", "localhost").as_ref(),
                        )
                        .expect("Failed to connect Redis!"),
                    )
                    .expect("Failed to connect Redis!");
                REDIS_POOL = Some(Box::new(p));
            });
            RedisWritePool::new(&REDIS_POOL)
        }
    }
}

pub trait UsesRedisContext {
    type RedisContext: RedisContext;
    fn redis_context(&self) -> &Self::RedisContext;
}
