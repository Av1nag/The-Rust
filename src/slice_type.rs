pub fn main() {
    let mut value = String::from("Hello alllll");
    let word = first_word(&value);
    print!("{word}");
    value.clear();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
