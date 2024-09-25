use futures::StreamExt;
use image::ImageReader;
use nanoid::nanoid;
use ntex_multipart::Multipart;
use std::{io::Cursor, path::Path};
use webp::Encoder;

pub async fn save_image(mut payload: Multipart) -> Result<(), Box<dyn std::error::Error>> {
    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;

        let mut bytes = vec![];

        while let Some(chunk) = field.next().await {
            bytes.extend_from_slice(&chunk?);
        }
        // 从字节中读取图像
        let image = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()?
            .decode()?;

        // Optionally, resize the existing photo and convert back into DynamicImage
        // let size_factor = 1.0;
        // let image = DynamicImage::ImageRgba8(imageops::resize(
        //     &image,
        //     (image.width() as f64 * size_factor) as u32,
        //     (image.height() as f64 * size_factor) as u32,
        //     imageops::FilterType::Triangle,
        // ));

        // Create the WebP encoder for the above image
        let encoder = Encoder::from_image(&image)?;
        // Encode the image at a specified quality 0-100, Google recommends 75
        let webp = encoder.encode(75f32);
        // Define and write the WebP-encoded file to a given path
        let file_path = Path::new("uploads").join(nanoid!()).with_extension("webp");
        std::fs::write(&file_path, &*webp).unwrap();
    }
    Ok(())
}
