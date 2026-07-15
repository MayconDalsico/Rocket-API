
use rocket::futures::future::ok;

use crate::models::recurso::Recurso;

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


pub fn cadastrar_recurso(recurso: Recurso) -> Result<(), String> {
    // Lógica para cadastrar o recurso no banco de dados
    println!("Id: {}", recurso.id);
    println!("Título: {}", recurso.titulo);
    println!("Descrição: {}", recurso.descricao);

    if true {
        Ok(())
    } else {
        Err("Erro ao cadastrar recurso".to_string())
    }
}