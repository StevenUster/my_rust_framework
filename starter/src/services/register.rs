use crate::{
    AppData, AppError, AppResult, Deserialize, HttpResponse, actix_web::get,
    actix_web::http::header::LOCATION, hash_password, serde_json::json, web,
};

#[derive(Deserialize, Debug)]
pub struct FormData {
    pub email: String,
    pub password: String,
    pub repeat_password: String,
    // you can add a register key here to only allow trusted registers
    // pub register_key: String,
}

#[get("/register")]
pub async fn get(data: web::Data<AppData>) -> HttpResponse {
    data.render("register").await
}

pub async fn post(data: web::Data<AppData>, form: web::Form<FormData>) -> AppResult {
    // Optional
    // if let Some(register_key) = &data.register_key {
    //     if form.register_key != *register_key {
    //         return Ok(data
    //             .render_tpl("register", &json!({"error": "Falscher Register-Schlüssel"}))
    //             .await);
    //     }
    // }

    if form.password.len() < 8 {
        return Ok(data
            .render_tpl(
                "register",
                &json!({"error": "Passwort muss mindestens 8 Zeichen lang sein"}),
            )
            .await);
    }

    if form.password != form.repeat_password {
        return Ok(data
            .render_tpl(
                "register",
                &json!({"error": "Passwörter stimmen nicht überein"}),
            )
            .await);
    }

    let email = form.email.trim().to_lowercase();
    if !email.contains('@') || email.is_empty() {
        return Ok(data
            .render_tpl("register", &json!({"error": "Ungültige E-Mail-Adresse"}))
            .await);
    }

    let hashed_password =
        hash_password(&form.password).map_err(|e| AppError::Internal(e.to_string()))?;

    let user_exists = sqlx::query!("SELECT id FROM users WHERE email = ?", email)
        .fetch_optional(&data.db)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if user_exists.is_some() {
        return Ok(data
            .render_tpl(
                "register",
                &json!({"error": "E-Mail wird bereits verwendet"}),
            )
            .await);
    }

    let _user_id = sqlx::query!(
        "INSERT INTO users (email, password, role) VALUES (?, ?, ?)",
        email,
        hashed_password,
        crate::UserRole::User
    )
    .execute(&data.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
    .last_insert_rowid();

    Ok(HttpResponse::SeeOther()
        .append_header((LOCATION, "/login"))
        .finish())
}
