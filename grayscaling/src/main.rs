use std::env;
use std::fs::File;
use std::io::{self, Read, Write, Cursor};
use image::{io::Reader as ImageReader, ColorType};

fn main() -> io::Result<()> {
    // Read command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide an input file as an argument.");
        std::process::exit(1);
    }

    let input_filename = &args[1];

    // Read input image file
    let mut file = File::open(input_filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Convert the image to grayscale
    let grayscale_buffer = convert_to_grayscale(&buffer)?;

    // Save the grayscale image
    let mut output_file = File::create("output_image.png")?;
    output_file.write_all(&grayscale_buffer)?;

    println!("Image converted to grayscale successfully!");
    Ok(())
}

fn convert_to_grayscale(buffer: &[u8]) -> io::Result<Vec<u8>> {
    // Load the image from the buffer
    let img = ImageReader::new(Cursor::new(buffer))
        .with_guessed_format()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        .decode()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Convert the image to grayscale
    let gray_img = image::imageops::grayscale(&img);

    // Save the dimensions of the image before converting it to raw bytes
    let (width, height) = (gray_img.width(), gray_img.height());

    // Save the grayscale image back into a buffer
    let mut grayscale_buffer = Cursor::new(Vec::new());
    image::png::PngEncoder::new(&mut grayscale_buffer).encode(
        &gray_img.into_raw(),
        width,
        height,
        ColorType::L8,
    ).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(grayscale_buffer.into_inner())
}
