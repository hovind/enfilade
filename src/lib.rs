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

struct Sin<E>(PhantomData<E>);
struct Prod<E, F>(PhantomData<E>, PhantomData<F>);

struct Expr<E>(PhantomData<E>);


fn f64() -> Expr<f64> {
    Expr(PhantomData)
}

trait Evaluable: Sized {
    type Input;
    type Output;
    fn eval(self, x: Self::Input) -> Self::Output;
    fn expr() -> Expr<Self> {
        Expr(PhantomData)
    }
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


impl<E> Expr<E> {
    fn sin() -> Expr<Sin<E>> {
        Expr(PhantomData)
    }
}


impl<E> Evaluable for Expr<Sin<E>> where
Expr<E>: Sized + Evaluable,
<Expr<E> as Evaluable>::Output: Trig,
{
    type Input = <Expr<E> as Evaluable>::Input;
    type Output = <Expr<E> as Evaluable>::Output;
    fn eval(self, x: Self::Input) -> Self::Output {
        E::expr().eval(x).sin()
    }
}

impl Evaluable for Expr<f64> where
{
    type Input = f64;
    type Output = f64;
    fn eval(self, x: Self::Input) -> Self::Output {
        x
    }
}
/*
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
*/
