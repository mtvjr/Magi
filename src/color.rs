use std::fmt;

#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

const WHITE_SYMBOL: char = 'W';
const BLUE_SYMBOL:  char = 'U';
const BLACK_SYMBOL: char = 'B';
const RED_SYMBOL:   char = 'R';
const GREEN_SYMBOL: char = 'G';

impl Color {
    pub fn get_symbol(color: Color) -> char {
        use Color::*;
        match color {
            White => WHITE_SYMBOL,
            Blue  => BLUE_SYMBOL,
            Black => BLACK_SYMBOL,
            Red   => RED_SYMBOL,
            Green => GREEN_SYMBOL
        }
    }

    pub fn from_symbol(symbol: char) -> Option<Color> {
        use Color::*;
        match symbol {
            WHITE_SYMBOL => Some(White),
            BLUE_SYMBOL  => Some(Blue),
            BLACK_SYMBOL => Some(Black),
            RED_SYMBOL   => Some(Red),
            GREEN_SYMBOL => Some(Green),
            _ => None
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Color::get_symbol(*self))
    }
}

#[cfg(test)]
mod color_test {
    use super::*;

    #[test]
    fn from_symbol() {
        assert_eq!(Some(Color::White), Color::from_symbol('W'));
        assert_eq!(Some(Color::Blue), Color::from_symbol('U'));
        assert_eq!(Some(Color::Black), Color::from_symbol('B'));
        assert_eq!(Some(Color::Red), Color::from_symbol('R'));
        assert_eq!(Some(Color::Green), Color::from_symbol('G'));
        assert_eq!(None, Color::from_symbol('?'));
    }

    #[test]
    fn to_symbol() {
        assert_eq!(Color::get_symbol(Color::White), 'W');
        assert_eq!(Color::get_symbol(Color::Blue), 'U');
        assert_eq!(Color::get_symbol(Color::Black), 'B');
        assert_eq!(Color::get_symbol(Color::Red), 'R');
        assert_eq!(Color::get_symbol(Color::Green), 'G');
    }
}
