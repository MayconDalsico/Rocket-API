#[macro_use]
extern crate rocket;

use rocket::serde::{Deserialize, Serialize, json::Json};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Recurso {
    id: u32,
    titulo: String,
    descricao: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Home {
    mensagem: String,
    endpoints: Vec<String>,
}

#[get("/")]
fn home() -> Json<Home> {
    Json(Home {
        mensagem: "Bem-vindo à nossa API!".to_string(),
        endpoints: vec!["/recursos".to_string()],
    })
}

#[get("/recursos")]
fn recurso_index() -> Json<Vec<Recurso>> {
    let recursos = vec![
        Recurso {
            id: 1,
            titulo: "Recurso 1".to_string(),
            descricao: "Descrição do recurso 1".to_string(),
        },
        Recurso {
            id: 2,
            titulo: "Recurso 2".to_string(),
            descricao: "Descrição do recurso 2".to_string(),
        },
    ];
    Json(recursos)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![home, recurso_index])
}
