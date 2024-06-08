use crate::prelude::*;

/// Resource for "Greek" mythology.
#[derive(Debug, Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct GreekMythology;

impl IMythology for GreekMythology {
    fn id(&self) -> SmolStr {
        "greek".into()
    }

    fn name(&self) -> LocalizedText {
        LocalizedText::Localized {
            key: "greek-mythology-name",
            args: smallvec![],
            fallback: "Greek Mythology".into(),
        }
    }
}
