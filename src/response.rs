use serde::Serialize;
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: Uuid,
    pub username: String,
    pub createdAt: Option<chrono::DateTime<chrono::Utc>>,
    pub updatedAt: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub user: FilteredUser,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}
