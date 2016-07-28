extern crate num;
use std;

define_elements! {
    (E1<T>, +),
    (E2<T>, +),
    (E12<T>, -)
}

#[derive(Clone,Copy)]
pub struct C<T>(pub T, pub E12<T>);

impl<T: num::Float> C<T> {
    pub fn norm(&self) -> T {
        (self.0*self.0 + (self.1).0*(self.1).0).sqrt()
    }
}

impl<T: num::Float> std::ops::Add<C<T>> for C<T> {
    type Output = C<T>;
    fn add(self, rhs: C<T>) -> C<T> {
        C(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: num::Float> std::ops::Sub<C<T>> for C<T> {
    type Output = C<T>;
    fn sub(self, rhs: C<T>) -> C<T> {
        C(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: num::Float> std::ops::Mul<C<T>> for C<T> {
    type Output = C<T>;

    fn mul(self, rhs: C<T>) ->  C<T> {
        C(self.0*rhs.0 + self.1*rhs.1, rhs.1*self.0 + self.1*rhs.0)
    }
}


impl<T: num::Float> std::ops::Not for C<T> {
    type Output = C<T>;

    fn not(self) ->  C<T> {
        C(self.0, -self.1)
    }
}
