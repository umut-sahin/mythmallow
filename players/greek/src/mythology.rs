use mythmallow::prelude::*;

/// Greek mythology.
#[derive(Debug)]
pub struct GreekMythology;

impl Mythology for GreekMythology {
    fn id(&self) -> SmolStr {
        "greek".into()
    }

    fn name(&self) -> SmolStr {
        "Greek".into()
    }
}
