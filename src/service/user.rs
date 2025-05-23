use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use serde::Deserialize;
use serde::Serialize;

use crate::db;
use crate::jwt;
use crate::model;
use crate::response::ApiResponse;

#[derive(Deserialize)]
pub struct LoginParams {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Default)]
pub struct Reply {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub avatar: String,
    pub name: String,
    pub token: String,
}

pub async fn login(Json(params): Json<LoginParams>) -> impl IntoResponse {
    let db = db::get();

    let result = model::sys_user::Entity::find()
        .filter(model::sys_user::Column::Username.eq(params.username.clone()))
        .one(db)
        .await;

    match result {
        Ok(user) => {
            if let Some(user) = user {
                if user.password != format!("{:x}", md5::compute(params.password)) {
                    return ApiResponse::error("登陆密码错误", StatusCode::BAD_REQUEST);
                };
                let claims = jwt::Claims::new(user.id, &user.username);

                let reply = Reply {
                    user_id: user.id,
                    username: user.username.clone(),
                    email: user.email,
                    phone: user.phone,
                    avatar: user.avatar,
                    name: user.name.clone(),
                    token: claims.token(),
                };
                ApiResponse::success(reply)
            } else {
                ApiResponse::error("user not exists", StatusCode::BAD_REQUEST)
            }
        }
        Err(e) => {
            tracing::error!(error=%e, "user login failed, db error");
            ApiResponse::error("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
