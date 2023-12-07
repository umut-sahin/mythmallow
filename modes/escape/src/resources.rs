use mythmallow::prelude::*;


/// Resource for the current chapter.
#[derive(Debug, Deref, DerefMut, Reflect, Resource)]
pub struct CurrentChapter(pub u8);

impl Default for CurrentChapter {
    fn default() -> CurrentChapter {
        CurrentChapter(1)
    }
}


/// Resource for the remaining time to complete the current chapter.
#[derive(Debug, Deref, DerefMut, Reflect, Resource)]
pub struct ChapterTimer(pub Timer);
