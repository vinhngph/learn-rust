fn main() {
    let mut n = 1;
    incr(&mut n);
    println!("{n}");
}

fn incr(n: &mut i32) {
    *n += 1;
}
