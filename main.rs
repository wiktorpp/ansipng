use rgb::*;
use std::path::Path;

fn main() {
    let path = Path::new("test.png");

    let mut state = lodepng::Decoder::new();

    match state.decode_file(&path) {
        Ok(image) => match image {
            lodepng::Image::RGBA(bitmap) => {
                println!("{} x {}", bitmap.width, bitmap.height);
                for pixel in bitmap.buffer {
                    println!("{}", pixel);
                }
            },
            x => println!("Decoded some other image format {:?}", x),
        },
        Err(reason) => println!("Could not load {}, because: {}", path.display(), reason),
    }
}