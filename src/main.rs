use pixel_art::converter::pixelate::{ load_image, save_image, pixelate_image };

fn main() {
    // load image
    let input_image = load_image("./images/new/img.jpg").expect("Failed to load image");

    // convert image
    let output_image = pixelate_image(&input_image, 4);

    // save image
    save_image(&output_image, "./images/pixelated/img.jpg").expect("Failed to save pixel art");
}
