use std::rc::Rc;

use crate::*;
use db::*;

/// Query evaluation pipeline node, basically an iterator
pub trait QepNode {
	fn get_next(&mut self) -> Result<Option<Tuple>, Error>;
	fn schema(&self) -> Rc<Schema>;
}
