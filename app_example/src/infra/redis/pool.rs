use r2d2_redis::r2d2::PooledConnection;
use r2d2_redis::RedisConnectionManager;

pub trait RedisPoolOps {
    fn retrieve_con(&self) -> PooledConnection<RedisConnectionManager>;
}

pub mod read {
    use crate::infra::redis::pool::RedisPoolOps;
    use r2d2_redis::r2d2::{Pool, PooledConnection};
    use r2d2_redis::RedisConnectionManager;

    pub struct RedisReadPool<'r>(pub &'r Box<Pool<RedisConnectionManager>>);

    impl<'r> RedisReadPool<'r> {
        pub fn new(pool: &Option<Box<Pool<RedisConnectionManager>>>) -> RedisReadPool {
            let s = match pool {
                Some(p) => p,
                None => panic!("there is no pool!"),
            };
            RedisReadPool(s)
        }
    }

    impl<'r> RedisPoolOps for RedisReadPool<'r> {
        fn retrieve_con(&self) -> PooledConnection<RedisConnectionManager> {
            self.0.get().expect("failed to retrieve read connection")
        }
    }
}

pub mod write {
    use crate::infra::redis::pool::RedisPoolOps;
    use r2d2_redis::r2d2::{Pool, PooledConnection};
    use r2d2_redis::RedisConnectionManager;

    pub struct RedisWritePool<'w>(pub &'w Box<Pool<RedisConnectionManager>>);

    impl<'w> RedisWritePool<'w> {
        pub fn new(pool: &Option<Box<Pool<RedisConnectionManager>>>) -> RedisWritePool {
            let s = match pool {
                Some(p) => p,
                None => panic!("there is no pool!"),
            };
            RedisWritePool(s)
        }
    }

    impl<'w> RedisPoolOps for RedisWritePool<'w> {
        fn retrieve_con(&self) -> PooledConnection<RedisConnectionManager> {
            self.0.get().expect("failed to retrieve write connection")
        }
    }
}
