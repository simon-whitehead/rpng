extern crate rpng;

fn main() {
    match rpng::PngFile::from_path("/Users/Simon/ship.png") {
        Err(error) => println!("Error loading PNG: {:?}", error),
        Ok(_) => println!("Successfully loaded PNG!")
    }
}
