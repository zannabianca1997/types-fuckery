use crate::{
    list::{Cons, Nil},
    value::V00,
};

pub struct Tape<Before, Value, After, Input, Output>(Before, Value, After, Input, Output);

pub trait GoLeft {
    type Left;
}
impl<Before, This, Left, After, Input, Output> GoLeft
    for Tape<Cons<Left, Before>, This, After, Input, Output>
{
    type Left = Tape<Before, Left, Cons<This, After>, Input, Output>;
}
impl<This, After, Input, Output> GoLeft for Tape<Nil, This, After, Input, Output> {
    type Left = Tape<Nil, V00, Cons<This, After>, Input, Output>;
}

pub trait GoRight {
    type Right;
}
impl<Before, This, Right, After, Input, Output> GoRight
    for Tape<Before, This, Cons<Right, After>, Input, Output>
{
    type Right = Tape<Cons<This, Before>, Right, After, Input, Output>;
}
impl<Before, This, Input, Output> GoRight for Tape<Before, This, Nil, Input, Output> {
    type Right = Tape<Cons<This, Before>, V00, Nil, Input, Output>;
}

pub trait HasOutput {
    type Output;
}

impl<Before, Value, After, Input, Output> HasOutput for Tape<Before, Value, After, Input, Output> {
    type Output = Output;
}
