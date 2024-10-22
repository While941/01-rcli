use rand::seq::SliceRandom;

pub fn gen_pass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();

    let mut password = String::new();
    let mut char = Vec::new();

    if upper {
        char.extend_from_slice(b"ABCDEFGHJKLMNPQRSTUVWXYZ");
    }

    if lower {
        char.extend_from_slice(b"abcdefghijkmnopqrstuvwxyz");
    }

    if number {
        char.extend_from_slice(b"123456789");
    }

    if symbol {
        char.extend_from_slice(b"!@#$%^&*_");
    }

    for _ in 0..length {
        let c = char
            .choose(&mut rng)
            .expect("chars won't empty in this context");
        password.push(*c as char);
    }
    println!("{}", password);

    Ok(())
}
