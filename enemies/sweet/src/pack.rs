use mythmallow::prelude::*;

/// Sweet enemies.
#[derive(Debug)]
pub struct SweetMunchiesPack;

impl MunchiePack for SweetMunchiesPack {
    fn id(&self) -> SmolStr {
        "sweet".into()
    }

    fn name(&self) -> SmolStr {
        "Sweet".into()
    }
}
