// Note: Image processing is CPU-intensive.
// The program will run much faster using the `--release` flag.

use clap::{App, Arg};
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Pixel};
use std::process;

fn main() {
    let matches = App::new("Image processor")
        .version("1.0")
        .author("Webber <webber@takken.io>")
        .about("Helps quickly change images")
        .arg(
            Arg::new("input_file")
                .about("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output_file")
                .about("Sets the output file, to write the result")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("verbosity")
                .short('v')
                .multiple_occurrences(true)
                .takes_value(false)
                .about("Sets the level of verbosity"),
        )
        .arg(
            Arg::new("blur")
                .short('b')
                .long("blur")
                .value_name("blur")
                .about("Sets the level of blur from 0 to 100")
                .takes_value(true),
        )
        .arg(
            Arg::new("fractal")
                .short('f')
                .long("fractal")
                .value_name("fractal")
                .about("Uses fractal")
                .takes_value(false),
        )
        .get_matches();

    match matches.occurrences_of("verbosity") {
        0 => println!("Verbose mode is off"),
        1 => println!("Verbose mode in on"),
        2 => println!("Verbose mode is at maximum"),
        _ => println!("Don't be crazy"),
    }

    let mut image: DynamicImage;
    let mut dest_file: &str;

    // You can check the value provided by positional arguments, or option arguments
    if let Some(input_file) = matches.value_of("input_file") {
        println!("File to read: {}", input_file);

        image = image::open(input_file).expect("Failed to open file.");
        println!(
            "Dimensions: width {}, height {}",
            image.dimensions().0,
            image.dimensions().1,
        );
    } else {
        panic!("Input file must be provided.")
    }

    if let Some(output_file) = matches.value_of("output_file") {
        println!("Filename to output: {}", output_file);
        dest_file = output_file;
    } else {
        panic!("Output file must be provided.")
    }

    if let Some(amount) = matches.value_of("blur") {
        let amount = amount
            .parse::<f32>()
            .expect(&format!("Error: {} is not a valid number for blur", amount));

        image = blur(image, amount)
    }

    if matches.is_present("fractal") {
        println!("- Using fractal");

        image = fractal(image);
    }

    image.save(dest_file).expect("Failed writing file.");
}

fn blur(image: DynamicImage, blur_amount: f32) -> DynamicImage {
    if blur_amount > 10.0 || blur_amount < 0.0 {
        println!("Error: Expected blur amount to be between 0.0 and 10.0");
        process::exit(1)
    };

    println!("- Using blur of {:.1}/10.0", blur_amount);

    return image.blur(blur_amount);
}

fn brighten(infile: String, outfile: String) {
    // See blur() for an example of how to open / save an image.

    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.

    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
}

fn crop(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.

    // Challenge: parse the four values from the command-line and pass them
    // through to this function.

    // See blur() for an example of how to save the image.
}

fn rotate(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!

    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.

    // See blur() for an example of how to save the image.
}

fn invert(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.

    // See blur() for an example of how to save the image.
}

fn grayscale(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // .grayscale() takes no arguments. It returns a new image.

    // See blur() for an example of how to save the image.
}

fn generate(outfile: String) {
    // Create an ImageBuffer -- see fractal() for an example

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example

    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    // See blur() for an example of how to save the image
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(image: DynamicImage) -> DynamicImage {
    let mut new_image = image.clone();
    let (width, height) = new_image.dimensions();

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in new_image.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let [red, green, blue] = pixel.data;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    return new_image;
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
