use actix_web::HttpResponse;

// cloud native pattern - container orchestrator (Kubernetes / Nomad)
// checks this end-point and restarts application container if the API becomes unresponsive
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
