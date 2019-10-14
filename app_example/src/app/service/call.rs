use crate::domain::repository::task::{TaskRepository, UsesTaskRepository};
use crate::infra::mysql::mysqlcon::{
    MySqlConnection, MySqlConnectionDependencies, UsesMySqlConnection,
};
use crate::infra::redis::pool_ctxt::{RedisContext, RedisContextDependencies, UsesRedisContext};
use crate::infra::repository::task::TaskRepositoryDependencies;

use crate::{deps, mixin};

pub struct TaskBootstrap {}

impl TaskBootstrap {
    pub fn call(&self) {
        let a = self.task_repository().find_task_by_id("aaa".to_string());
        a;
    }
}

deps!(
    TaskBootstrap,
    TaskRepositoryDependencies,
    RedisContextDependencies,
    MySqlConnectionDependencies
);

mixin!(
    TaskBootstrap,
    TaskRepository,
    UsesTaskRepository,
    task_repository
);
mixin!(
    TaskBootstrap,
    MySqlConnection,
    UsesMySqlConnection,
    mysql_connection
);
mixin!(TaskBootstrap, RedisContext, UsesRedisContext, redis_context);
