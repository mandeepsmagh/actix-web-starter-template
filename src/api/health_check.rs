use actix_web::HttpResponse;

// this end-point can be used by container orchestrator (Kubernetes / Nomad)
// to check and restart application container if the API becomes unresponsive
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
