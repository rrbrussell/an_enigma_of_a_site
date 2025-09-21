// fn main() {
//     println!("Hello, world!");
// }

use enigma::Characters;
use std::vec::Vec;

// I("EKMFLGDVZNTOWYHXUSPAIBRCJ", 'Q')
// II("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'E')
// III("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'V')
// IV("ESOVPZJAYQUIRHXLNFTGKDCMWB", 'J')
// V("VZBRGITYUPSDNHLXZWMJQOFECK", 'Z')

fn main() {
    // Okay time to convert the wiring table for a rotor into two different
    // maps. One for the forward condition and the second for the backward
    // condition.
    //
    // The forward map is fairly easy.
    //
    // I("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Q')
    // II("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'E')
    // III("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'V')
    // IV("ESOVPZJAYQUIRHXLNFTGKDCMWB", 'J')
    // V("VZBRGITYUPSDNHLXZWMJQOFECK", 'Z')

    let rotors: [&'static str; 3] = ["YRUHQSLDPXNGOKMIEBFZCWVJAT", "FVPJIAOYEDRZXWGCTKUQSBNMHL"];

    for (number, string) in rotors.iter().enumerate() {
        let mut forward: Vec<Characters> = Vec::with_capacity(26);
        println!("\npub(crate) static REFLECTOR_WIDE{number}: [Characters; 26] = [");
        for (index, value) in string.chars().enumerate() {
            if let Ok(value) = Characters::try_from(value) {
                forward.push(value);
                if index == 25 {
                    println!("{value:?}\n];")
                } else {
                    println!("{value:?},")
                }
            }
        }
    }
}
