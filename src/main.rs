#![recursion_limit = "1020"]

use std::any::type_name;

use types_fuckery::types_fuckery;

type Test = types_fuckery! {
    [, < , [- > + <] > .]
    [ V05 V03 ]
};

fn main() {
    println!("{}", type_name::<Test>())
}
