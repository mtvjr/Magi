use magi::mana::Mana;
use magi::color::Color::*;
use magi::mana::Mana::*;

fn main() {
    let mana = Mana::make_phyrexian(Colored(Green));
    let hybrid = Mana::make_hybrid(Colored(Red), Colored(Blue));

    println!("Hello, world {}, {}!", mana, hybrid);
}
