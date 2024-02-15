#[cfg(feature = "broken")]
const CAZZO: &[&str] = &["ciao"];

#[cfg(not(feature = "broken"))]
const CAZZO: &[&str] = &[0];

fn main() {
    for word in CAZZO {
        println!("{word}");
    }
}
