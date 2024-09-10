use serde::{Deserialize, Serialize};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct Usuario{
    cpf:i32,
    nome:String,
    data_de_nascimento:String,
}


#[get("/exibir")]
async fn exibir(data: web::Data<Mutex<Usuario>>) -> impl Responder {
    let usuario = data.lock().unwrap();
    let variavel = format!("CPF: {}, Nome: {}, Data de Nascimento: {}",usuario.cpf,usuario.nome,usuario.data_de_nascimento);
    HttpResponse::Ok().body(variavel)
}

#[post("/cadastro")]
async fn cadastro(var:web::Json<Usuario> , data: web::Data<Mutex<Usuario>>) -> impl Responder {
    let mut usuario = data.lock().unwrap();
    *usuario = var.into_inner();
    HttpResponse::Ok().body("Usuário criado")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let usuario = web::Data::new(
        Mutex::new(Usuario {
            nome: "Invalido".to_string(),
            cpf: -1,
            data_de_nascimento: "Invalido".to_string(),
        })
    );

    HttpServer::new(move|| {
        App::new().app_data(usuario.clone())
            .service(exibir)
            .service(cadastro)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}