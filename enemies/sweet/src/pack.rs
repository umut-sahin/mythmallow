use crate::prelude::*;

/// Resource for "Sweet" enemy pack.
#[derive(Debug, Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct SweetEnemyPack;

impl IEnemyPack for SweetEnemyPack {
    fn id(&self) -> SmolStr {
        "sweet".into()
    }

    fn name(&self) -> LocalizedText {
        LocalizedText::Localized {
            key: "sweet-enemies-pack-name",
            args: smallvec![],
            fallback: "Sweet Enemies".into(),
        }
    }
}
