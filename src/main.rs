use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{:?}", *color);
    }
    println!("********************************");

    let x = 5 +  5;
    println!("Is `x` 10 or 100? x = {}", x);

    println!("********************************");

    println!("{} days", 31);

    println!("********************************");

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("********************************");

    println!("{subject}, {verb}, {object}",
    object="the lazy dog",
    subject="the quick brown fox",
    verb="jumps over");

    println!("********************************");

    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    println!("********************************");

    println!("{number:>width$}", number=6, width=20);

    println!("********************************");

    println!("{number:0>width$}", number=2, width=8);

    println!("********************************");

    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    println!("********************************");
}