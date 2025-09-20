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
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Characters {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Characters {
    /// Get a interator from this Characters to Characters::Z.
    pub const fn iter(&self) -> CharactersIter {
        CharactersIter {
            current: *self,
            exhausted: false,
        }
    }
}

impl std::convert::Into<char> for Characters {
    fn into(self) -> char {
        // Ruthlessly exploits data type fudging and the ASCII table.
        return char::from(65 + self as u8);
    }
}

impl std::convert::TryFrom<char> for Characters {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_ascii_alphabetic() {
            let value: char = value.to_ascii_uppercase();
            return Characters::try_from(value as u8 - 65);
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
            00 => Ok(Self::A),
            01 => Ok(Self::B),
            02 => Ok(Self::C),
            03 => Ok(Self::D),
            04 => Ok(Self::E),
            05 => Ok(Self::F),
            06 => Ok(Self::G),
            07 => Ok(Self::H),
            08 => Ok(Self::I),
            09 => Ok(Self::J),
            10 => Ok(Self::K),
            11 => Ok(Self::L),
            12 => Ok(Self::M),
            13 => Ok(Self::N),
            14 => Ok(Self::O),
            15 => Ok(Self::P),
            16 => Ok(Self::Q),
            17 => Ok(Self::R),
            18 => Ok(Self::S),
            19 => Ok(Self::T),
            20 => Ok(Self::U),
            21 => Ok(Self::V),
            22 => Ok(Self::W),
            23 => Ok(Self::X),
            24 => Ok(Self::Y),
            25 => Ok(Self::Z),
            _ => Err("Characters only map to the values from 0 to 25."),
        }
    }
}

impl std::fmt::Debug for Characters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Characters::{self}")
    }
}

impl std::fmt::Display for Characters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Characters as Into<char>>::into(*self))
    }
}

impl std::ops::Add<Characters> for Characters {
    type Output = Self;

    /// This is a wrapping add.
    fn add(self, other: Self) -> Self::Output {
        return self + other as u8;
    }
}

impl std::ops::Add<u8> for Characters {
    type Output = Self;

    /// This is a wrapping add.
    fn add(self, other: u8) -> Self::Output {
        let mut sum: u8 = self as u8;
        sum += other;
        sum %= 26;
        return Characters::try_from(sum).unwrap();
    }
}

impl std::ops::Sub<Characters> for Characters {
    type Output = Self;

    /// This is a wrapping subtract.
    fn sub(self, other: Self) -> Self::Output {
        return self - other as u8;
    }
}

impl std::ops::Sub<u8> for Characters {
    type Output = Self;

    /// This is a wrapping subtract.
    fn sub(self, other: u8) -> Self::Output {
        let mut difference: u8 = self as u8 + 52;
        difference -= other;
        difference %= 26;
        return Characters::try_from(difference).unwrap();
    }
}

/// All of the test for the Enigma.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_char() {
        let func_ptr: fn(Characters) -> char = <Characters as Into<char>>::into;
        let test_data = [
            (Characters::A, 'A'),
            (Characters::B, 'B'),
            (Characters::C, 'C'),
            (Characters::D, 'D'),
            (Characters::E, 'E'),
            (Characters::F, 'F'),
            (Characters::G, 'G'),
            (Characters::H, 'H'),
            (Characters::I, 'I'),
            (Characters::J, 'J'),
            (Characters::K, 'K'),
            (Characters::L, 'L'),
            (Characters::M, 'M'),
            (Characters::N, 'N'),
            (Characters::O, 'O'),
            (Characters::P, 'P'),
            (Characters::Q, 'Q'),
            (Characters::R, 'R'),
            (Characters::S, 'S'),
            (Characters::T, 'T'),
            (Characters::U, 'U'),
            (Characters::V, 'V'),
            (Characters::W, 'W'),
            (Characters::X, 'X'),
            (Characters::Y, 'Y'),
            (Characters::Z, 'Z'),
        ];
        for (data, expected) in test_data {
            assert_eq!(func_ptr(data), expected);
        }
    }

    #[test]
    fn test_try_from_u8() {
        let test_data: [(u8, Result<Characters, &'static str>); 27] = [
            (00 as u8, Ok(Characters::A)),
            (01 as u8, Ok(Characters::B)),
            (02 as u8, Ok(Characters::C)),
            (03 as u8, Ok(Characters::D)),
            (04 as u8, Ok(Characters::E)),
            (05 as u8, Ok(Characters::F)),
            (06 as u8, Ok(Characters::G)),
            (07 as u8, Ok(Characters::H)),
            (08 as u8, Ok(Characters::I)),
            (09 as u8, Ok(Characters::J)),
            (10 as u8, Ok(Characters::K)),
            (11 as u8, Ok(Characters::L)),
            (12 as u8, Ok(Characters::M)),
            (13 as u8, Ok(Characters::N)),
            (14 as u8, Ok(Characters::O)),
            (15 as u8, Ok(Characters::P)),
            (16 as u8, Ok(Characters::Q)),
            (17 as u8, Ok(Characters::R)),
            (18 as u8, Ok(Characters::S)),
            (19 as u8, Ok(Characters::T)),
            (20 as u8, Ok(Characters::U)),
            (21 as u8, Ok(Characters::V)),
            (22 as u8, Ok(Characters::W)),
            (23 as u8, Ok(Characters::X)),
            (24 as u8, Ok(Characters::Y)),
            (25 as u8, Ok(Characters::Z)),
            (
                47 as u8,
                Err("Characters only map to the values from 0 to 25."),
            ),
        ];
        for (data, wanted) in test_data {
            assert_eq!(Characters::try_from(data), wanted);
        }
    }
}

pub struct CharactersIter {
    current: Characters,
    exhausted: bool,
}

impl std::iter::Iterator for CharactersIter {
    type Item = Characters;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        } else {
            match self.current {
                Characters::A => {
                    self.current = Characters::B;
                    return Some(Characters::A);
                }
                Characters::B => {
                    self.current = Characters::C;
                    return Some(Characters::B);
                }
                Characters::C => {
                    self.current = Characters::D;
                    return Some(Characters::C);
                }
                Characters::D => {
                    self.current = Characters::E;
                    return Some(Characters::D);
                }
                Characters::E => {
                    self.current = Characters::F;
                    return Some(Characters::E);
                }
                Characters::F => {
                    self.current = Characters::G;
                    return Some(Characters::F);
                }
                Characters::G => {
                    self.current = Characters::H;
                    return Some(Characters::G);
                }
                Characters::H => {
                    self.current = Characters::I;
                    return Some(Characters::H);
                }
                Characters::I => {
                    self.current = Characters::J;
                    return Some(Characters::I);
                }
                Characters::J => {
                    self.current = Characters::K;
                    return Some(Characters::J);
                }
                Characters::K => {
                    self.current = Characters::L;
                    return Some(Characters::K);
                }
                Characters::L => {
                    self.current = Characters::M;
                    return Some(Characters::L);
                }
                Characters::M => {
                    self.current = Characters::N;
                    return Some(Characters::M);
                }
                Characters::N => {
                    self.current = Characters::O;
                    return Some(Characters::N);
                }
                Characters::O => {
                    self.current = Characters::P;
                    return Some(Characters::O);
                }
                Characters::P => {
                    self.current = Characters::Q;
                    return Some(Characters::P);
                }
                Characters::Q => {
                    self.current = Characters::R;
                    return Some(Characters::Q);
                }
                Characters::R => {
                    self.current = Characters::S;
                    return Some(Characters::R);
                }
                Characters::S => {
                    self.current = Characters::T;
                    return Some(Characters::S);
                }
                Characters::T => {
                    self.current = Characters::U;
                    return Some(Characters::T);
                }
                Characters::U => {
                    self.current = Characters::V;
                    return Some(Characters::U);
                }
                Characters::V => {
                    self.current = Characters::W;
                    return Some(Characters::V);
                }
                Characters::W => {
                    self.current = Characters::X;
                    return Some(Characters::W);
                }
                Characters::X => {
                    self.current = Characters::Y;
                    return Some(Characters::X);
                }
                Characters::Y => {
                    self.current = Characters::Z;
                    return Some(Characters::Y);
                }
                Characters::Z => {
                    self.exhausted = true;
                    return Some(Characters::Z);
                }
            }
        }
    }
}
