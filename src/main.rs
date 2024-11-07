use regexy::utils::is_match;

fn main() {
    let pattern = r"\d+";
    let text = "There are three apples.";

    if is_match(pattern, text) {
        println!("The text contains digits!");
    } else {
        println!("No digits found in the text.");
    }
}
