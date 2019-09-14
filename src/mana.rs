use std::fmt;

use crate::color::Color;
use crate::mana::Mana::{Phyrexian, Hybrid};

#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub enum Mana {
    Colored(Color),
    Colorless,
    Generic(u16),
    Phyrexian(Box<Mana>),
    Hybrid(Box<Mana>, Box<Mana>),
}

impl Mana {
    pub fn make_phyrexian(mana: Mana) -> Mana {
        Phyrexian(Box::new(mana))
    }

    pub fn make_hybrid(left_mana: Mana, right_mana: Mana) -> Mana {
        Hybrid(Box::new(left_mana), Box::new(right_mana))
    }

    pub fn to_string(&self) -> String {
        format!("{{{}}}", self.get_symbolic())
    }

    fn get_symbolic(&self) -> String {
        use Mana::*;
        match self {
            Colored(color) => format!("{}", color),
            Colorless => String::from("C"),
            Generic(amount) => format!("{}", amount),
            Phyrexian(mana) => format!("{}/P", mana.get_symbolic()),
            Hybrid(a, b) => format!("{}/{}", a.get_symbolic(),
                                                        b.get_symbolic()),
        }
    }
}

impl fmt::Display for Mana {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

