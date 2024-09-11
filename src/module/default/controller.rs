use actix_web::HttpResponse;

pub async fn empty() -> HttpResponse {
   return HttpResponse::Ok().body("Index API - RUST");
}

pub async fn index() -> HttpResponse {
   return HttpResponse::Ok().body("Listo");
}