use crate::prelude::*;


/// Database of registered market systems.
#[derive(Clone, Copy, Debug, Deref, DerefMut, Resource)]
pub struct RegisteredMarketSystems {
    pub refresh_market: SystemId,
}

impl RegisteredMarketSystems {
    /// Creates the database.
    pub fn new(app: &mut App) -> RegisteredMarketSystems {
        use super::systems::*;
        RegisteredMarketSystems { refresh_market: app.world.register_system(refresh_market) }
    }
}


/// Resource for available balance to spend in the market.
#[derive(Clone, Copy, Debug, Default, Deref, DerefMut, PartialOrd, PartialEq, Reflect, Resource)]
pub struct Balance(pub f64);

impl Balance {
    /// Zero balance.
    pub const ZERO: Balance = Balance(0.00);
}

impl Balance {
    /// Sets balance.
    pub fn set(&mut self, amount: Balance) {
        *self = amount;
        log::info!("setting the balance to {}", self);
    }

    /// Records gains.
    pub fn gain(&mut self, amount: Balance, reason: impl AsRef<str>) {
        log::info!("gaining {} from {}", amount, reason.as_ref());
        self.0 += amount.0;
        log::info!("new balance is {}", self);
    }

    /// Records losses.
    pub fn spend(&mut self, amount: Balance, reason: impl AsRef<str>) {
        log::info!("spending {} to {}", amount, reason.as_ref());
        self.0 -= amount.0;
        log::info!("new balance is {}", self);
    }
}

impl Display for Balance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_nan() {
            return write!(f, "?");
        }
        if self.is_infinite() {
            write!(f, "{}∞", if self.is_sign_positive() { "" } else { "-" })
        } else {
            write!(f, "{:.2} $", self.0)
        }
    }
}


/// Resource for experience to balance ratio.
#[derive(Debug, Reflect, Resource)]
pub struct ExperienceToBalanceRatio(pub f64);

impl Default for ExperienceToBalanceRatio {
    fn default() -> ExperienceToBalanceRatio {
        ExperienceToBalanceRatio(1.00)
    }
}


/// Resource for the configuration of the market.
///
/// Configures the number of items offered in the market and which items can be offered.
///
/// # Examples
/// - items except the ones that require explicit whitelisting can be offered
/// ```
/// # use mythmallow_game::prelude::*;
/// MarketConfiguration::new();
/// ```
/// - items except the ones that require explicit whitelisting can be offered
/// - `bow-of-artemis` cannot be offered
/// ```
/// # use mythmallow_game::prelude::*;
/// MarketConfiguration::new().with_blacklisted_item("bow-of-artemis");
/// ```
/// - items except the ones that require explicit whitelisting can be offered
/// - items with `melee` tag cannot be not offered
/// ```
/// # use mythmallow_game::prelude::*;
/// MarketConfiguration::new().with_blacklisted_tag("melee");
/// ```
/// - items except the ones that require explicit whitelisting can be offered
/// - items with `melee` tag cannot be not offered
/// - `bow-of-artemis` cannot be offered
/// ```
/// # use mythmallow_game::prelude::*;
/// MarketConfiguration::new().with_blacklisted_tag("melee").with_blacklisted_item("bow-of-artemis");
/// ```
/// - `bow-of-artemis` can be offered
/// ```
/// # use mythmallow_game::prelude::*;
/// MarketConfiguration::new().with_whitelisted_item("bow-of-artemis");
/// ```
/// - items with `ranged` tag can be offered
/// ```
/// # use mythmallow_game::prelude::*;
/// MarketConfiguration::new().with_whitelisted_tag("ranged");
/// ```
/// - items with `melee` tag can be offered
/// - `bow-of-artemis` can be offered
/// ```
/// # use mythmallow_game::prelude::*;
/// MarketConfiguration::new().with_whitelisted_tag("melee").with_whitelisted_item("bow-of-artemis");
/// ```
/// - items with `speed-buff` tag cannot be not offered
/// - items with `utility` tag can be offered if they don't also have `speed-buff` tag.
/// ```
/// # use mythmallow_game::prelude::*;
/// MarketConfiguration::new().with_blacklisted_tag("speed-buff").with_whitelisted_tag("utility");
/// ```
#[derive(Debug, Reflect, Resource)]
pub struct MarketConfiguration {
    /// Whether the market can be opened by the player using the market keys.
    pub can_be_opened_by_player: bool,

    /// Number of items offered in the market.
    pub number_of_items: u8,

    /// Cost of using the refresh button.
    pub refresh_cost: MarketRefreshCost,

    /// Number of remaining free refreshes.
    pub free_refreshes: usize,

    /// Whether the refresh is free when no item is available to purchase.
    pub free_refresh_when_no_item_is_available: bool,

    /// Identifiers of blacklisted items.
    ///
    /// Blacklisted items are never ever offered in the market.
    pub blacklisted_items: HashSet<SmolStr>,

    /// Blacklisted tags.
    ///
    /// Items with blacklisted tags are not offered in the market unless whitelisted explicitly.
    pub blacklisted_tags: HashSet<SmolStr>,

    /// Identifiers of whitelisted items.
    ///
    /// Whitelisted items can be offered in the market unless blacklisted explicitly.
    pub whitelisted_tags: HashSet<SmolStr>,

    /// Whitelisted tags.
    ///
    /// Items with whitelisted tags can be offered in the market unless blacklisted.
    pub whitelisted_items: HashSet<SmolStr>,
}

impl MarketConfiguration {
    /// Creates a new market configuration.
    pub fn new() -> MarketConfiguration {
        MarketConfiguration::default()
    }
}

impl MarketConfiguration {
    /// Blacklists an item.
    pub fn with_blacklisted_item(mut self, item_id: impl AsRef<str>) -> MarketConfiguration {
        self.blacklisted_items.insert(item_id.as_ref().into());
        self
    }

    /// Blacklists a tag.
    pub fn with_blacklisted_tag(mut self, tag: impl AsRef<str>) -> MarketConfiguration {
        self.blacklisted_tags.insert(tag.as_ref().into());
        self
    }

    /// Whitelists an item.
    pub fn with_whitelisted_item(mut self, item_id: impl AsRef<str>) -> MarketConfiguration {
        self.whitelisted_items.insert(item_id.as_ref().into());
        self
    }

    /// Whitelists a tag.
    pub fn with_whitelisted_tag(mut self, tag: impl AsRef<str>) -> MarketConfiguration {
        self.whitelisted_tags.insert(tag.as_ref().into());
        self
    }
}

impl MarketConfiguration {
    /// Gets whether the refresh is free because no item is available.
    pub fn refresh_is_free_as_no_item_is_available(&self, state: &MarketState) -> bool {
        if self.free_refresh_when_no_item_is_available {
            let number_of_offered_items = state.offered_item_ids.len();
            let number_of_acquired_items = state.acquired_item_indices.len();
            if number_of_acquired_items == number_of_offered_items {
                return true;
            }
        }
        false
    }

    /// Gets the refresh cost.
    pub fn refresh_cost(&self, state: &MarketState) -> Balance {
        if self.refresh_is_free_as_no_item_is_available(state) || self.free_refreshes > 0 {
            Balance::ZERO
        } else {
            self.refresh_cost.get()
        }
    }

    /// Calculates the commonness of an item in the item registry.
    pub fn commonness_of(&self, item: &RegisteredItem) -> u64 {
        let id = item.id();

        if self.blacklisted_items.contains(&id) {
            return 0;
        }

        if self.whitelisted_items.contains(&id) {
            return item.commonness;
        }

        if item.needs_to_be_whitelisted_to_appear_in_market() {
            return 0;
        }

        let mut is_included = self.whitelisted_items.is_empty();

        if !self.whitelisted_tags.is_empty() {
            let has_whitelisted_tag =
                item.tags.iter().any(|tag| self.whitelisted_tags.contains(tag));
            is_included = has_whitelisted_tag;
        }

        if !self.blacklisted_tags.is_empty() {
            let has_blacklisted_tag =
                item.tags.iter().any(|tag| self.blacklisted_tags.contains(tag));
            if has_blacklisted_tag {
                is_included = false;
            }
        }

        if is_included { item.commonness } else { 0 }
    }
}

impl MarketConfiguration {
    /// Blacklists an item in place.
    pub fn blacklist_item(&mut self, item_id: impl AsRef<str>) -> &mut MarketConfiguration {
        self.blacklisted_items.insert(item_id.as_ref().into());
        self
    }

    /// Blacklists a tag in place.
    pub fn blacklist_tag(&mut self, tag: impl AsRef<str>) -> &mut MarketConfiguration {
        self.blacklisted_tags.insert(tag.as_ref().into());
        self
    }

    /// Whitelists an item in place.
    pub fn whitelist_item(&mut self, item_id: impl AsRef<str>) -> &mut MarketConfiguration {
        self.whitelisted_items.insert(item_id.as_ref().into());
        self
    }

    /// Whitelists a tag in place.
    pub fn whitelist_tag(&mut self, tag: impl AsRef<str>) -> &mut MarketConfiguration {
        self.whitelisted_tags.insert(tag.as_ref().into());
        self
    }
}

impl Default for MarketConfiguration {
    fn default() -> MarketConfiguration {
        MarketConfiguration {
            can_be_opened_by_player: true,
            number_of_items: 4,
            refresh_cost: MarketRefreshCost::default(),
            free_refreshes: 0,
            free_refresh_when_no_item_is_available: true,
            blacklisted_items: HashSet::new(),
            blacklisted_tags: HashSet::new(),
            whitelisted_items: HashSet::new(),
            whitelisted_tags: HashSet::new(),
        }
    }
}


/// Custom refresh cost function for the market.
#[derive(Clone, Copy, Debug, Deref, DerefMut)]
pub struct CustomRefreshCostInStepFunction(pub fn(usize) -> Balance);

impl Default for CustomRefreshCostInStepFunction {
    fn default() -> CustomRefreshCostInStepFunction {
        CustomRefreshCostInStepFunction(|_| Balance(1.00))
    }
}

impl From<fn(usize) -> Balance> for CustomRefreshCostInStepFunction {
    fn from(function: fn(usize) -> Balance) -> CustomRefreshCostInStepFunction {
        CustomRefreshCostInStepFunction(function)
    }
}


/// Refresh cost model for the market.
#[derive(Debug, Reflect)]
pub enum MarketRefreshCost {
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
        cost_in_step: CustomRefreshCostInStepFunction,
        current_step: usize,
        current_cost: Balance,
    },
}

impl MarketRefreshCost {
    /// Creates a constant refresh cost.
    pub fn constant(cost: Balance) -> MarketRefreshCost {
        MarketRefreshCost::Constant { cost }
    }

    /// Creates a linearly increasing refresh cost.
    pub fn linear(
        base_cost: Balance,
        increase_per_step: Balance,
        max_cost: Option<Balance>,
    ) -> MarketRefreshCost {
        MarketRefreshCost::Linear {
            base_cost,
            increase_per_step,
            current_step: 0,
            current_cost: base_cost,
            max_cost,
        }
    }

    /// Creates an exponentially increasing refresh cost.
    pub fn exponential(
        base_cost: Balance,
        increase_factor_per_step: f64,
        max_cost: Option<Balance>,
    ) -> MarketRefreshCost {
        if increase_factor_per_step < 1.00 {
            panic!("exponential refresh market cost factor cannot be smaller than 1.00");
        }
        MarketRefreshCost::Exponential {
            base_cost,
            increase_factor_per_step,
            current_step: 0,
            current_cost: base_cost,
            max_cost,
        }
    }

    /// Creates a custom refresh cost.
    pub fn custom(cost_in_step: fn(usize) -> Balance) -> MarketRefreshCost {
        MarketRefreshCost::Custom {
            cost_in_step: cost_in_step.into(),
            current_step: 0,
            current_cost: cost_in_step(0),
        }
    }
}

impl MarketRefreshCost {
    /// Gets the current refresh cost.
    pub fn get(&self) -> Balance {
        match self {
            MarketRefreshCost::Constant { cost } => *cost,
            MarketRefreshCost::Linear { current_cost, .. } => *current_cost,
            MarketRefreshCost::Exponential { current_cost, .. } => *current_cost,
            MarketRefreshCost::Custom { current_cost, .. } => *current_cost,
        }
    }
}

impl MarketRefreshCost {
    /// Increases the refresh cost according to the refresh cost model.
    pub fn step(&mut self) {
        match self {
            MarketRefreshCost::Constant { .. } => {},
            MarketRefreshCost::Linear {
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
            MarketRefreshCost::Exponential {
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
            MarketRefreshCost::Custom { current_step, current_cost, cost_in_step } => {
                *current_step += 1;
                *current_cost = (*cost_in_step)(*current_step);
            },
        }
    }

    /// Updates the refresh cost to be in a step.
    pub fn set_step(&mut self, step: usize) {
        self.reset();
        for _ in 0..step {
            self.step();
        }
    }

    /// Resets the refresh cost.
    pub fn reset(&mut self) {
        match self {
            MarketRefreshCost::Constant { .. } => {},
            MarketRefreshCost::Linear { base_cost, current_step, current_cost, .. } => {
                *current_step = 0;
                *current_cost = *base_cost;
            },
            MarketRefreshCost::Exponential { base_cost, current_step, current_cost, .. } => {
                *current_step = 0;
                *current_cost = *base_cost;
            },
            MarketRefreshCost::Custom { cost_in_step, current_step, current_cost } => {
                *current_step = 0;
                *current_cost = (*cost_in_step)(*current_step);
            },
        }
    }
}

impl Default for MarketRefreshCost {
    fn default() -> MarketRefreshCost {
        MarketRefreshCost::constant(Balance(1.00))
    }
}

impl Display for MarketRefreshCost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarketRefreshCost::Constant { cost } => write!(f, "constant {}", cost),
            MarketRefreshCost::Linear {
                base_cost,
                increase_per_step,
                current_step,
                current_cost,
                max_cost,
            } => {
                write!(
                    f,
                    "starting with {} increased by {} on every refresh{} \
                    (currently on step {} at {})",
                    base_cost,
                    increase_per_step,
                    max_cost.map(|cost| format!(" up to {}", cost)).unwrap_or_default(),
                    current_step,
                    current_cost,
                )
            },
            MarketRefreshCost::Exponential {
                base_cost,
                increase_factor_per_step,
                current_step,
                current_cost,
                max_cost,
            } => {
                write!(
                    f,
                    "starting with {} increased by {:.2} % on every refresh{} \
                    (currently on step {} at {})",
                    base_cost,
                    (increase_factor_per_step - 1.00) * 100.00,
                    max_cost.map(|cost| format!(" up to {}", cost)).unwrap_or_default(),
                    current_step,
                    current_cost,
                )
            },
            MarketRefreshCost::Custom { .. } => write!(f, "custom"),
        }
    }
}


/// Resource for the state of the market.
#[derive(Debug, Default, Resource)]
pub struct MarketState {
    pub offered_item_ids: Vec<SmolStr>,
    pub locked_item_indices: Vec<usize>,
    pub acquired_item_indices: Vec<usize>,
    pub processed_acquirements: usize,
}

impl MarketState {
    /// Gets whether the item in a given position is locked.
    pub fn is_locked(&self, position: NonZeroUsize) -> bool {
        self.locked_item_indices.contains(&(position.get() - 1))
    }

    /// Gets whether the item in a given position is acquired.
    pub fn is_acquired(&self, position: NonZeroUsize) -> bool {
        self.acquired_item_indices.contains(&(position.get() - 1))
    }
}

impl MarketState {
    /// Locks the item in the given position.
    pub fn lock(&mut self, position: NonZeroUsize) -> LockUnlockStatus {
        let index = position.get() - 1;
        if index >= self.offered_item_ids.len() {
            log::error!("unable to lock item {} in the market as it doesn't exist", position);
            return LockUnlockStatus::NotExist;
        }

        if self.acquired_item_indices.contains(&index) {
            log::error!("unable to lock item {} in the market as it's acquired", position);
            return LockUnlockStatus::Acquired;
        }
        if self.locked_item_indices.contains(&index) {
            log::error!("unable to lock item {} in the market as it's already locked", position);
            return LockUnlockStatus::AlreadyLocked;
        }

        log::info!("locking item {} in the market", position);
        self.locked_item_indices.push(index);
        LockUnlockStatus::Locked
    }

    /// Unlocks the item in the given position.
    pub fn unlock(&mut self, position: NonZeroUsize) -> LockUnlockStatus {
        let index = position.get() - 1;
        if index >= self.offered_item_ids.len() {
            log::error!("unable to unlock item {} in the market as it doesn't exist", position);
            return LockUnlockStatus::NotExist;
        }

        if self.acquired_item_indices.contains(&index) {
            log::error!("unable to unlock item {} in the market as it's acquired", position);
            return LockUnlockStatus::Acquired;
        }
        if !self.locked_item_indices.contains(&index) {
            log::error!("unable to unlock item {} in the market as it not locked", position);
            return LockUnlockStatus::AlreadyUnlocked;
        }

        log::info!("unlocking item {} in the market", position);
        self.locked_item_indices.retain(|i| *i != index);
        LockUnlockStatus::Unlocked
    }

    /// Locks the item in the given position.
    pub fn acquire(&mut self, position: NonZeroUsize) -> AcquireStatus {
        let index = position.get() - 1;
        if index >= self.offered_item_ids.len() {
            log::error!("unable to acquire item {} in the market as it doesn't exist", position);
            return AcquireStatus::NotExist;
        }

        if self.is_acquired(position) {
            log::error!(
                "unable to acquire item {} in the market as it's already acquired",
                position,
            );
            return AcquireStatus::AlreadyAcquired;
        }

        log::info!("acquiring item {} in the market", position);
        self.acquired_item_indices.push(index);
        AcquireStatus::Acquired
    }
}


/// Status of locking or unlocking.
#[derive(Debug)]
pub enum LockUnlockStatus {
    NotExist,
    Acquired,
    AlreadyLocked,
    Locked,
    AlreadyUnlocked,
    Unlocked,
}


/// Status of acquiring.
#[derive(Debug)]
pub enum AcquireStatus {
    NotExist,
    AlreadyAcquired,
    Acquired,
}
