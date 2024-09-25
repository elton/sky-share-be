use crate::utils::http::ApiResponse;
use futures::StreamExt;
use ntex::web;
use ntex_multipart::Multipart;
use std::{fs::create_dir_all, path::Path};
use tokio::io::AsyncWriteExt;

// 接收前端通过POST提交的form-data中传入的图片文件，并保存到本地文件夹
pub async fn handle_upload_image(mut payload: Multipart) -> Result<web::HttpResponse, web::Error> {
    // 确保uploads目录存在
    let upload_dir = Path::new("uploads");
    if !upload_dir.exists() {
        create_dir_all(upload_dir).unwrap(); // 创建目录
    }

    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;

        let file_name = field
            .headers()
            .get("Content-Disposition")
            .and_then(|hd| hd.to_str().ok())
            .and_then(|hd| {
                hd.split(';').find_map(|s| {
                    let trimmed = s.trim();
                    if trimmed.starts_with("filename=") {
                        Some(trimmed.trim_start_matches("filename=").trim_matches('"'))
                    } else {
                        None
                    }
                })
            })
            .unwrap_or("default_filename.jpg");

        let file_path = Path::new("uploads").join(file_name);
        let mut file = tokio::fs::File::create(&file_path).await.unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file.write_all(&data).await.unwrap();
        }
    }

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
