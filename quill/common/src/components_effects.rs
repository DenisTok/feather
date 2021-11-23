use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeSet;

/// Storing Potion effects info.
#[derive(Copy, Clone, Debug, Hash, Serialize, PartialEq, Eq, Deserialize)]
pub struct PotionApplication {
    /// Strength Level of effect.
    pub amplifier: u8,
    /// Tick-based duration of the effect.
    pub duration: u32,
    /// Whether spawn particles or not.
    pub particle: bool,
    /// Whether the effect was given by a beacon or not.
    pub ambient: bool,
    /// Show effect icon or not.
    pub icon: bool,

    /// Store when effect was added, if start_tick == 0 effect not yet sent to client
    pub start_tick: u64,
}

impl PotionApplication {
    pub fn start_at(&mut self, start_tick: u64) {
        self.start_tick = start_tick
    }
}

impl Ord for PotionApplication {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.amplifier > other.amplifier || self.duration > other.duration {
            Ordering::Greater
        } else if self.amplifier == other.amplifier || self.duration == other.duration {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}
impl PartialOrd for PotionApplication {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

macro_rules! impl_effect {
    ($ident:ident) => {
        #[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
        pub struct $ident(pub BTreeSet<PotionApplication>);
        impl $ident {
            pub fn new() -> $ident {
                $ident { 0: BTreeSet::new() }
            }
            pub fn add_effect(&mut self, effect: PotionApplication) -> bool {
                self.0.insert(effect)
            }
            pub fn not_started(&mut self) -> Vec<PotionApplication> {
                self.0
                    .iter()
                    .filter(|effect| effect.start_tick == 0)
                    .cloned()
                    .collect::<Vec<_>>()
            }
            pub fn ended_on_tick(&mut self, tick: u64) -> Vec<PotionApplication> {
                self.0
                    .iter()
                    .filter(|effect| {
                        tick >= effect.start_tick + effect.duration as u64 && effect.start_tick != 0
                    })
                    .cloned()
                    .collect::<Vec<_>>()
            }
            pub fn active_effect(&mut self) -> Option<&PotionApplication> {
                self.0.iter().last()
            }
        }
        impl Default for $ident {
            fn default() -> Self {
                $ident::new()
            }
        }
        bincode_component_impl!($ident);
    };
}

impl_effect!(SpeedEffect);
impl_effect!(SlownessEffect);
impl_effect!(HasteEffect);
impl_effect!(MiningFatigueEffect);
impl_effect!(StrengthEffect);
impl_effect!(InstantHealthEffect);
impl_effect!(InstantDamageEffect);
impl_effect!(JumpBoostEffect);
impl_effect!(NauseaEffect);
impl_effect!(RegenerationEffect);
impl_effect!(ResistanceEffect);
impl_effect!(FireResistanceEffect);
impl_effect!(WaterBreathingEffect);
impl_effect!(InvisibilityEffect);
impl_effect!(BlindnessEffect);
impl_effect!(NightVisionEffect);
impl_effect!(HungerEffect);
impl_effect!(WeaknessEffect);
impl_effect!(PoisonEffect);
impl_effect!(WitherEffect);
impl_effect!(HealthBoostEffect);
impl_effect!(AbsorptionEffect);
impl_effect!(SaturationEffect);
impl_effect!(GlowingEffect);
impl_effect!(LevitationEffect);
impl_effect!(LuckEffect);
impl_effect!(BadLuckEffect);
impl_effect!(SlowFallingEffect);
impl_effect!(ConduitPowerEffect);
impl_effect!(DolphinsGraceEffect);
impl_effect!(BadOmenEffect);
impl_effect!(HeroOfTheVillageEffect);

/// A walk speed modifier in percent
#[derive(
    Copy, Clone, Debug, PartialEq, Serialize, Deserialize, derive_more::Deref, derive_more::DerefMut,
)]
pub struct SpeedEffectModifier(pub i32);

bincode_component_impl!(SpeedEffectModifier);
