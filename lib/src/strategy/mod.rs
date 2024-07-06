mod hidden_set;
mod hidden_single;
mod locked_candidates;
mod medusa;
mod naked_set;
mod naked_single;
mod pointing_set;
mod rectangle_elimination;
mod single_digit_pattern;
mod strategy_result;
mod strategy_type;
mod swordfish;
mod xwing;
mod xywing;
mod xyzwing;

pub mod aic;
pub mod highlight;
pub mod link;

pub use aic::find_general_aic;
pub use hidden_set::*;
pub use hidden_single::*;
pub use locked_candidates::*;
pub use medusa::*;
pub use naked_set::*;
pub use naked_single::*;
pub use pointing_set::*;
pub use rectangle_elimination::*;
pub use single_digit_pattern::*;
pub use strategy_result::*;
pub use strategy_type::*;
pub use swordfish::*;
pub use xwing::*;
pub use xywing::*;
pub use xyzwing::*;
