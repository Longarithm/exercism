pub fn reverse(input: &str) -> String {
//    unimplemented!("Write a function to reverse {}", input);
    let mut s = String::new(); // String::new(input);
    for c in input.chars().rev() {
        s.push(c);
    }
    return s;
}

// fn main() {
//     println!("{}", reverse("Hello World!"));
// }