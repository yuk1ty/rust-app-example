use crate::domain::entity::task::Task;

pub trait TaskRepository {
    fn find_task_by_id(&self, id: String) -> Option<Task>;
}

pub trait UsesTaskRepository {
    type TaskRepository: TaskRepository;
    fn task_repository(&self) -> &Self::TaskRepository;
}
