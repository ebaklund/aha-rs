
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


#[cfg(test)]
mod ga2_tests {
//    use super::Ge0;

    #[test]
    fn test_elements_construct() {
        use super::ga2::*;
        assert!((2 as f32 * e0).val == 2 as f32);
        assert!((2 as f32 * e1).val == 2 as f32);
        assert!((2 as f32 * e2).val == 2 as f32);
        assert!((2 as f32 * e12).val == 2 as f32);
    }

    #[test]
    fn test_element_scale() {
        use super::ga2::*;
        assert!((3f32*(2 as f32 * e0)).val == 6 as f32);
        assert!((3f32*(2 as f32 * e1)).val == 6 as f32);
        assert!((3f32*(2 as f32 * e2)).val == 6 as f32);
        assert!((3f32*(2 as f32 * e12)).val == 6 as f32);
    }

    #[test]
    fn test_elemet_ga() {
        use super::*;
        use super::ga2::*;
        assert!((2f32 * e1).gp(3f32 * e1).val == 6.);
    }
}
