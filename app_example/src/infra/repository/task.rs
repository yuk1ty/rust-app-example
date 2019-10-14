use crate::domain::entity::task::Task;
use crate::domain::repository::task::TaskRepository;
use crate::infra::mysql::mysqlcon::UsesMySqlConnection;
use crate::infra::redis::pool::RedisPoolOps;
use crate::infra::redis::pool_ctxt::{RedisContext, UsesRedisContext};

pub trait TaskRepositoryDependencies: UsesMySqlConnection + UsesRedisContext {}

impl<T: TaskRepositoryDependencies> TaskRepository for T {
    fn find_task_by_id(&self, id: String) -> Option<Task> {
        let read_pool = self.redis_context().try_get_read_pool();
        let open = read_pool.retrieve_con().is_open();
        let open2 = read_pool.retrieve_con().is_open();
        println!("opened?: {}", open);
        println!("opened2?: {}", open2);

        let write_pool = self.redis_context().try_get_write_pool();
        let open = write_pool.retrieve_con().is_open();
        let open2 = write_pool.retrieve_con().is_open();
        println!("opened?: {}", open);
        println!("opened2?: {}", open2);

        None
    }
}
