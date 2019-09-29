use std::fmt;

// strum::IntoEnumIterator is required for Color::iter(), but produces unused import warning
#[allow(unused_imports)]
use strum::IntoEnumIterator;

use crate::core::color::{Color, HasColor, Colors};

/// ManaType is used to represent a single type of Mana
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Ord, PartialOrd)]
pub enum ManaType {
    /// X is used to represent a variable amount of mana, chosen by the player
    X,
    /// Snow is any mana generated from a Snow permanent
    Snow,
    /// Generic(num) mana requires num mana of any color to be paid
    Generic(u16),
    /// Colorless mana must be paid with mana without color
    Colorless,
    /// Phyrexian may be paid with the color of mana, or by paying 2 life
    Phyrexian(Color),
    /// Generic hybrid mana may be paid with 2 generic mana, or with the color
    GenericHybrid(Color),
    /// Color hybrid mana may be paid with either of the two given colors
    ColorHybrid(Color, Color),
    /// Colored mana must be paid with mana of the given color
    Colored(Color),
}

impl ManaType {
    /// Formats the mana to the standard format. Eg White mana will be represented as {W}
    pub fn to_string(self) -> String {
        format!("{{{}}}", self.to_symbolic())
    }

    // A helper function used to resolve the symbol in the to_string function
    fn to_symbolic(self) -> String {
        use ManaType::*;
        match self {
            Colored(color) => color.symbol().to_string(),
            Colorless => String::from("C"),
            Snow => String::from("S"),
            X => String::from("X"),
            Generic(amount) => format!("{}", amount),
            Phyrexian(color) => format!("{}/P", color),
            GenericHybrid(color) => format!("2/{}", color),
            ColorHybrid(a, b) => format!("{}/{}", a, b)
        }
    }

    /// Returns the converted mana cost value of the mana element
    pub fn cmc(self) -> u16 {
        use ManaType::*;
        match self {
            GenericHybrid(_c) => 2,
            Generic(val) => val,
            X => 0,
            _ => 1
        }
    }
}

impl HasColor for ManaType {
    fn colors(&self) -> Colors {
        let mut colors = Colors::new();

        match self {
            ManaType::Colored(color) => {colors.insert(color.clone());},
            ManaType::Phyrexian(color) => {colors.insert(color.clone());},
            ManaType::GenericHybrid(color) => {colors.insert(color.clone());},
            ManaType::ColorHybrid(c1, c2) => {
                colors.insert(c1.clone());
                colors.insert(c2.clone());
            }
            _ => (),
        }

        colors
    }
}

impl fmt::Display for ManaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod mana_test {
    use super::*;
    use crate::core::color::Color::*;

    #[test]
    fn color_test() {
        use ManaType::*;

        let mut colored_mana = Vec::new();
        let mut phyrexian_mana = Vec::new();
        let mut generic_hybrid_mana = Vec::new();
        let mut color_hybrid_mana = Vec::new();
        let mut generic_mana = Vec::new();

        for color in Color::iter() {
            colored_mana.push(Colored(color));
            phyrexian_mana.push(Phyrexian(color));
            generic_hybrid_mana.push(GenericHybrid(color));

            for c2 in Color::iter() {
                if c2 != color
                {
                    color_hybrid_mana.push(ColorHybrid(color, c2));
                }
            }
        }

        for num in 1..20 {
            generic_mana.push(Generic(num));
        }

        let mut all_mana = vec![Colorless, Snow, X];
        all_mana.extend_from_slice(colored_mana.as_slice());
        all_mana.extend_from_slice(phyrexian_mana.as_slice());
        all_mana.extend_from_slice(generic_hybrid_mana.as_slice());
        all_mana.extend_from_slice(color_hybrid_mana.as_slice());
        all_mana.extend_from_slice(generic_mana.as_slice());

        // Not equal assertions
        for mana in all_mana {
            assert_ne!(mana.is_colored(), mana.is_colorless())
        }

        for mana in colored_mana {
            assert_eq!(mana.is_colored(), true);
            assert_eq!(mana.is_colorless(), false);
            assert_eq!(mana.is_monocolored(), true);
            assert_eq!(mana.is_multicolored(), false);
        }

        for mana in generic_hybrid_mana {
            assert_eq!(mana.is_colored(), true);
            assert_eq!(mana.is_colorless(), false);
            assert_eq!(mana.is_monocolored(), true);
            assert_eq!(mana.is_multicolored(), false);
        }

        for mana in color_hybrid_mana {
            assert_eq!(mana.is_colored(), true);
            assert_eq!(mana.is_colorless(), false);
            assert_eq!(mana.is_monocolored(), false);
            assert_eq!(mana.is_multicolored(), true);
        }

        for mana in generic_mana {
            assert_eq!(mana.is_colored(), false);
            assert_eq!(mana.is_colorless(), true);
            assert_eq!(mana.is_monocolored(), false);
            assert_eq!(mana.is_multicolored(), false);
        }

        assert_eq!(Colorless.is_colored(), false);
        assert_eq!(Colorless.is_colorless(), true);
        assert_eq!(Colorless.is_monocolored(), false);
        assert_eq!(Colorless.is_multicolored(), false);

        assert_eq!(Snow.is_colored(), false);
        assert_eq!(Snow.is_colorless(), true);
        assert_eq!(Snow.is_monocolored(), false);
        assert_eq!(Snow.is_multicolored(), false);

        assert_eq!(X.is_colored(), false);
        assert_eq!(X.is_colorless(), true);
        assert_eq!(X.is_monocolored(), false);
        assert_eq!(X.is_multicolored(), false);
    }

    #[test]
    fn to_string() {
        use ManaType::*;

        assert_eq!(Colored(White).to_string(), String::from("{W}"));
        assert_eq!(ColorHybrid(Blue, Green).to_string(), String::from("{U/G}"));
        assert_eq!(GenericHybrid(Black).to_string(), String::from("{2/B}"));
        assert_eq!(Phyrexian(Red).to_string(), String::from("{R/P}"));
    }

    #[test]
    fn cmc() {
        use ManaType::*;

        assert_eq!(ColorHybrid(Blue, Black).cmc(), 1);
        assert_eq!(Colorless.cmc(), 1);
        assert_eq!(Phyrexian(Green).cmc(), 1);
        assert_eq!(Colored(White).cmc(), 1);
        assert_eq!(GenericHybrid(Red).cmc(), 2);

        for num in 1..20 {
            assert_eq!(Generic(num).cmc(), num);
        }
    }
}
