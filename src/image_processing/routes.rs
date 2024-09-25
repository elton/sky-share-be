use crate::utils::{http::ApiResponse, image};
use ntex::web;
use ntex_multipart::Multipart;
use std::{fs::create_dir_all, path::Path};

// 接收前端通过POST提交的form-data中传入的图片文件，并保存到本地文件夹
pub async fn handle_upload_image(payload: Multipart) -> Result<web::HttpResponse, web::Error> {
    // 确保uploads目录存在
    let upload_dir = Path::new("uploads");
    if !upload_dir.exists() {
        create_dir_all(upload_dir)?; // 创建目录
    }

    image::save_image(payload).await.unwrap();

    Ok(web::HttpResponse::Created().json(&ApiResponse::<_> {
        success: true,
        count: None,
        data: Some("Image uploaded successfully".to_string()),
        error: None,
    }))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/image_processing").route("/upload", web::post().to(handle_upload_image)),
    );
}
