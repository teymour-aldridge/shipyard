mod multiple;
#[cfg(feature = "parallel")]
mod par_multiple;

use super::{
    AbstractMut, CurrentId, DoubleEndedShiperator, ExactSizeShiperator, IntoAbstract, IntoIterator,
    Shiperator,
};

pub use multiple::*;
#[cfg(feature = "parallel")]
pub use par_multiple::*;
