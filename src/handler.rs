use ntex::web::{self, ServiceConfig};

use crate::image_processing;
use crate::utils::http::ApiResponse;

async fn health_check() -> Result<web::HttpResponse, web::Error> {
    let response = ApiResponse {
        success: true,
        count: None,
        data: Some("Server is running...".to_string()),
        error: None,
    };
    Ok(web::HttpResponse::Ok().json(&response))
}

async fn not_found_error() -> Result<web::HttpResponse, web::Error> {
    let response = ApiResponse {
        success: false,
        count: None,
        data: Some("Not Found".to_string()),
        error: None,
    };
    Ok(web::HttpResponse::NotFound().json(&response))
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/health", web::get().to(health_check))
            .configure(image_processing::routes::init)
            .default_service(web::route().to(not_found_error)),
    );
}
