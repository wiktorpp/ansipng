//use rgb::*;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <image>", args[0]);
    } else {
        let path = Path::new(&args[1]);

        let mut state = lodepng::Decoder::new();

        match state.decode_file(&path) {
            Ok(image) => match image {
                lodepng::Image::RGBA(bitmap) => {
                    for row in (0..bitmap.height).step_by(2) {
                        for col in 0..bitmap.width {
                            let upper_pixel = bitmap.buffer[(row * bitmap.width) + col];
                            let lower_pixel = bitmap.buffer[((row + 1) * bitmap.width) + col];
                            print!(
                                "\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m▄", 
                                upper_pixel.r, upper_pixel.g, upper_pixel.b, 
                                lower_pixel.r, lower_pixel.g, lower_pixel.b
                            );
                        }
                        print!("\x1b[0m\n");
                    }
                },
                x => println!("Decoded some other image format {:?}", x),
            },
            Err(reason) => println!("Could not load {}, because: {}", path.display(), reason),
        }
    }
}