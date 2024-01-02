use mythmallow::prelude::*;

/// Sweet enemies.
#[derive(Debug)]
pub struct SweetEnemyPack;

impl IEnemyPack for SweetEnemyPack {
    fn id(&self) -> SmolStr {
        "sweet".into()
    }

    fn name(&self) -> SmolStr {
        "Sweet".into()
    }
}
