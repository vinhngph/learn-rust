fn main() {
    let mut tup: (i32, f64, u8) = (-1, 3.2, 2);

    println!("The pretty print:\n{:#?}", tup);
    println!("{:?}", tup);

    tup.0 = 12;
    tup.1 = -3.3;
    tup.2 = 14;

    println!("The pretty print:\n{:#?}", tup);

    // Array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!(
        "[i32;5] means i32 is a type and 5 is the size of this array: {:?}",
        a
    );

    let b = [3; 5];
    println!(
        "[3;5] means this array has the same 5 numbers of 3: {:?}",
        b
    );
}
