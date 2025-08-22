use std::{clone, fmt::format, path, sync::Mutex};

use actix_web::{    
    get, post, web::{self, Json}, App, HttpResponse, HttpServer, Responder
};
use serde_json::{self};

#[actix_web::main]
async fn main() {
    let person =Person{
        name: "Tom".to_string(),
        age: 18,
    };
    let mut_pesron = web::Data::new(MutablePerson{
        name: Mutex::new("Tom".to_string()),
        age: Mutex::new(18),
    });
   HttpServer::new(move || {
      App::new()
          .app_data(web::Data::new(person.clone()))
          .app_data(mut_pesron.clone())
         .route("/", web::get().to(|| async { HttpResponse::Ok().body("Hello, world!") }))
         // .route("/{name}", web::get().to(|name: web::Path<String>| async move {
         //    HttpResponse::Ok().body(format!("Hello, {}!", name.into_inner()))
         // }))
         .route("/", web::post().to(|| async { HttpResponse::Ok().body("POST request!") }))
         .route("/",web::delete().to(|| async { 
            HttpResponse::Ok().body("DELETE request!") 
         }))
          .route("/",web::put().to(|| async { 
            HttpResponse::Ok().body("PUT request!") 
         })).service(hello)
            .service(world)
            .service(dynamic)
            .service(user)
            .service(postuser)
            .service(hello3)
            .default_service(web::to(not_found))

   })
   .bind("127.0.0.1:8080").unwrap()
   .run()
   .await.unwrap();
}

#[get("/hello")]
async fn hello(person : web::Data<Person>)-> impl Responder {
    let msg = format!("name: {}, age: {}", person.name, person.age);
    HttpResponse::Ok().body(msg)
}

#[get("/world")]
async fn world(person: web::Data<MutablePerson>)-> impl Responder {
    let mut name = person.name.lock().unwrap();
    *name = "Tom".to_string();
    let mut age = person.age.lock().unwrap();
    *age = 18;
    let msg = format!("name: {}, age: {}", name, age);
    HttpResponse::Ok().body(msg)
}

#[get("/use/{id}")]
async fn dynamic(path : web::Path<i32>)-> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}!", path.into_inner()))
}


#[get("/user")]
async fn user(info: web::Query<Info>)-> impl Responder {
   let msg = format!("name: {}, age: {}", info.name, info.age);
    HttpResponse::Ok().body(msg)
}

#[post("/user")]
async fn postuser(userItem: Json<User>)-> impl Responder {
   let msg = format!("name: {}, age: {}", userItem.name, userItem.age);
    HttpResponse::Ok().body(msg)
}

#[get("/hello3")]
async fn hello3()-> impl Responder {
    let person =Person{
        name: "Tom".to_string(),
        age: 18,
    };
    let person_json = serde_json::to_string(&person).unwrap();

    HttpResponse::Ok().json(person_json)
}
#[get("/hello2")]
async fn hello2()-> impl Responder {
    HttpResponse::BadRequest().body("Hello, world2!")
}

#[get("/world2")]
async fn world2()-> impl Responder {
    HttpResponse::InternalServerError().body("Hello, world2!")
}


async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("404 Not Found")
}

#[derive(serde::Deserialize)]
struct Info{
      name: String,
      age: u8,
}

#[derive(serde::Deserialize)]
struct User{
      name: String,
      age: u8,
}
#[derive(serde::Serialize,Clone)]
struct Person{
    name: String,
    age: u8,
}

struct MutablePerson{
    name:Mutex<String>,
    age: Mutex<u8>,
}