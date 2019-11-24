extern crate ppm;

pub fn main() {
    let canvas = ppm::load("/Users/chris-paterson/Documents/wow.ppm").unwrap();
    let _ = ppm::save(&canvas, "/Users/chris-paterson/Desktop/test.ppm");
}
