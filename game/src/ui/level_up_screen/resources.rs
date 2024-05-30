use crate::prelude::*;


/// Database of registered level up screen systems.
#[derive(Clone, Copy, Debug, Resource)]
pub struct RegisteredLevelUpScreenSystems {
    pub reroll_perks: SystemId,
}

impl RegisteredLevelUpScreenSystems {
    /// Creates the database.
    pub fn new(app: &mut App, systems: Entity) -> RegisteredLevelUpScreenSystems {
        use super::systems::*;

        let reroll_perks = app.world.register_system(reroll_perks);
        RegisteredSystems::attach(app, systems, reroll_perks, "reroll_perks");

        RegisteredLevelUpScreenSystems { reroll_perks }
    }
}


/// Resource for the configuration of the level up screen.
#[derive(Debug, Reflect, Resource)]
#[reflect(Resource)]
pub struct LevelUpScreenConfiguration {
    /// Number of perks offered in the level up screen.
    pub number_of_perks: u8,

    /// Cost of using the reroll button.
    pub reroll_cost: LevelUpScreenRerollCost,
}

impl LevelUpScreenConfiguration {
    /// Gets the reroll cost.
    pub fn reroll_cost(&self) -> Balance {
        self.reroll_cost.get()
    }
}

impl Default for LevelUpScreenConfiguration {
    fn default() -> LevelUpScreenConfiguration {
        LevelUpScreenConfiguration {
            number_of_perks: 4,
            reroll_cost: LevelUpScreenRerollCost::default(),
        }
    }
}


/// Custom reroll cost function for the level up screen.
#[derive(Clone, Copy, Debug, Deref, DerefMut)]
pub struct CustomRerollCostInStepFunction(pub fn(usize) -> Balance);

impl Default for CustomRerollCostInStepFunction {
    fn default() -> CustomRerollCostInStepFunction {
        CustomRerollCostInStepFunction(|_| Balance(1.00))
    }
}

impl From<fn(usize) -> Balance> for CustomRerollCostInStepFunction {
    fn from(function: fn(usize) -> Balance) -> CustomRerollCostInStepFunction {
        CustomRerollCostInStepFunction(function)
    }
}


/// Reroll cost model for the level up screen.
#[derive(Debug, Reflect)]
pub enum LevelUpScreenRerollCost {
    Constant {
        cost: Balance,
    },
    Linear {
        base_cost: Balance,
        increase_per_step: Balance,
        current_step: usize,
        current_cost: Balance,
        max_cost: Option<Balance>,
    },
    Exponential {
        base_cost: Balance,
        increase_factor_per_step: f64,
        current_step: usize,
        current_cost: Balance,
        max_cost: Option<Balance>,
    },
    Custom {
        #[reflect(ignore)]
        cost_in_step: CustomRerollCostInStepFunction,
        current_step: usize,
        current_cost: Balance,
    },
}

impl LevelUpScreenRerollCost {
    /// Creates a constant reroll cost.
    pub fn constant(cost: Balance) -> LevelUpScreenRerollCost {
        LevelUpScreenRerollCost::Constant { cost }
    }

    /// Creates a linearly increasing reroll cost.
    pub fn linear(
        base_cost: Balance,
        increase_per_step: Balance,
        max_cost: Option<Balance>,
    ) -> LevelUpScreenRerollCost {
        LevelUpScreenRerollCost::Linear {
            base_cost,
            increase_per_step,
            current_step: 0,
            current_cost: base_cost,
            max_cost,
        }
    }

    /// Creates an exponentially increasing reroll cost.
    pub fn exponential(
        base_cost: Balance,
        increase_factor_per_step: f64,
        max_cost: Option<Balance>,
    ) -> LevelUpScreenRerollCost {
        if increase_factor_per_step < 1.00 {
            panic!("exponential level up screen reroll cost factor cannot be smaller than 1.00");
        }
        LevelUpScreenRerollCost::Exponential {
            base_cost,
            increase_factor_per_step,
            current_step: 0,
            current_cost: base_cost,
            max_cost,
        }
    }

    /// Creates a custom reroll cost.
    pub fn custom(cost_in_step: fn(usize) -> Balance) -> LevelUpScreenRerollCost {
        LevelUpScreenRerollCost::Custom {
            cost_in_step: cost_in_step.into(),
            current_step: 0,
            current_cost: cost_in_step(0),
        }
    }
}

impl LevelUpScreenRerollCost {
    /// Gets the current reroll cost.
    pub fn get(&self) -> Balance {
        match self {
            LevelUpScreenRerollCost::Constant { cost } => *cost,
            LevelUpScreenRerollCost::Linear { current_cost, .. } => *current_cost,
            LevelUpScreenRerollCost::Exponential { current_cost, .. } => *current_cost,
            LevelUpScreenRerollCost::Custom { current_cost, .. } => *current_cost,
        }
    }
}

impl LevelUpScreenRerollCost {
    /// Increases the refresh cost according to the refresh cost model.
    pub fn step(&mut self) {
        match self {
            LevelUpScreenRerollCost::Constant { .. } => {},
            LevelUpScreenRerollCost::Linear {
                increase_per_step,
                current_step,
                current_cost,
                max_cost,
                ..
            } => {
                *current_step += 1;
                *current_cost = Balance(current_cost.0 + increase_per_step.0);
                if let Some(max_cost) = max_cost {
                    if *current_cost > *max_cost {
                        *current_cost = *max_cost;
                    }
                }
            },
            LevelUpScreenRerollCost::Exponential {
                increase_factor_per_step,
                current_step,
                current_cost,
                max_cost,
                ..
            } => {
                *current_step += 1;
                *current_cost = Balance(current_cost.0 * *increase_factor_per_step);
                if let Some(max_cost) = max_cost {
                    if *current_cost > *max_cost {
                        *current_cost = *max_cost;
                    }
                }
            },
            LevelUpScreenRerollCost::Custom { current_step, current_cost, cost_in_step } => {
                *current_step += 1;
                *current_cost = (*cost_in_step)(*current_step);
            },
        }
    }

    /// Updates the reroll cost to be in a step.
    pub fn set_step(&mut self, step: usize) {
        self.reset();
        for _ in 0..step {
            self.step();
        }
    }

    /// Resets the reroll cost.
    pub fn reset(&mut self) {
        match self {
            LevelUpScreenRerollCost::Constant { .. } => {},
            LevelUpScreenRerollCost::Linear { base_cost, current_step, current_cost, .. } => {
                *current_step = 0;
                *current_cost = *base_cost;
            },
            LevelUpScreenRerollCost::Exponential {
                base_cost, current_step, current_cost, ..
            } => {
                *current_step = 0;
                *current_cost = *base_cost;
            },
            LevelUpScreenRerollCost::Custom { cost_in_step, current_step, current_cost } => {
                *current_step = 0;
                *current_cost = (*cost_in_step)(*current_step);
            },
        }
    }
}

impl Default for LevelUpScreenRerollCost {
    fn default() -> LevelUpScreenRerollCost {
        LevelUpScreenRerollCost::constant(Balance(1.00))
    }
}

impl Display for LevelUpScreenRerollCost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LevelUpScreenRerollCost::Constant { cost } => write!(f, "constant {}", cost),
            LevelUpScreenRerollCost::Linear {
                base_cost,
                increase_per_step,
                current_step,
                current_cost,
                max_cost,
            } => {
                write!(
                    f,
                    "starting with {} increased by {} on every reroll{} \
                    (currently on step {} at {})",
                    base_cost,
                    increase_per_step,
                    max_cost.map(|cost| format!(" up to {}", cost)).unwrap_or_default(),
                    current_step,
                    current_cost,
                )
            },
            LevelUpScreenRerollCost::Exponential {
                base_cost,
                increase_factor_per_step,
                current_step,
                current_cost,
                max_cost,
            } => {
                write!(
                    f,
                    "starting with {} increased by {:.2} % on every reroll{} \
                    (currently on step {} at {})",
                    base_cost,
                    (increase_factor_per_step - 1.00) * 100.00,
                    max_cost.map(|cost| format!(" up to {}", cost)).unwrap_or_default(),
                    current_step,
                    current_cost,
                )
            },
            LevelUpScreenRerollCost::Custom { .. } => write!(f, "custom"),
        }
    }
}


/// Resource for the state of level up screen.
#[derive(Debug, Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct LevelUpScreenState {
    pub offered_perk_ids: Vec<SmolStr>,
}


/// Resource for the widgets of the level up screen.
#[derive(Debug, Default, Deref, DerefMut, Reflect, Resource)]
#[reflect(Resource)]
pub struct LevelUpScreenWidgets(
    /// Rows of widgets in the level up screen.
    ///
    /// - Select buttons
    /// - Balance & Reroll button
    pub [Vec<Entity>; 2],
);


/// Resource for the previously selected widget in the level up screen.
#[derive(Debug, Deref, DerefMut, Reflect, Resource)]
pub struct PreviouslySelectedLevelUpScreenWidget(pub Entity);


/// Reason of the level up screen.
#[derive(Debug, Reflect, Resource)]
pub enum LevelUpScreenReason {
    LevelingUp { to: Level },
    Cheating,
}
