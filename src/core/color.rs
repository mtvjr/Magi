use std::fmt;

use std::collections::HashSet;

#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Copy, Clone, Hash)]
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
    pub fn symbol(self) -> char {
        use Color::*;
        match self {
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

    pub fn colors() -> std::slice::Iter<'static, Color> {
        use Color::*;
        [White, Blue, Black, Red, Green].iter()
    }
}

pub type Colors = HashSet<Color>;

pub trait HasColor {
    fn colors(&self) -> Colors;

    fn is_colored(&self) -> bool {
        !self.colors().is_empty()
    }

    fn is_colorless(&self) -> bool {
        !self.is_colored()
    }

    fn is_multicolored(&self) -> bool {
        self.colors().len() > 1
    }

    fn is_monocolored(&self) -> bool {
        self.colors().len() == 1
    }

    fn is_color(&self, color: Color) -> bool {
        self.colors().contains(&color)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl HasColor for Color {
    fn colors(&self) -> Colors {
        let mut set = Colors::new();
        set.insert(self.clone());
        set
    }

    fn is_colored(&self) -> bool {
        true
    }

    fn is_colorless(&self) -> bool {
        false
    }

    fn is_multicolored(&self) -> bool {
        false
    }

    fn is_monocolored(&self) -> bool {
        true
    }

    fn is_color(&self, color: Color) -> bool {
        *self == color
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
        assert_eq!(Color::White.symbol(), 'W');
        assert_eq!(Color::Blue.symbol(), 'U');
        assert_eq!(Color::Black.symbol(), 'B');
        assert_eq!(Color::Red.symbol(), 'R');
        assert_eq!(Color::Green.symbol(), 'G');
    }
}
