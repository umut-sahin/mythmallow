use crate::prelude::*;


/// Schedule for running multiple substeps in a physics update.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ScheduleLabel)]
pub struct SubstepSchedule;
