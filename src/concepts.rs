use std::fmt::Debug;
use std::ops::{Add, Mul, MulAssign, Sub, Div};

struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

// index is power, value is multiple
#[derive(Clone)]
struct Polynomial<T> {
    values: Vec<T>,
}

impl<'a, T> Polynomial<T>
where
    T: Add + Sub + Mul<Output = T> + MulAssign<T> + 'a + From<usize> + Copy + Debug,
{
    pub fn new(values: Vec<T>) -> Self {
        Self { values }
    }
    pub fn derivative(&mut self) {
        let len = self.values.len();
        self.values.pop();
        self.values = self
            .values
            .iter_mut()
            .enumerate()
            .map(|(i, x)| *x * (len - (i + 1)).try_into().unwrap())
            .collect();
    }
}

// impl <T: Add + Sub + Mul>Vec3<T> {
//
// }
//
struct TaylorSeries<T> {
    center: T,
    start: usize,
    // value of n
    current: usize,
    next: usize,
    terms: Vec<Polynomial<T>>,
}

impl<T> Iterator for TaylorSeries<T>
where
    T: Add + Sub + Mul<Output = T> + MulAssign<T> + From<usize> + Copy + Debug,
{
    type Item = T;
    // next is the sum
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.terms.last() {
            let mut next_term = v.clone();
            next_term.derivative();
            self.terms.push(next_term);
        }
        self.current += 1;
        unimplemented!()
    }
}

impl<T> TaylorSeries<T>
where
    T: Add<T, Output = T> + Sub<Output = T> + Mul<Output = T> + MulAssign<T> + From<usize> + Copy + Debug + Div<Output = T>,
{
    pub fn sum(&self, x_value: T) -> T {
        self.terms.iter().enumerate().fold(0.try_into().unwrap(), |sum: T, (n, x)| {
            let len = x.values.len();
            sum + x
                .values
                .iter()
                .enumerate()
                .fold(0.try_into().unwrap(), |s: T, (e, j)| {
                    let mut x_power = x_value;
                    for _ in 1..(len - (e + 1)) {
                        x_power *= x_power;
                    }
                    let mut center_power = x_value - self.center;
                    for _ in 1..n {
                        center_power *= center_power;
                    }
                    let mut n_factorial = n;
                    for i in 1..n {
                        n_factorial *= n_factorial - i;
                    }
                    let n_factorial: T = n_factorial.try_into().unwrap();
                    s + ((*j * x_power) * center_power) / n_factorial
                })
        });
        unimplemented!();
    }
}

//
//

#[cfg(test)]
mod test {
    use super::Polynomial;

    #[test]
    fn polynomial_derivative() {
        let val = vec![4, 3, 2, 1];
        let mut v = Polynomial::<usize>::new(val);
        v.derivative();
        assert_eq!(v.values, vec![12, 6, 2])
    }
}
