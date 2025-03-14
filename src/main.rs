use std::fmt::{self, Formatter, Display};

/* Demonstrates printing of a user defined struct using println! macro.*/

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string into a buffer (the first argument)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color
{
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "red: {}, green: {}, blue: {}", self.red, self.green, self.blue)
    }
}


fn main()
{
        for city in
        [
            City { name: "Glassboro", lat: 39.702892, lon: -75.111839 },
            City { name: "Mullica Hill", lat: 39.73928, lon: -75.224072 },
            City { name: "Swedesboro", lat: 39.747616, lon: -75.310463 },
        ]
        .iter()

        {
            println!("{}", *city);
        }


        for color in
        [
            Color { red: 128, green: 255, blue: 90 },
            Color { red: 0, green: 3, blue: 254 },
            Color { red: 0, green: 0, blue: 0 },
        ]

        .iter()


        {
            // Hint : Fix the code so you can print it using {}
            println!("{}", *color);
        }
}

// "color" was originally being printed with {:?} which means that the debug implementation was being used
// If the brackets are left empty, the corresponding argument uses the display trait instead

// std::fmt has a formatting syntax composed of two parts: position or name & formatting
// when there is an absence of both parts, the default is to pick the matching index of {} and to use the display trait

// the parameter's type isn't relevant, but the traits they implement are
// ex: vectors implement debug but not display, which is why {?} has to be used with vectors and not {}
// source: https://stackoverflow.com/questions/40100077/what-is-the-difference-between-printlns-format-styles