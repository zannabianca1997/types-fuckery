use crate::{
    list::{Cons, Nil},
    tape::{GoLeft, GoRight, Tape},
    value::{IsNotZero, V, Value},
};

pub trait Instruction<Tape> {
    type Output;
}

// Moving tape

pub struct Left;

impl<Tape> Instruction<Tape> for Left
where
    Tape: GoLeft,
{
    type Output = Tape::Left;
}

pub struct Right;

impl<Tape> Instruction<Tape> for Right
where
    Tape: GoRight,
{
    type Output = Tape::Right;
}

// Cell manipulation

pub struct Increase;

impl<Before, This, After, Input, Output> Instruction<Tape<Before, This, After, Input, Output>>
    for Increase
where
    This: Value,
{
    type Output = Tape<Before, This::Next, After, Input, Output>;
}

pub struct Decrease;

impl<Before, This, After, Input, Output> Instruction<Tape<Before, This, After, Input, Output>>
    for Decrease
where
    This: Value,
{
    type Output = Tape<Before, This::Prev, After, Input, Output>;
}

// List of instruction as single instruction

impl<Tape> Instruction<Tape> for Nil {
    type Output = Tape;
}

impl<Tape, This, After> Instruction<Tape> for Cons<This, After>
where
    This: Instruction<Tape>,
    After: Instruction<<This as Instruction<Tape>>::Output>,
{
    type Output = <After as Instruction<<This as Instruction<Tape>>::Output>>::Output;
}

// Loop

pub struct Loop<Body>(Body);

impl<Before, After, Body, Input, Output> Instruction<Tape<Before, V<0>, After, Input, Output>>
    for Loop<Body>
{
    type Output = Tape<Before, V<0>, After, Input, Output>;
}
impl<Before, This, After, Body, Input, Output> Instruction<Tape<Before, This, After, Input, Output>>
    for Loop<Body>
where
    This: IsNotZero,
    Cons<Body, Loop<Body>>: Instruction<Tape<Before, This, After, Input, Output>>,
{
    type Output =
        <Cons<Body, Loop<Body>> as Instruction<Tape<Before, This, After, Input, Output>>>::Output;
}

// Input and output

pub struct Input;

impl<Before, This, After, InputValue, NextInput, Output>
    Instruction<Tape<Before, This, After, Cons<InputValue, NextInput>, Output>> for Input
{
    type Output = Tape<Before, InputValue, After, NextInput, Output>;
}

pub struct Output;

impl<Before, This, After, Input, OutputPrev>
    Instruction<Tape<Before, This, After, Input, OutputPrev>> for Output
{
    type Output = Tape<Before, This, After, Input, Cons<This, OutputPrev>>;
}
