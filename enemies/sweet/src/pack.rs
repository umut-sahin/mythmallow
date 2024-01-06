use mythmallow::prelude::*;

/// Resource for "Sweet" enemy pack.
#[derive(Debug, Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct SweetEnemyPack;

impl IEnemyPack for SweetEnemyPack {
    fn id(&self) -> SmolStr {
        "sweet".into()
    }

    fn name(&self) -> SmolStr {
        "Sweet".into()
    }
}
