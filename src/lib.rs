//! FAUX
//!
//! A library to create mocks out of `struct`s without polluting your
//! code with traits that exist for test only.
//!
//! This library makes liberal use of unsafe Rust features, and it is
//! not recommended for use outside of tests.
//!
//! Basic Usage:
//! ```edition2018
//! #[faux::duck]
//! pub struct Foo {
//!     a: u32,
//! }
//!
//! #[faux::quack]
//! impl Foo {
//!     pub fn new(a: u32) -> Self {
//!         Foo { a }
//!     }
//!
//!     pub fn get_stuff(&self) -> u32 {
//!         self.a
//!     }
//! }
//!
//! fn main() {
//!   // `faux` will not override making the real version of your struct
//!   let real = Foo::new(3);
//!   assert_eq!(real.get_stuff(), 3);
//!
//!   // while providing a method to create a mock
//!   let mut mock = Foo::quack();
//!   unsafe { mock._mock_once_get_stuff(|_| 10) };
//!   # // when!(mock.get_stuff).then(|_| 10);
//!   assert_eq!(mock.get_stuff(), 10);
//! }
//! ```

mod quack;

pub use faux_macros::{duck, quack};
pub use quack::Quack;
use std::{any::TypeId, cell::RefCell};

#[doc(hidden)]
pub enum MaybeQuack<T> {
    Real(T),
    Quack(RefCell<Quack>),
}

impl<T> MaybeQuack<T> {
    pub fn quack() -> Self {
        MaybeQuack::Quack(RefCell::new(Quack::default()))
    }

    pub unsafe fn mock_once<I, O: 'static>(
        &mut self,
        id: TypeId,
        mock: impl (FnOnce(I) -> O) + 'static,
    ) {
        match self {
            MaybeQuack::Quack(quack) => quack.get_mut().mock_once(id, mock),
            MaybeQuack::Real(_) => panic!("not allowed to mock a real instance!"),
        }
    }
}
