extern crate num;


/// Geometric product trait
pub trait GP<RHS = Self> {
    type Output;
    fn gp(self, rhs: RHS) -> Self::Output;
}

pub trait IP<RHS = Self> {
    type Output;
    fn ip(self, rhs: RHS) -> Self::Output;
}

pub trait OP<RHS = Self> {
    type Output;
    fn op(self, rhs: RHS) -> Self::Output;
}


macro_rules! ga_decl_constructor_elements {
    ($($e:ident: $E:ident),*) => {$(
        pub struct $E;
        #[allow(non_upper_case_globals)]
        #[allow(dead_code)]
        pub const $e: $E = $E;
    )*};
}

macro_rules! ga_decl_elements {
    ($($name:ident<T>),*) => {$(
        #[derive(Clone, Copy)]
        pub struct $name<T> {
            pub val: T
        }
    )*}
}

macro_rules! ga_impl_element_constructors {
    ($($ty_a:ident * $E:ident -> $elm:ident<$ty_c:ident> ),*) => {$(
        impl std::ops::Mul<$E> for $ty_a {
            type Output = $elm<$ty_c>;

            fn mul(self, _: $E) ->  $elm<$ty_c> {
                $elm { val: self}
            }
        }
    )*};
}

macro_rules! ga_impl_element_display {
    ($($elm_ty:ty: $fmt:expr),*) => {$(
        impl std::fmt::Display for $elm_ty {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, $fmt, self.val)
            }
        }
    )*}
}

macro_rules! ga_impl_element_cmp {
    ($( ( $elm_a:ident<$ty_a:ident>, $elm_b:ident<$ty_b:ident> ) -> bool),*) => {$(
        impl std::cmp::PartialEq<$elm_b<$ty_b>> for $elm_a<$ty_a> {
            fn eq(&self, rhs: &$elm_b<$ty_b>) ->  bool {
                self.val == rhs.val
            }

            fn ne(&self, rhs: &$elm_b<$ty_b>) ->  bool {
                self.val != rhs.val
            }
        }
    )*}
}

macro_rules! ga_impl_element_scale {
    ($($ty_a:ident * $elm_b:ident<$ty_b:ident> -> $elm_c:ident<$ty_c:ident>),*) => {$(
        impl std::ops::Mul<$elm_b<$ty_b>> for $ty_a {
            type Output = $elm_c<$ty_c>;

            fn mul(self, rhs: $elm_b<$ty_b>) ->  $elm_c<$ty_c> {
                $elm_c { val: self * rhs.val}
            }
        }
    )*}
}

macro_rules! drop_plus {
    ( + $expr:expr) => ($expr);
    ( - $expr:expr) => (-($expr));
}

macro_rules! ga_impl_element_gp {
    ($($elm_a:ident<$ty_a:ident> * $elm_b:ident<$ty_b:ident> -> $op:tt $elm_c:ident<$ty_c:ident> ),*) => {$(
        impl GP<$elm_b<$ty_b>> for $elm_a<$ty_a> {
            type Output = $elm_c<$ty_c>;

            fn gp(self, rhs: $elm_b<$ty_b>) ->  $elm_c<$ty_c> {
                $elm_c { val: drop_plus!($op (self.val * rhs.val))}
            }
        }
    )*};
}

macro_rules! ga_decl_blades {
    ($($name:ident<T>: [T; $grade:expr]),*) => {$(
        #[derive(Clone, Copy)]
        pub struct $name<T: std::marker::Copy> {
            pub val: [T; $grade]
        }
    )*}
}


mod ga2 {

    use std;
    use GP;

    ga_decl_constructor_elements! {
        e0: E0,
        e1: E1,
        e2: E2,
        e12: E12
    }

    ga_decl_elements! {
        Ge0<T>,
        Ge1<T>,
        Ge2<T>,
        Ge12<T>
    }

    ga_impl_element_constructors! {
        f32 * E0 -> Ge0<f32>,
        f32 * E1 -> Ge1<f32>,
        f32 * E2 -> Ge2<f32>,
        f32 * E12 -> Ge12<f32>,

        f64 * E0 -> Ge0<f64>,
        f64 * E1 -> Ge1<f64>,
        f64 * E2 -> Ge2<f64>,
        f64 * E12 -> Ge12<f64>
    }

    ga_impl_element_display! {
        Ge0<f32>: "{}*e0",
        Ge1<f32>: "{}*e1",
        Ge2<f32>: "{}*e2",
        Ge12<f32>: "{}*e12",

        Ge0<f64>: "{}*e0",
        Ge1<f64>: "{}*e1",
        Ge2<f64>: "{}*e2",
        Ge12<f64>: "{}*e12"
    }

    ga_impl_element_cmp! {
        ( Ge0<f32>, Ge0<f32> ) -> bool,
        ( Ge1<f32>, Ge1<f32> ) -> bool,
        ( Ge2<f32>, Ge2<f32> ) -> bool,
        ( Ge12<f32>, Ge12<f32> ) -> bool
    }

    ga_impl_element_scale! {
        f32 * Ge0<f32> -> Ge0<f32>,
        f32 * Ge1<f32> -> Ge1<f32>,
        f32 * Ge2<f32> -> Ge2<f32>,
        f32 * Ge12<f32> -> Ge12<f32>,

        f64 * Ge0<f64> -> Ge0<f64>,
        f64 * Ge1<f64> -> Ge1<f64>,
        f64 * Ge2<f64> -> Ge2<f64>,
        f64 * Ge12<f64> -> Ge12<f64>
    }

    ga_impl_element_gp! {
        Ge0<f32> * Ge0<f32>  -> + Ge0<f32>,
        Ge0<f32> * Ge1<f32>  -> + Ge1<f32>,
        Ge0<f32> * Ge2<f32>  -> + Ge2<f32>,
        Ge0<f32> * Ge12<f32> -> + Ge12<f32>,

        Ge1<f32> * Ge0<f32>  -> + Ge1<f32>,
        Ge1<f32> * Ge1<f32>  -> + Ge0<f32>,
        Ge1<f32> * Ge2<f32>  -> + Ge12<f32>,
        Ge1<f32> * Ge12<f32> -> + Ge2<f32>,

        Ge2<f32> * Ge0<f32>  -> + Ge2<f32>,
        Ge2<f32> * Ge1<f32>  -> - Ge12<f32>,
        Ge2<f32> * Ge2<f32>  -> + Ge0<f32>,
        Ge2<f32> * Ge12<f32> -> - Ge1<f32>,

        Ge12<f32> * Ge0<f32>  -> + Ge12<f32>,
        Ge12<f32> * Ge1<f32>  -> - Ge2<f32>,
        Ge12<f32> * Ge2<f32>  -> + Ge1<f32>,
        Ge12<f32> * Ge12<f32> -> - Ge0<f32>,

        Ge0<f64> * Ge0<f64>  -> + Ge0<f64>,
        Ge0<f64> * Ge1<f64>  -> + Ge1<f64>,
        Ge0<f64> * Ge2<f64>  -> + Ge2<f64>,
        Ge0<f64> * Ge12<f64> -> + Ge12<f64>,

        Ge1<f64> * Ge0<f64>  -> + Ge1<f64>,
        Ge1<f64> * Ge1<f64>  -> + Ge0<f64>,
        Ge1<f64> * Ge2<f64>  -> + Ge12<f64>,
        Ge1<f64> * Ge12<f64> -> + Ge2<f64>,

        Ge2<f64> * Ge0<f64>  -> + Ge2<f64>,
        Ge2<f64> * Ge1<f64>  -> - Ge12<f64>,
        Ge2<f64> * Ge2<f64>  -> + Ge0<f64>,
        Ge2<f64> * Ge12<f64> -> - Ge1<f64>,
//        (f64,E2) * (f64,E12) -> - (f64,E1)
        Ge12<f64> * Ge0<f64>  -> + Ge12<f64>,
        Ge12<f64> * Ge1<f64>  -> - Ge2<f64>,
        Ge12<f64> * Ge2<f64>  -> + Ge1<f64>,
        Ge12<f64> * Ge12<f64> -> - Ge0<f64>
    }

    ga_decl_blades! {
        Gb0<T>: [T; 1],
        Gb1<T>: [T; 2],
        Gb2<T>: [T; 1]
    }

}

mod ga3 {

    use std;

    ga_decl_blades! {
        Gb0<T>: [T; 1],
        Gb1<T>: [T; 3]
    }
}

macro_rules! define_elements {
    ($(($E:ident<$T:ident>, $op:tt)),*) => {$(

        #[derive(Clone, Copy)]
        struct $E<$T>($T);

        impl<T: num::Float> std::ops::Add<$E<$T>> for $E<$T> {
            type Output = $E<$T>;
            fn add(self, rhs: $E<$T>) -> $E<$T> {
                $E(self.0 + rhs.0)
            }
        }

        impl<T: num::Float> std::ops::Sub<$E<$T>> for $E<$T> {
            type Output = $E<$T>;
            fn sub(self, rhs: $E<$T>) -> $E<$T> {
                $E(self.0 - rhs.0)
            }
        }

        impl<T: num::Float> std::ops::Mul<$T> for $E<$T> {
            type Output = $E<$T>;
            fn mul(self, rhs: $T) -> $E<$T> {
                $E(self.0 * rhs)
            }
        }

        impl<T: num::Float> std::ops::Mul<$E<$T>> for $E<$T> {
            type Output = $T;
            fn mul(self, rhs: $E<$T>) -> $T {
                drop_plus!($op (self.0 * rhs.0))
            }
        }

        impl<T: num::Float> std::ops::Div<$T> for $E<$T> {
            type Output = $E<$T>;
            fn div(self, rhs: $T) -> $E<$T> {
                $E(self.0 / rhs)
            }
        }

        impl<T: num::Float> std::ops::Div<$E<$T>> for $E<$T> {
            type Output = $T;
            fn div(self, rhs: $E<$T>) -> $T {
                drop_plus!($op (self.0 / rhs.0))
            }
        }

        impl<T: num::Float> std::ops::Rem<$T> for $E<$T> {
            type Output = $E<$T>;
            fn rem(self, rhs: $T) -> $E<$T> {
                $E(self.0 % rhs)
            }
        }

        impl<T: num::Float> std::ops::Rem<$E<$T>> for $E<$T> {
            type Output = $T;
            fn rem(self, rhs: $E<$T>) -> $T {
                drop_plus!($op (self.0 % rhs.0))
            }
        }

        impl<T: num::Float> std::cmp::PartialEq<$E<$T>> for $E<$T> {
            fn eq(&self, rhs: &$E<$T>) -> bool {
                &self.0 == &rhs.0
            }
            fn ne(&self, rhs: &$E<$T>) -> bool {
                &self.0 != &rhs.0
            }
        }

        impl<T: num::Float> std::cmp::Eq for $E<$T> {
        }

        impl<T: num::Float> std::cmp::PartialOrd<$E<$T>> for $E<$T> {
            fn partial_cmp(&self, rhs: &$E<$T>) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&rhs.0)
            }
            fn lt(&self, rhs: &$E<$T>) -> bool {
                self.0.lt(&rhs.0)
            }
            fn le(&self, rhs: &$E<$T>) -> bool {
                self.0.le(&rhs.0)
            }
            fn gt(&self, rhs: &$E<$T>) -> bool {
                self.0.gt(&rhs.0)
            }
            fn ge(&self, rhs: &$E<$T>) -> bool {
                self.0.ge(&rhs.0)
            }
        }
/*
        impl<T: num::Float> std::cmp::Ord for $E<$T> {
            fn cmp(&self, rhs: &$E<$T>) -> std::cmp::Ordering {
                self.0.cmp(&rhs.0)
            }
        }
*/
        impl<T: num::Float> std::ops::Not for $E<$T> {
            type Output = $E<$T>;

            fn not(self) ->  $E<$T> {
                $E(drop_plus!($op self.0))
            }
        }

        impl<T: num::Float> std::ops::Neg for $E<$T> {
            type Output = $E<$T>;

            fn neg(self) ->  $E<$T> {
                $E(-self.0)
            }
        }

    )*}
}

#[cfg(test)]
mod ga2_tests {
//    use super::Ge0;

    #[test]
    fn test_() {
        use ::std;
        use super::ga2::*;
        println!("{} {}", std::mem::size_of::<(f32,E0)>(), std::mem::size_of_val(&(2_f32,e0)));
//        assert!(false);
    }
    #[test]
    fn test_elements_construct() {
        use super::ga2::*;
        assert!((2_f32 * e0).val == 2_f32);
        assert!((2_f32 * e1).val == 2_f32);
        assert!((2_f32 * e2).val == 2_f32);
        assert!((2_f32 * e12).val == 2_f32);
    }

    #[test]
    fn test_element_cmp() {
        use super::ga2::*;
        assert!(2_f32 * e0 == 2_f32 * e0);
        assert!(3_f32 * e0 != 2_f32 * e0);
        assert!(2_f32 * e12 == 2_f32 * e12);
        assert!(3_f32 * e12 != 2_f32 * e12);
    }

    #[test]
    fn test_element_scale() {
        use super::ga2::*;
        assert!((3_f32*(2_f32 * e0)).val == 6_f32);
        assert!((3_f32*(2_f32 * e1)).val == 6_f32);
        assert!((3_f32*(2_f32 * e2)).val == 6_f32);
        assert!((3_f32*(2_f32 * e12)).val == 6_f32);
    }

    #[test]
    fn test_elemet_ga() {
        use super::*;
        use super::ga2::*;
        assert!((2_f32 * e1).gp(3_f32 * e1).val == 6_f32);
    }

    #[test]
    fn text_mandelbrot() {
        use ::std;
        use ::num;

        define_elements! {
            (E1<T>, +),
            (E2<T>, +),
            (E12<T>, -)
        }

        #[derive(Clone,Copy)]
        struct C<T>(T,E12<T>);

        impl<T: num::Float> C<T> {
            fn norm(&self) -> T {
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

        let dx = 0.05_f32;
        let dy = 0.05_f32;
        let ni = 40;
        let nj = ni;

        for j in -nj/2..nj {
            let y = j as f32 * dy;
            for i in -ni/2..ni {
                let x = i as f32 * dx;
                let z0 = C(x, E12(y));
                let mut z = z0;
                let mut k = 0;
                while (k < 255) && (z.norm() < 10.) {
                    z = z*z + z0;
                    k += 1;
                }
                if k >= 255 {
                    print!("{}", "*");
                }
                else {
                    print!("{}", " ");
                }
            }
            println!("{}", "");
        }
        assert!(false);
    }
}
