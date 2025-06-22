use anyhow::Ok;
use rand::{seq::IndexedRandom};

pub fn process_genpass(length :u8, upper: bool, lower: bool, number: bool, symbol: bool) -> anyhow::Result<()> {
    let mut rng = rand::rng();
    let mut password = String::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(b"ABCDEFGHJKLMNPQRSTUVWXYZ");
    }
    if lower {
        chars.extend_from_slice(b"abcdefghijkmnopqrstuvwxyz");
    }
    if number {
        chars.extend_from_slice(b"023456789");
    }
    if symbol {
        chars.extend_from_slice(b"!@#$%^&*_");
    }
    
    for _ in 0..length {
        let c = chars
            .choose(&mut rng)
            .expect("char s won't be empty in this context");
        password.push(*c as char);
    }

    println!("{}", password);

    Ok(())
}