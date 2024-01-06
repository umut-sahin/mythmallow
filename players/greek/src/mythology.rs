use mythmallow::prelude::*;

/// Resource for "Greek" mythology.
#[derive(Debug, Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct GreekMythology;

impl IMythology for GreekMythology {
    fn id(&self) -> SmolStr {
        "greek".into()
    }

    fn name(&self) -> SmolStr {
        "Greek".into()
    }
}
