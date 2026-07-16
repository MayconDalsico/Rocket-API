
use rocket::futures::future::ok;

use crate::models::recurso::Recurso;
use crate::dtos::recurso_dto::RecursoDto;

pub fn lista_de_recursos() -> Vec<Recurso> {

    //buscando os recursos no banco de dados ou em outro lugar
    vec![
        Recurso {
            id: 1,
            titulo: "Recurso 1".to_string(),
            descricao: "Descrição do Recurso 1".to_string(),
        },
        Recurso {
            id: 2,
            titulo: "Recurso 2".to_string(),
            descricao: "Descrição do Recurso 2".to_string(),
        },
    ]
}


pub fn cadastrar_recurso(recurso_dto: RecursoDto) -> Result<Recurso, String> {
    // Lógica para cadastrar o recurso no banco de dados

    println!("Título: {}", recurso_dto.titulo);
    println!("Descrição: {}", recurso_dto.descricao);

    if true {
        Ok( Recurso {id: 1, titulo: recurso_dto.titulo, descricao: recurso_dto.descricao} )
    } else {
        Err("Erro ao cadastrar recurso".to_string())
    }
}