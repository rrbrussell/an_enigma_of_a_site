// fn main() {
//     println!("Hello, world!");
// }

use enigma::Characters;

fn main() {
    let a = Characters::A;
    let b = Characters::B;
    println!("{}", a + b); // Should print "C"

    let z = Characters::Z;
    let a = Characters::A;
    println!("{}", z + a); // Should print "A"

    let m = Characters::M;
    let n = Characters::N;
    println!("{}", m + n); // Should print "R"
    println!("{}", <Characters as Into<u8>>::into(m));
    println!("{}", <Characters as Into<u8>>::into(n));
}
