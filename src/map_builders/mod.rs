use std::collections::VecDeque;

use crate::{algorithms::Option, map::Map};
pub use random::RandomMap;
pub use rooms::RoomsMap;

mod random;
mod rooms;
pub trait MapBuilder {
    fn build_map(&mut self, width: i32, height: i32, options: &[Option]);
    fn get_map(&self) -> Map;
    fn take_snapshot(&mut self);
    fn get_snapshot_history(&self) -> VecDeque<Map>;
}
