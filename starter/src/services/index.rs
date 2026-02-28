use crate::{AppData, AuthUser, Data, Responder, get, json};

#[get("/")]
pub async fn index(data: Data<AppData>, user: AuthUser) -> impl Responder {
    data.render_tpl("index", &json!({ "role": user.claims.role.to_string() }))
        .await
}
