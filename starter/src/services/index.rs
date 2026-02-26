use crate::{AppData, AuthUser, Data, Responder, get};

#[get("/")]
pub async fn index(data: Data<AppData>, _user: AuthUser) -> impl Responder {
    data.render("index").await
}
