use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewUserResponse {
    id: i32,
    username: String,
}

#[post("/new")]
pub async fn new_user() -> Json<NewUserResponse> {
    Json(NewUserResponse {
        id: 1,
        username: "test".to_string(),
    })
}
