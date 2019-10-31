extern crate grok;

use grok::Grok;

fn main() {
    // Instantiate Grok
    let mut grok = Grok::default();

    // Insert a definition which might be a regex or an alias
    grok.insert_definition("USERNAME", r"[a-zA-Z0-9._-]+");

    // Compile the definitions into the pattern you want
    let pattern = grok
        .compile("%{USERNAME}", false)
        .expect("Error while compiling!");

    //  Match the compiled pattern against a string
    match pattern.match_against("root") {
        Some(m) => println!("Found username {:?}", m.get("USERNAME")),
        None => println!("No matches found!"),
    }
}
