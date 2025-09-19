use image::{DynamicImage, GenericImageView, ImageError, imageops::resize};
use std::{fs, path::Path};
use tokio; // 使用tokio异步运行时

/// 图片尺寸批量调整器
///
/// 此函数接收一个包含图片路径的切片和目标尺寸，异步地调整每个图片的尺寸。
///
/// # 参数
/// * `paths` - 图片路径的切片
/// * `target_width` - 目标宽度
/// * `target_height` - 目标高度
///
/// # 返回值
/// 一个Result类型，包含成功或失败信息
#[tokio::main]
async fn main() -> Result<(), ImageError> {
    // 示例图片路径和目标尺寸
    let paths = vec![
        "./images/image1.jpg",
        "./images/image2.png",
    ];
    let target_width = 800;
    let target_height = 600;

    // 遍历图片路径
    for path in paths {
        // 异步地读取和调整图片尺寸
        tokio::spawn(async move {
            if let Err(e) = resize_image(&path, target_width, target_height).await {
                eprintln!("Error resizing image {}: {}", path, e);
            }
        })
        .await
        .expect("Failed to join task");
    }

    Ok(())
}

/// 异步调整单个图片的尺寸
///
/// # 参数
/// * `path` - 图片的路径
/// * `target_width` - 目标宽度
/// * `target_height` - 目标高度
///
/// # 返回值
/// 一个Result类型，包含成功或失败信息
async fn resize_image(path: &str, target_width: u32, target_height: u32) -> Result<(), ImageError> {
    // 读取图片
    let img = image::open(path)?;

    // 调整图片尺寸
    let resized_img = resize(&img, target_width, target_height, image::imageops::FilterType::Nearest);

    // 保存调整后的图片
    let output_path = format!("./resized_{}", path);
    fs::write(output_path, resized_img.into_rgba8().as_raw()).map_err(|e| e.into())
}
