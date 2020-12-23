use std::marker::PhantomData;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let f = f64().sin();
        let _x = f.eval(0.0);
        assert_eq!(2 + 2, 4);
    }
}


struct Sin<Expr> {
    phantom: PhantomData<Expr>,
}

struct Cos<Expr> {
    phantom: PhantomData<Expr>,
}

struct F64 {
    phantom: PhantomData<f64>,
}

fn f64() -> F64 {
    F64 { phantom: PhantomData }
}

impl F64 {
    fn sin(self) -> Sin<Self> {
        Sin { phantom: PhantomData }
    }
    fn cos(self) -> Cos<Self> {
        Cos { phantom: PhantomData }
    }
}

trait Evaluable {
    type Input;
    type Output;
    fn eval(self, x: Self::Input) -> Self::Output;
    fn expr() -> Self;
}

trait Trig {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
}

impl Trig for f64 {
    fn sin(self) -> Self {
        self.sin()
    }
    fn cos(self) -> Self {
        self.cos()
    }
}



impl<E> Evaluable for Sin<E> where
E: Evaluable,
E::Output: Trig,
{
    type Input = E::Input;
    type Output = E::Output;
    fn eval(self, x: Self::Input) -> Self::Output {
        E::expr().eval(x).sin()
    }

    fn expr() -> Self {
        Sin { phantom: PhantomData }
    }
}

impl<E> Evaluable for Cos<E> where
E: Evaluable,
E::Output: Trig,
{
    type Input = E::Output;
    type Output = E::Output;
    fn eval(self, x: Self::Input) -> Self::Output {
        Self::expr().eval(x).cos()
    }

    fn expr() -> Self {
        Cos { phantom: PhantomData }
    }
}

impl Evaluable for F64 where
{
    type Input = f64;
    type Output = f64;
    fn eval(self, x: Self::Input) -> Self::Output {
        x
    }

    fn expr() -> Self {
        F64 { phantom: PhantomData }
    }
}
