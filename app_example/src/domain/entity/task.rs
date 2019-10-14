use crate::domain::entity::user::User;
use crate::domain::vo::tag::Tag;
use crate::domain::vo::task::{Description, Period, Title};

pub struct Task {
    id: String,
    title: Title,
    description: Description,
    period: Period, // TODO
    tags: Vec<Tag>,
    created_by: User,
}
