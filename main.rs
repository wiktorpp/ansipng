use rgb::*;
use std::path::Path;

fn main() {
    let path = Path::new("test.png");

    let mut state = lodepng::Decoder::new();

    match state.decode_file(&path) {
        Ok(image) => match image {
            lodepng::Image::RGBA(bitmap) => {
                println!("{} x {}", bitmap.width, bitmap.height);
                println!("{:?}", bitmap.buffer);
                for pixel in bitmap.buffer {
                    print!(
                        "\x1b[38;2;{};{};{}m", 
                        pixel.r, pixel.g, pixel.b
                    );
                    print!(
                        "\x1b[48;2;{};{};{}m", 
                        pixel.r, pixel.g, pixel.b
                    );
                    print!("▄")
                }
            },
            x => println!("Decoded some other image format {:?}", x),
        },
        Err(reason) => println!("Could not load {}, because: {}", path.display(), reason),
    }
}