use actix_web::{    
    web, App, HttpResponse, HttpServer,
};

#[actix_web::main]
async fn main() {
   HttpServer::new(|| {
      App::new()
         .route("/", web::get().to(|| async { HttpResponse::Ok().body("Hello, world!") }))
         .route("/{name}", web::get().to(|name: web::Path<String>| async move {
            HttpResponse::Ok().body(format!("Hello, {}!", name.into_inner()))
         }))
         .route("/", web::post().to(|| async { HttpResponse::Ok().body("POST request!") }))
         .route("/",web::delete().to(|| async { 
            HttpResponse::Ok().body("DELETE request!") 
         }))
          .route("/",web::put().to(|| async { 
            HttpResponse::Ok().body("PUT request!") 
         }))

   })
   .bind("127.0.0.1:8080").unwrap()
   .run()
   .await.unwrap();
}
