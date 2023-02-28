use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dogrec::*;

/*
apply in web browser
 */
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, welcome to dog recommendation system!\n")
}

#[get("/recommend")]
async fn recommend() -> impl Responder {
    let dogs = read_csv("data/dog_breeds.csv").unwrap();
    let recommend_dogs = get_recommended_dogs(dogs);
    HttpResponse::Ok().body(recommend_dogs)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running the service");
    HttpServer::new(|| App::new().service(index).service(recommend))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}