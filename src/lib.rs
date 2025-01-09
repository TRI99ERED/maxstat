#![allow(clippy::type_complexity)]

pub mod modifier;
pub mod non_send;
pub mod send;
pub mod stat;

pub mod prelude {
    pub use crate::modifier::standard::Additive;
    pub use crate::modifier::standard::Flat;
    pub use crate::modifier::standard::Multiplicative;
    pub use crate::modifier::standard::PostAdditive;
    pub use crate::modifier::standard::PostMultiplicative;

    pub use crate::non_send::standard::StandardStatNS;

    pub use crate::send::standard::StandardStatS;
}
