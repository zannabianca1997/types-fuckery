use core::{
    fmt::{Display, Formatter, Result},
    marker::PhantomData,
};

use types_fuckery::{
    list::{Cons, Nil},
    value::V,
};

struct Displayable<T: ?Sized>(PhantomData<T>);
impl<T: StaticDisplay + ?Sized> Display for Displayable<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        T::fmt(f)
    }
}

pub trait StaticDisplay {
    fn fmt(f: &mut Formatter<'_>) -> Result;
    fn display() -> impl Display {
        Displayable::<Self>(PhantomData)
    }
}

impl<V, V2, T> StaticDisplay for Cons<V, Cons<V2, T>>
where
    V: StaticDisplay,
    Cons<V2, T>: StaticDisplay,
{
    fn fmt(f: &mut Formatter<'_>) -> Result {
        Cons::<V2, T>::fmt(f)?;
        f.write_str(", ")?;
        V::fmt(f)?;
        Ok(())
    }
}

impl<V> StaticDisplay for Cons<V, Nil>
where
    V: StaticDisplay,
{
    fn fmt(f: &mut Formatter<'_>) -> Result {
        V::fmt(f)?;
        Ok(())
    }
}

impl StaticDisplay for Nil {
    fn fmt(_f: &mut Formatter<'_>) -> Result {
        Ok(())
    }
}

impl<const N: u8> StaticDisplay for V<N> {
    fn fmt(f: &mut Formatter<'_>) -> Result {
        N.fmt(f)
    }
}
