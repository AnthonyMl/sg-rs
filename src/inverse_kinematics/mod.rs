pub use self::chain::{Axis, Chain, Joint};
pub use self::updater::{State, Transition, update};
pub use self::jacobian_pseudo_inverse::{jacobian_pseudo_inverse};
pub use self::jacobian_transpose::{jacobian_transpose};
pub use self::cyclic_coordinate_descent::{cyclic_coordinate_descent};

mod chain;
mod cyclic_coordinate_descent;
mod jacobian_transpose;
mod jacobian_pseudo_inverse;
pub mod updater;