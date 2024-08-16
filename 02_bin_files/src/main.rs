mod mod1;
mod mod2;
mod module3 {
    pub mod mod3;
}

use mod1::fn1;
use mod2::fn2;
use module3::mod3::fn3;

fn main() {
    let sum = fn1() + fn2() + fn3();
    println!("{sum:?}");
    ()
}
