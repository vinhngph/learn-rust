fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut count = 0;
    let mut begin = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if count == 1 && begin != 0 {
                return &s[begin..i];
            } else {
                count += 1;
                begin = i + 1;
            }
        }
    }

    &s[..]
}

fn main() {
    let a = String::from("Nguyen Van A");
    println!("{}", first_word(&a));
    println!("{}", second_word(&a));
}
