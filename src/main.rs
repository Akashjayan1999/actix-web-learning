use std::{clone, fmt::format, path, sync::Mutex};

use actix_web::{
    body::MessageBody, dev::{ServiceRequest, ServiceResponse}, get, guard, http::header::ContentType, middleware::{from_fn, Next}, post, web::{self, Form, Json, Redirect}, App, Error, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder
};
use serde::de;
use serde_json::{self};

#[actix_web::main]
async fn main() {
    let person = Person {
        name: "Tom".to_string(),
        age: 18,
    };
    let mut_pesron = web::Data::new(MutablePerson {
        name: Mutex::new("Tom".to_string()),
        age: Mutex::new(18),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(person.clone()))
            .app_data(mut_pesron.clone())
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("Hello, world!") }),
            )
            // .route("/{name}", web::get().to(|name: web::Path<String>| async move {
            //    HttpResponse::Ok().body(format!("Hello, {}!", name.into_inner()))
            // }))
            .route(
                "/",
                web::post().to(|| async { HttpResponse::Ok().body("POST request!") }),
            )
            .route(
                "/",
                web::delete().to(|| async { HttpResponse::Ok().body("DELETE request!") }),
            )
            .route(
                "/",
                web::put().to(|| async { HttpResponse::Ok().body("PUT request!") }),
            )
            .service(web::redirect("/api/hello2", "/api/world2"))
            .service(
                web::scope("/api")
                    .guard(guard::Get())
                    .route("/hello2", web::get().to(hello2))
                    .route("/world2", web::get().to(world2))
                    .wrap(from_fn(my_middleware)),
            )
            .service(web::scope("/actix").configure(config))
            .service(web::scope("/actix2").configure(config))
            .service(hello)
            .service(world)
            .service(dynamic)
            .service(user)
            .service(postuser)
            .service(hello3)
            .default_service(web::to(not_found))
            .wrap(from_fn(my_middleware))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
    .unwrap();
}

#[get("/hello")]
async fn hello(person: web::Data<Person>) -> impl Responder {
    web::redirect("/hello", "/world")
    //Redirect::to("/world")
    // let msg = format!("name: {}, age: {}", person.name, person.age);
    // HttpResponse::Ok().body(msg)
}

#[get("/world")]
async fn world(person: web::Data<MutablePerson>) -> impl Responder {
    let mut name = person.name.lock().unwrap();
    *name = "Tom".to_string();
    let mut age = person.age.lock().unwrap();
    *age = 18;
    let msg = format!("name: {}, age: {}", name, age);
    HttpResponse::Ok().body(msg)
}

#[get("/use/{id}")]
async fn redirect_dynamic(path: web::Path<i32>) -> impl Responder {
    Redirect::to("/world")
    // HttpResponse::Ok().body(format!("Hello, {}!", path.into_inner()))
}
    
#[get("/use/{id}")]
async fn dynamic(path: web::Path<i32>) -> impl Responder {
   
    HttpResponse::Ok().body(format!("Hello, {}!", path.into_inner()))
}


#[get("/hello/{a:.*}")]
async fn wild_card(req:HttpRequest) -> impl Responder {
    let path = req.match_info().query("a");
    let msg = format!("path: {}", path);
    HttpResponse::Ok().body(msg)
  
}


#[get("/user")]
async fn user(info: web::Query<Info>) -> impl Responder {
    let msg = format!("name: {}, age: {}", info.name, info.age);
    HttpResponse::Ok().body(msg)
}


#[get("/custom-response")]
async fn custom_reponse() -> impl Responder {
    Person {
        name: "Alice".to_string(),
        age: 30,
    }
}

#[post("/user")]
async fn postuser(userItem: Json<User>) -> impl Responder {
    let msg = format!("name: {}, age: {}", userItem.name, userItem.age);
    HttpResponse::Ok().body(msg)
}



#[post("/hello")]
async fn post_form(info: Form<Info>) -> impl Responder {
    let msg = format!("name: {}, age: {}", info.name, info.age);
    HttpResponse::Ok().body(msg)
}




#[get("/hello3")]
async fn hello3() -> impl Responder {
    let person = Person {
        name: "Tom".to_string(),
        age: 18,
    };
    let person_json = serde_json::to_string(&person).unwrap();

    HttpResponse::Ok().json(person_json)
}

async fn hello2(req: HttpRequest) -> impl Responder {
    match req.extensions().get::<Person>() {
        Some(msg) => HttpResponse::Ok().body(format!("name: {}, age: {}", msg.name, msg.age)),
        None => HttpResponse::Ok().body("No data in extensions"),
    }
    // HttpResponse::BadRequest().body("Hello, world2!")
}

async fn world2(req: HttpRequest) -> impl Responder {
    // HttpResponse::InternalServerError().body("Hello, world2!")
    match req.extensions().get::<String>() {
        Some(msg) => HttpResponse::Ok().body(msg.to_string()),
        None => HttpResponse::Ok().body("No data in extensions"),
    }
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("404 Not Found")
}

async fn my_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    println!("Hello from my middleware!");
    //    req.extensions().insert("Some data");
    let person = Person {
        name: "Tom".to_string(),
        age: 18,
    };
    req.extensions_mut()
        .insert("Hello world from middleware".to_string());
     req.extensions_mut()
        .insert(person);
    let res = next.call(req).await?;
    Ok(res)
    //    Ok(req.into_response(HttpResponse::Unauthorized().body("UnAuthrized")))
}

#[derive(serde::Deserialize)]
struct Info {
    name: String,
    age: u8,
}

#[derive(serde::Deserialize)]
struct User {
    name: String,
    age: u8,
}
#[derive(serde::Serialize, Clone)]
struct Person {
    name: String,
    age: u8,
}

impl Responder for Person {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        // Create response and set content type
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

struct MutablePerson {
    name: Mutex<String>,
    age: Mutex<u8>,
}


fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api2")
        .guard(guard::Get())
        .route("/hello2", web::get().to( ||async{HttpResponse::Ok().body("hello2")}))
        .route("/world2", web::get().to(world2))
        .wrap(from_fn(my_middleware)));
    cfg.service(world);
}