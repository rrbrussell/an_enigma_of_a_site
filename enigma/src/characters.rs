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
#[derive(Clone, Copy)]
pub enum Characters {
    A,
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
        match self {
            Characters::A => 0,
            Characters::B => 1,
            Characters::C => 2,
            Characters::D => 3,
            Characters::E => 4,
            Characters::F => 5,
            Characters::G => 6,
            Characters::H => 7,
            Characters::I => 8,
            Characters::J => 9,
            Characters::K => 10,
            Characters::L => 11,
            Characters::M => 12,
            Characters::N => 13,
            Characters::O => 14,
            Characters::P => 15,
            Characters::Q => 16,
            Characters::R => 17,
            Characters::S => 18,
            Characters::T => 19,
            Characters::U => 20,
            Characters::V => 21,
            Characters::W => 22,
            Characters::X => 23,
            Characters::Y => 24,
            Characters::Z => 25,
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
        let z: u8 = (x + y + 1) % 26;
        match z {
            00 => Characters::A,
            01 => Characters::B,
            02 => Characters::C,
            03 => Characters::D,
            04 => Characters::E,
            05 => Characters::F,
            06 => Characters::G,
            07 => Characters::H,
            08 => Characters::I,
            09 => Characters::J,
            10 => Characters::K,
            11 => Characters::L,
            12 => Characters::M,
            13 => Characters::N,
            14 => Characters::O,
            15 => Characters::P,
            16 => Characters::Q,
            17 => Characters::R,
            18 => Characters::S,
            19 => Characters::T,
            20 => Characters::U,
            21 => Characters::V,
            22 => Characters::W,
            23 => Characters::X,
            24 => Characters::Y,
            25 => Characters::Z,
            _ => {
                unreachable!(
                    "You managed to bypass a modulus operation. {} {}",
                    file!(),
                    line!()
                );
            }
        }
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
}
