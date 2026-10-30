use crossterm::terminal;
use rand::prelude::IndexedRandom;
use rand::RngExt;

struct SnowFlake {
    symbol: char,
    snow_x: u16,
    y: u16,
}

pub fn main_loop() -> std::io::Result<()> {
    let symbols = ['0', '.', '*', 'o'];
    let mut snowy: Vec<SnowFlake> = Vec::new();
    let mut rng = rand::rng();
    let (x, y) = terminal::size().unwrap_or((80, 24));

    for _ in 0..100 {
        let random_symbol: char = *symbols.choose(&mut rng).unwrap_or(&'*');

        let snow_x = rng.random_range(0..x);
        let snowy_struct = SnowFlake{ symbol: random_symbol, snow_x, y: 0 };

        snowy.push(snowy_struct);
    }
    Ok(())
}