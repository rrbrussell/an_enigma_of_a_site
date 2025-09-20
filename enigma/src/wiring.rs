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

use crate::Characters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Wiring {
    data: [Characters; 26],
}

impl std::ops::Index<Characters> for Wiring {
    type Output = Characters;

    fn index(&self, index: Characters) -> &Self::Output {
        &(self.data[index as usize])
    }
}

impl Wiring {
    pub fn new(data: [Characters; 26]) -> Wiring {
        Wiring { data }
    }
}
