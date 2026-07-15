use crate::model_views::home::Home;
use rocket::serde::json::Json;

#[get("/")]
pub fn index() -> Json<Home> {
    Json(Home {
        mensagem: "Bem-vindo à nossa API!".to_string(),
        endpoints: vec!["/recursos".to_string()],
    })
}
