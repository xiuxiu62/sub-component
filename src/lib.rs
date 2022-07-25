use num::One;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug)]
struct Number<T>(T);

impl<T: Copy + One + Add<Output = T> + Sub<Output = T>> Number<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn get(&self) -> T {
        self.0
    }

    pub fn set(&mut self, value: T) {
        self.0 = value;
    }

    pub fn increment(&mut self) {
        *self += num::one();
    }

    pub fn decrement(&mut self) {
        *self -= num::one();
    }
}

impl<T: Add<Output = T>> Add<T> for Number<T> {
    type Output = T;

    fn add(self, rhs: T) -> Self::Output {
        self.0 + rhs
    }
}

impl<T: Copy + Add<Output = T>> AddAssign<T> for Number<T> {
    fn add_assign(&mut self, rhs: T) {
        *self = Self(self.0 + rhs);
    }
}

impl<T: Sub<Output = T>> Sub<T> for Number<T> {
    type Output = T;

    fn sub(self, rhs: T) -> Self::Output {
        self.0 - rhs
    }
}

impl<T: Copy + Sub<Output = T>> SubAssign<T> for Number<T> {
    fn sub_assign(&mut self, rhs: T) {
        *self = Self(self.0 - rhs);
    }
}

#[cfg(test)]
mod tests {
    use crate::Number;

    #[test]
    fn number_works() {
        let mut number = Number(0_u32);

        number.set(10);
        println!("{}", number.get());

        (0..10).for_each(|_| number.increment());
        println!("{}", number.get());

        (0..8).for_each(|_| number.decrement());
        println!("{}", number.get());

        assert_eq!(number.get(), 12);
    }
}
