// an_enigma_of_a_site/enigma - A German WW2 Enigma machine.
// Copyright (C) 2025 Robert R. Russell
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, version 3.
//
// This program is distributed in the hope that it will be usefull, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.

/// These are the characters used in the Enigma machine.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Characters {
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    F = 6,
    G = 7,
    H = 8,
    I = 9,
    J = 10,
    K = 11,
    L = 12,
    M = 13,
    N = 14,
    O = 15,
    P = 16,
    Q = 17,
    R = 18,
    S = 19,
    T = 20,
    U = 21,
    V = 22,
    W = 23,
    X = 24,
    Y = 25,
    Z = 26,
}

// impl Characters {
//     /// Not sure if this is useful.
//     fn into_char(&self) -> char {
//         return (*self).into();
//     }
// }

impl std::convert::Into<char> for Characters {
    fn into(self) -> char {
        match self {
            Characters::A => 'A',
            Characters::B => 'B',
            Characters::C => 'C',
            Characters::D => 'D',
            Characters::E => 'E',
            Characters::F => 'F',
            Characters::G => 'G',
            Characters::H => 'H',
            Characters::I => 'I',
            Characters::J => 'J',
            Characters::K => 'K',
            Characters::L => 'L',
            Characters::M => 'M',
            Characters::N => 'N',
            Characters::O => 'O',
            Characters::P => 'P',
            Characters::Q => 'Q',
            Characters::R => 'R',
            Characters::S => 'S',
            Characters::T => 'T',
            Characters::U => 'U',
            Characters::V => 'V',
            Characters::W => 'W',
            Characters::X => 'X',
            Characters::Y => 'Y',
            Characters::Z => 'Z',
        }
    }
}

impl std::convert::Into<u8> for Characters {
    fn into(self) -> u8 {
        return self as u8;
    }
}

impl std::convert::TryFrom<char> for Characters {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_ascii_alphabetic() {
            let value: char = value.to_ascii_uppercase();
            match value {
                'A' => return Ok(Characters::A),
                'B' => return Ok(Characters::B),
                'C' => return Ok(Characters::C),
                'D' => return Ok(Characters::D),
                'E' => return Ok(Characters::E),
                'F' => return Ok(Characters::F),
                'G' => return Ok(Characters::G),
                'H' => return Ok(Characters::H),
                'I' => return Ok(Characters::I),
                'J' => return Ok(Characters::J),
                'K' => return Ok(Characters::K),
                'L' => return Ok(Characters::L),
                'M' => return Ok(Characters::M),
                'N' => return Ok(Characters::N),
                'O' => return Ok(Characters::O),
                'P' => return Ok(Characters::P),
                'Q' => return Ok(Characters::Q),
                'R' => return Ok(Characters::R),
                'S' => return Ok(Characters::S),
                'T' => return Ok(Characters::T),
                'U' => return Ok(Characters::U),
                'V' => return Ok(Characters::V),
                'W' => return Ok(Characters::W),
                'X' => return Ok(Characters::X),
                'Y' => return Ok(Characters::Y),
                'Z' => return Ok(Characters::Z),
                _ => unreachable!(
                    "You managed to escape out of is_ascii_alphabetic at line \
{} in {}", line!(), file!()),
            }
        } else {
            return Err("Characters can only be made from ASCII alphabetic \
 characters.");
        }
    }
}

impl std::convert::TryFrom<u8> for Characters {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            01 => Ok(Self::A),
            02 => Ok(Self::B),
            03 => Ok(Self::C),
            04 => Ok(Self::D),
            05 => Ok(Self::E),
            06 => Ok(Self::F),
            07 => Ok(Self::G),
            08 => Ok(Self::H),
            09 => Ok(Self::I),
            10 => Ok(Self::J),
            11 => Ok(Self::K),
            12 => Ok(Self::L),
            13 => Ok(Self::M),
            14 => Ok(Self::N),
            15 => Ok(Self::O),
            16 => Ok(Self::P),
            17 => Ok(Self::Q),
            18 => Ok(Self::R),
            19 => Ok(Self::S),
            20 => Ok(Self::T),
            21 => Ok(Self::U),
            22 => Ok(Self::V),
            23 => Ok(Self::W),
            24 => Ok(Self::X),
            25 => Ok(Self::Y),
            26 => Ok(Self::Z),
            _ => Err("Characters only map to the values from 1 to 26."),
        }
    }
}

impl std::fmt::Display for Characters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Characters as Into<char>>::into(*self))
    }
}

impl std::ops::Add for Characters {
    type Output = Characters;

    /// This is a wrapping add.
    fn add(self, other: Self) -> Self {
        let x: u8 = self.into();
        let y: u8 = other.into();
        let z: u8 = ((x + y) % 26) + 1;
        Self::try_from(z).unwrap()
    }
}

/// All of the test for the Enigma.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        //todo!();
    }

    #[test]
    fn test_try_from_u8() {
        let test_data:[(u8, Result<Characters, &'static str>); 27] = [
            (1 as u8, Ok(Characters::A)),
            (2 as u8, Ok(Characters::B)),
            (3 as u8, Ok(Characters::C)),
            (4 as u8, Ok(Characters::D)),
            (5 as u8, Ok(Characters::E)),
            (6 as u8, Ok(Characters::F)),
            (7 as u8, Ok(Characters::G)),
            (8 as u8, Ok(Characters::H)),
            (9 as u8, Ok(Characters::I)),
            (10 as u8, Ok(Characters::J)),
            (11 as u8, Ok(Characters::K)),
            (12 as u8, Ok(Characters::L)),
            (13 as u8, Ok(Characters::M)),
            (14 as u8, Ok(Characters::N)),
            (15 as u8, Ok(Characters::O)),
            (16 as u8, Ok(Characters::P)),
            (17 as u8, Ok(Characters::Q)),
            (18 as u8, Ok(Characters::R)),
            (19 as u8, Ok(Characters::S)),
            (20 as u8, Ok(Characters::T)),
            (21 as u8, Ok(Characters::U)),
            (22 as u8, Ok(Characters::V)),
            (23 as u8, Ok(Characters::W)),
            (24 as u8, Ok(Characters::X)),
            (25 as u8, Ok(Characters::Y)),
            (26 as u8, Ok(Characters::Z)),
            (47 as u8, Err("Characters only map to the values from 1 to 26.")),];
        for (data, wanted) in test_data {
            assert_eq!(Characters::try_from(data), wanted);
        }
    }
}
