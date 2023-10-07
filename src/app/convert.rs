pub fn convert_to_ascii(raw_img: image::DynamicImage) -> String {
    use image::GenericImageView;
    let img = raw_img.resize(150, 150, image::imageops::FilterType::CatmullRom);
    let (width, height) = img.dimensions();

    let mut ascii = "".to_string();

    for y in 0..height / 2 {
        for x in 0..width - 1 {
            let pixel = img.get_pixel(x, y * 2);
            let intent = intent(pixel);
            let character = get_ascii_char(intent);
            ascii.push(character);
        }
        ascii.push('\n');
    }

    ascii
}

fn intent(rgb: image::Rgba<u8>) -> u8 {
    match rgb.0[3] == 0 {
        true => 0,
        false => 255 - (rgb.0[0] / 3 + rgb.0[1] / 3 + rgb.0[2] / 3),
    }
}

fn get_ascii_char(intent: u8) -> char {
    let index = intent / 32;
    let ascii = [' ', '.', ',', '-', '~', '+', '=', '@'];
    ascii[index as usize]
}
