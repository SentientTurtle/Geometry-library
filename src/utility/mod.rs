use std::fmt::Debug;

/// Error type to signal the input to a "Law/Theorem" function (e.g. [`crate::shapes::triangle::law_of_sines`] is invalid.
///
/// Where used in a result `Result<T, InvalidInput>` this signals a solution is always extant where the input invariants are upheld. (E.g. values for a valid triangle are passed to a triangle-related function)
#[derive(Copy, Clone, Debug)]
pub struct InvalidInput;

/// Utility trait for Triangle solving
pub trait MaybeTwo<T>: Debug + Copy {
    fn count(&self) -> usize;
    fn first(self) -> T;
    fn both(self) -> (T, Option<T>);
    fn any_is<F: Fn(T) -> bool>(self, f: F) -> bool;

    type GenericSelf<U: Debug>: Debug;
    fn map<U: Debug, F: Fn(T) -> U>(self, f: F) -> Self::GenericSelf<U>;
}

impl<T: Debug + Copy> MaybeTwo<T> for T {
    fn count(&self) -> usize { 1 }

    fn first(self) -> T {
        self
    }

    fn both(self) -> (T, Option<T>) {
        (self, None)
    }

    fn any_is<F: Fn(T) -> bool>(self, f: F) -> bool {
        f(self)
    }

    type GenericSelf<U: Debug> = U;
    fn map<U: Debug, F: Fn(T) -> U>(self, f: F) -> Self::GenericSelf<U> {
        f(self)
    }
}

impl<T: Debug + Copy> MaybeTwo<T> for (T, Option<T>) {
    fn count(&self) -> usize {
        if self.1.is_some() {
            2
        } else {
            1
        }
    }

    fn first(self) -> T {
        self.0
    }

    fn both(self) -> (T, Option<T>) {
        self
    }

    fn any_is<F: Fn(T) -> bool>(self, f: F) -> bool {
        f(self.0) || self.1.is_some_and(f)
    }

    type GenericSelf<U: Debug> = (U, Option<U>);
    fn map<U: Debug, F: Fn(T) -> U>(self, f: F) -> Self::GenericSelf<U> {
        (f(self.0), self.1.map(f))
    }
}
