#![recursion_limit = "1024"]

use types_fuckery::types_fuckery;

mod static_display;
use static_display::StaticDisplay;

type Output = types_fuckery! {
    program: {
        > > + > > > > > , [ > + > > , ] > + [ - - [ + < < < - ] < [ < + > - ]
        < [ < [ - > [ < < < + > > > > + < - ] < < [ > > + > [ - > ] < < [ < ]
        < - ] > ] > > > + < [ [ - ] < [ > + < - ] < ] > [ [ > > > ] + < < < -
        < [ < < [ < < < ] > > + > [ > > > ] < - ] < < [ < < < ] > [ > > [ > >
        > ] < + < < [ < < < ] > - ] ] + < < < ] + [ - > > > ] > > ] > > [ . >
        > > ]
    },
    input:   { 5 3 8 9 2 1 1 29 2 3 84 9 21 0 }
};

fn main() {
    println!("Output: {}", Output::display());
}
