use crate::models::recurso::Recurso;
use rocket::serde::json::Json;
use crate::servicos::recurso_servico;
use crate::dtos::recurso_dto::RecursoDto;

#[get("/recursos")]
pub fn index() -> Json<Vec<Recurso>> {
    let recursos = recurso_servico::lista_de_recursos();
    Json(recursos)
}


#[post("/recursos", data = "<recurso_dto>")]
pub fn criar(recurso_dto_: Json<RecursoDto>) -> Json<Vec<Recurso>> {
    let recurso_dto_obj = recurso_dto.into_inner();
    match recurso_servico::cadastrar_recurso(recurso){
        Ok(recurso)
        Json(recurso)
    }else{

    }

}
