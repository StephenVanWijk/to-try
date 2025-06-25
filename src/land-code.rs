use std::str::FromStr;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Country {
    AF, AX, AL, DZ, /* ... */ ZW
}

// Marker types for different code formats
pub struct Alpha2;
pub struct Alpha3;
pub struct Numeric;

pub struct CountryCode<T> {
    code: &'static str,
    country: Country,
    _marker: PhantomData<T>,
}

impl<T> CountryCode<T> {
    pub const fn new(code: &'static str, country: Country) -> Self {
        Self {
            code,
            country,
            _marker: PhantomData,
        }
    }
}

// Static lookup tables for each code type
const ALPHA2_CODES: &[CountryCode<Alpha2>] = &[
    CountryCode::new("AF", Country::AF),
    CountryCode::new("AX", Country::AX),
    CountryCode::new("AL", Country::AL),
    // ... all other alpha-2 codes
];

const ALPHA3_CODES: &[CountryCode<Alpha3>] = &[
    CountryCode::new("AFG", Country::AF),
    CountryCode::new("ALA", Country::AX),
    CountryCode::new("ALB", Country::AL),
    // ... all other alpha-3 codes
];

const NUMERIC_CODES: &[CountryCode<Numeric>] = &[
    CountryCode::new("004", Country::AF),
    CountryCode::new("248", Country::AX),
    CountryCode::new("008", Country::AL),
    // ... all other numeric codes
];

impl Country {
    pub fn from_iso_code<T: AsRef<str>>(code: T) -> Option<Self> {
        let code = code.as_ref();
        match code.len() {
            2 => Self::lookup::<Alpha2>(code, ALPHA2_CODES),
            3 => {
                if code.chars().all(|c| c.is_ascii_digit()) {
                    Self::lookup::<Numeric>(code, NUMERIC_CODES)
                } else {
                    Self::lookup::<Alpha3>(code, ALPHA3_CODES)
                }
            },
            _ => None,
        }
    }

    fn lookup<T>(code: &str, table: &[CountryCode<T>]) -> Option<Self> {
        table.iter()
            .find(|cc| cc.code == code)
            .map(|cc| cc.country)
    }

    pub fn name(&self) -> &'static str {
        match self {
            Country::AF => "Afghanistan",
            Country::AX => "Åland Islands",
            // ... all other countries
        }
    }
}

// Example usage
fn main() {
    let country_name = Country::from_iso_code("AL")
        .map(|c| c.name())
        .unwrap_or("Unknown country");
    println!("{}", country_name); // Prints "Albania"
}