fn main() {
    let mut name = (String::from("hello"), String::from("world"));

    let first = &name.0;

    name.1.push_str(".");
    println!("{first} {}", name.1);

    let first = get_first(&name);
    // this method was rejected because compiler just know type signature of function, doesn't know exactly .0 or .1
    // name.1.push_str("!");
    println!("{first} {}", name.1);

    let mut a = [0, 1, 2, 3];
    let x = &mut a[1];
    *x += 1;
    println!("{a:?}");

    let x = &mut a[1];
    let y = &a[2];
    *x += *y;
    println!("{a:?}");
}

fn get_first(name: &(String, String)) -> &String {
    &name.0
}

fn copy_to_prev(v: &mut Vec<i32>, i: usize) {
    let n = &mut v[i];
    *n = v[i - 1];
}
