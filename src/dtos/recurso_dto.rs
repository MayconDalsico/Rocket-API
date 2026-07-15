use rocket::serde::Serialize;


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Recurso {
    pub titulo: String,
    pub descricao: String,
}