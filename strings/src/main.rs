fn main() {
    let mut some_string: String = String::from("Hi, how are you?");
    println!("String size: {}", some_string.len());
    println!("Is the String empty? {}", some_string.is_empty());

    for token in some_string.split_whitespace() {
        println!("{}", token);
    }

    println!("Is there a name, like John, on the String? {}",
             some_string.contains("John"));

    some_string.push_str(" I am fine, thank you.");
    println!("{}", some_string);
}
