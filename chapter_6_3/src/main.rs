fn main() {
    let config_max = Some(3u8);

    println!("{config_max:?}");

    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (), // this is boilerplate code
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}
