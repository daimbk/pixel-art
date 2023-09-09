use image::DynamicImage;

pub fn load_image(file_path: &str) -> Result<DynamicImage, image::ImageError> {
    image::open(file_path)
}

pub fn save_image(image: &DynamicImage, file_path: &str) -> Result<(), image::ImageError> {
    image.save(file_path)
}

pub fn pixelate(image: &DynamicImage, pixel_size: u32) -> DynamicImage {
    image.resize_exact(
        image.width() / pixel_size,
        image.height() / pixel_size,
        image::imageops::FilterType::Nearest,
    )
}
