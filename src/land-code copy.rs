pub struct Country {
    pub alpha2: String,
    pub alpha3: String,
    pub numeric: u16,
    pub name: String,
    // 20250620 2003 CET SDvW
    // regex' for postal code or postcode.
}

impl Country {
    /// Creates a new Country instance
    pub fn new(alpha2: &str, alpha3: &str, numeric: u16, name: &str) -> Self {
        Self {
            alpha2: alpha2.to_string(),
            alpha3: alpha3.to_string(),
            numeric,
            name: name.to_string(),
        }
    }

    /// Looks up a country by any ISO 3166-1 code (alpha-2, alpha-3, or numeric)
    pub fn from_code(code: &str) -> Option<Self> {
        // Normalize the input (uppercase for letter codes, trim whitespace)
        let code = code.trim().to_uppercase();
        
        // Match against all known country codes
        match code.as_str() {
            // Example countries - you would expand this with the full list
            "AF" | "AFG" | "004" => Some(Country::new("AF", "AFG", 4, "Afghanistan")),
            "AX" | "ALA" | "248" => Some(Country::new("AX", "ALA", 248, "Åland Islands")),
            "AL" | "ALB" | "008" => Some(Country::new("AL", "ALB", 8, "Albania")),
            "DZ" | "DZA" | "012" => Some(Country::new("DZ", "DZA", 12, "Algeria")),
            "US" | "USA" | "840" => Some(Country::new("US", "USA", 840, "United States")),
            "GB" | "GBR" | "826" => Some(Country::new("GB", "GBR", 826, "United Kingdom")),
            "FR" | "FRA" | "250" => Some(Country::new("FR", "FRA", 250, "France")),
            "DE" | "DEU" | "276" => Some(Country::new("DE", "DEU", 276, "Germany")),
            "JP" | "JPN" | "392" => Some(Country::new("JP", "JPN", 392, "Japan")),
            // Add all other countries here...
            _ => None,
        }
    }
}

// Example usage
fn main() {
    // Create from known codes
    let us1 = Country::from_code("US").unwrap();
    let us2 = Country::from_code("USA").unwrap();
    let us3 = Country::from_code("840").unwrap();
    
    assert_eq!(us1.alpha2, "US");
    assert_eq!(us2.alpha3, "USA");
    assert_eq!(us3.numeric, 840);
    
    // Direct instantiation
    let canada = Country::new("CA", "CAN", 124, "Canada");
    println!("{} ({}) - numeric code {}", canada.name, canada.alpha2, canada.numeric);
    
    // Handle unknown codes
    match Country::from_code("XX") {
        Some(country) => println!("Found country: {}", country.name),
        None => println!("Country not found"),
    }
}