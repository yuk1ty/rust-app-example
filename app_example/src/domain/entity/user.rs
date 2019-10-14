use crate::domain::vo::user::FullName;

#[derive(Clone)]
pub struct User {
    id: String,
    name: FullName,
}
