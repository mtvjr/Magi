use std::fmt;

use std::collections::HashSet;

// strum::IntoEnumIterator is required for EnumIter, but produces unused import warning
#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Color is one of the intrisic parts of the game of Magic. This enum is used to designate
/// a single color.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, EnumIter, Ord, PartialOrd)]
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
    /// Returns the letter used to represent the color
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

    /// Returns the color represented by a letter
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

/// A collection of colors. The Colors type is an alias of std::collection::HashSet<Color>
/// that implements the HasColor trait
pub type Colors = HashSet<Color>;

/// This trait should be implemented on any object that can be considered colored as it gives
/// access to many common color checks that appear in the magic rules.
pub trait HasColor {
    /// The colors() method returns a set of colors that the implementor has. The set is a
    /// std::collections::HashSet<Color> aliased to Colors.
    fn colors(&self) -> Colors;

    /// The is_colored() method returns true if the object has at least one color.
    fn is_colored(&self) -> bool {
        !self.colors().is_empty()
    }

    /// The is_colorless() method returns true if the object has no colors
    fn is_colorless(&self) -> bool {
        !self.is_colored()
    }

    /// The is_multicolored() method returns true if the object has more than one color.
    fn is_multicolored(&self) -> bool {
        self.colors().len() > 1
    }

    /// The is_multicolored() method returns true if the object has only one color.
    fn is_monocolored(&self) -> bool {
        self.colors().len() == 1
    }

    /// The is_multicolored() method returns true if the object has the provided color
    fn is_color(&self, color: Color) -> bool {
        self.colors().contains(&color)
    }
}

impl HasColor for Colors {
    fn colors(&self) -> Colors {
        return self.clone()
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

    #[test]
    fn color_iter() {
        let mut colors = Color::iter();

        assert_eq!(colors.next().unwrap(), Color::White);
        assert_eq!(colors.next().unwrap(), Color::Blue);
        assert_eq!(colors.next().unwrap(), Color::Black);
        assert_eq!(colors.next().unwrap(), Color::Red);
        assert_eq!(colors.next().unwrap(), Color::Green);
        assert_eq!(colors.next(), None);
    }

    #[test]
    fn color_ord() {
        assert_eq!(Color::White < Color::Blue, true);
        assert_eq!(Color::Blue < Color::Black, true);
        assert_eq!(Color::Black < Color::Red, true);
        assert_eq!(Color::Red < Color::Green, true);
        assert_eq!(Color::Green < Color::White, false);

        assert_eq!(Color::White <= Color::Blue, true);
        assert_eq!(Color::Blue <= Color::Black, true);
        assert_eq!(Color::Black <= Color::Red, true);
        assert_eq!(Color::Red <= Color::Green, true);
        assert_eq!(Color::Green <= Color::White, false);

        assert_eq!(Color::White == Color::White, true);
        assert_eq!(Color::Blue == Color::Blue, true);
        assert_eq!(Color::Black == Color::Black, true);
        assert_eq!(Color::Red == Color::Red, true);
        assert_eq!(Color::Green == Color::Green, true);
    }
}
