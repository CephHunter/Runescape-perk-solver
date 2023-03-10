use std::{fmt, str::FromStr, sync::atomic::AtomicBool};
use strum::EnumIter;
use strum_macros::EnumCount;

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, EnumCount, EnumIter)]
pub enum PerkName {
    Empty,
    Absorbative,
    Aftershock,
    Antitheism,
    Biting,
    Blunted,
    Brassican,
    Breakdown,
    BriefRespite,
    Bulwark,
    Butterfingers,
    Caroming,
    Cautious,
    Charitable,
    Cheapskate,
    ClearHeaded,
    Committed,
    Confused,
    Crackling,
    CrystalShield,
    DemonBait,
    DemonSlayer,
    Devoted,
    DragonBait,
    DragonSlayer,
    Efficient,
    Energising,
    EnhancedDevoted,
    EnhancedEfficient,
    Enlightened,
    Equilibrium,
    Fatiguing,
    Flanking,
    Fortune,
    Furnace,
    Genocidal,
    GlowWorm,
    Hallucinogenic,
    Hoarding,
    Honed,
    Impatient,
    ImpSouled,
    Inaccurate,
    Invigorating,
    JunkFood,
    Looting,
    Lucky,
    Lunging,
    Mediocrity,
    Mobile,
    Mysterious,
    NoEffect,
    PlantedFeet,
    Polishing,
    Precise,
    Preparation,
    Profane,
    Prosper,
    Pyromaniac,
    Rapid,
    Refined,
    Reflexes,
    Relentless,
    Ruthless,
    Scavenging,
    ShieldBashing,
    Spendthrift,
    Talking,
    Taunting,
    Tinker,
    TrophyTaker,
    Turtling,
    Ultimatums,
    UndeadBait,
    UndeadSlayer,
    Venomblood,
    Wise,
}

static mut USE_SIMPLE_PRINT_STYLE: AtomicBool = AtomicBool::new(false);

impl PerkName {
    pub const A: PerkName = PerkName::Absorbative;
    pub const B: PerkName = PerkName::Aftershock;
    pub const C: PerkName = PerkName::Antitheism;
    pub const D: PerkName = PerkName::Biting;
    pub const E: PerkName = PerkName::Blunted;
    pub const F: PerkName = PerkName::Brassican;
    pub const G: PerkName = PerkName::Breakdown;
    pub const H: PerkName = PerkName::BriefRespite;
    pub const I: PerkName = PerkName::Bulwark;
    pub const J: PerkName = PerkName::Butterfingers;
    pub const K: PerkName = PerkName::Caroming;

    pub fn using_simplified_names() {
        unsafe {
            *USE_SIMPLE_PRINT_STYLE.get_mut() = true;
        }
    }

    pub fn using_full_names() {
        unsafe {
            *USE_SIMPLE_PRINT_STYLE.get_mut() = false;
        }
    }
}

impl fmt::Display for PerkName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        if unsafe { *USE_SIMPLE_PRINT_STYLE.get_mut() } {
            match *self {
                PerkName::Empty => write!(f, "Empty"),
                PerkName::A => write!(f, "A"),
                PerkName::B => write!(f, "B"),
                PerkName::C => write!(f, "C"),
                PerkName::D => write!(f, "D"),
                PerkName::E => write!(f, "E"),
                PerkName::F => write!(f, "F"),
                PerkName::G => write!(f, "G"),
                PerkName::H => write!(f, "H"),
                PerkName::I => write!(f, "I"),
                PerkName::J => write!(f, "J"),
                PerkName::K => write!(f, "K"),
                _ => write!(f, "Empty")
            }
        } else {
            match self {
                PerkName::Empty => write!(f, "Empty"),
                PerkName::Absorbative => write!(f, "Absorbative"),
                PerkName::Aftershock => write!(f, "Aftershock"),
                PerkName::Antitheism => write!(f, "Antitheism"),
                PerkName::Biting => write!(f, "Biting"),
                PerkName::Blunted => write!(f, "Blunted"),
                PerkName::Brassican => write!(f, "Brassican"),
                PerkName::Breakdown => write!(f, "Breakdown"),
                PerkName::BriefRespite => write!(f, "Brief Respite"),
                PerkName::Bulwark => write!(f, "Bulwark"),
                PerkName::Butterfingers => write!(f, "Butterfingers"),
                PerkName::Caroming => write!(f, "Caroming"),
                PerkName::Cautious => write!(f, "Cautious"),
                PerkName::Charitable => write!(f, "Charitable"),
                PerkName::Cheapskate => write!(f, "Cheapskate"),
                PerkName::ClearHeaded => write!(f, "Clear Headed"),
                PerkName::Committed => write!(f, "Committed"),
                PerkName::Confused => write!(f, "Confused"),
                PerkName::Crackling => write!(f, "Crackling"),
                PerkName::CrystalShield => write!(f, "Crystal Shield"),
                PerkName::DemonBait => write!(f, "Demon Bait"),
                PerkName::DemonSlayer => write!(f, "Demon Slayer"),
                PerkName::Devoted => write!(f, "Devoted"),
                PerkName::DragonBait => write!(f, "Dragon Bait"),
                PerkName::DragonSlayer => write!(f, "Dragon Slayer"),
                PerkName::Efficient => write!(f, "Efficient"),
                PerkName::Energising => write!(f, "Energising"),
                PerkName::EnhancedDevoted => write!(f, "Enhanced Devoted"),
                PerkName::EnhancedEfficient => write!(f, "Enhanced Efficient"),
                PerkName::Enlightened => write!(f, "Enlightened"),
                PerkName::Equilibrium => write!(f, "Equilibrium"),
                PerkName::Fatiguing => write!(f, "Fatiguing"),
                PerkName::Flanking => write!(f, "Flanking"),
                PerkName::Fortune => write!(f, "Fortune"),
                PerkName::Furnace => write!(f, "Furnace"),
                PerkName::Genocidal => write!(f, "Genocidal"),
                PerkName::GlowWorm => write!(f, "Glow Worm"),
                PerkName::Hallucinogenic => write!(f, "Hallucinogenic"),
                PerkName::Hoarding => write!(f, "Hoarding"),
                PerkName::Honed => write!(f, "Honed"),
                PerkName::Impatient => write!(f, "Impatient"),
                PerkName::ImpSouled => write!(f, "Imp Souled"),
                PerkName::Inaccurate => write!(f, "Inaccurate"),
                PerkName::Invigorating => write!(f, "Invigorating"),
                PerkName::JunkFood => write!(f, "Junk Food"),
                PerkName::Looting => write!(f, "Looting"),
                PerkName::Lucky => write!(f, "Lucky"),
                PerkName::Lunging => write!(f, "Lunging"),
                PerkName::Mediocrity => write!(f, "Mediocrity"),
                PerkName::Mobile => write!(f, "Mobile"),
                PerkName::Mysterious => write!(f, "Mysterious"),
                PerkName::NoEffect => write!(f, "No effect"),
                PerkName::PlantedFeet => write!(f, "Planted Feet"),
                PerkName::Polishing => write!(f, "Polishing"),
                PerkName::Precise => write!(f, "Precise"),
                PerkName::Preparation => write!(f, "Preparation"),
                PerkName::Profane => write!(f, "Profane"),
                PerkName::Prosper => write!(f, "Prosper"),
                PerkName::Pyromaniac => write!(f, "Pyromaniac"),
                PerkName::Rapid => write!(f, "Rapid"),
                PerkName::Refined => write!(f, "Refined"),
                PerkName::Reflexes => write!(f, "Reflexes"),
                PerkName::Relentless => write!(f, "Relentless"),
                PerkName::Ruthless => write!(f, "Ruthless"),
                PerkName::Scavenging => write!(f, "Scavenging"),
                PerkName::ShieldBashing => write!(f, "Shield Bashing"),
                PerkName::Spendthrift => write!(f, "Spendthrift"),
                PerkName::Talking => write!(f, "Talking"),
                PerkName::Taunting => write!(f, "Taunting"),
                PerkName::Tinker => write!(f, "Tinker"),
                PerkName::TrophyTaker => write!(f, "Trophy-taker's"),
                PerkName::Turtling => write!(f, "Turtling"),
                PerkName::Ultimatums => write!(f, "Ultimatums"),
                PerkName::UndeadBait => write!(f, "Undead Bait"),
                PerkName::UndeadSlayer => write!(f, "Undead Slayer"),
                PerkName::Venomblood => write!(f, "Venomblood"),
                PerkName::Wise => write!(f, "Wise"),
            }
        }
    }
}

impl std::fmt::Debug for PerkName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl FromStr for PerkName {
    type Err = &'static str;

    fn from_str(perk: &str) -> Result<Self, Self::Err> {
        match perk.to_lowercase().as_str() {
            "empty" => Ok(PerkName::Empty),
            "absorbative" => Ok(PerkName::Absorbative),
            "aftershock" => Ok(PerkName::Aftershock),
            "antitheism" => Ok(PerkName::Antitheism),
            "biting" => Ok(PerkName::Biting),
            "blunted" => Ok(PerkName::Blunted),
            "brassican" => Ok(PerkName::Brassican),
            "breakdown" => Ok(PerkName::Breakdown),
            "brief respite" => Ok(PerkName::BriefRespite),
            "bulwark" => Ok(PerkName::Bulwark),
            "butterfingers" => Ok(PerkName::Butterfingers),
            "caroming" => Ok(PerkName::Caroming),
            "cautious" => Ok(PerkName::Cautious),
            "charitable" => Ok(PerkName::Charitable),
            "cheapskate" => Ok(PerkName::Cheapskate),
            "clear headed" => Ok(PerkName::ClearHeaded),
            "committed" => Ok(PerkName::Committed),
            "confused" => Ok(PerkName::Confused),
            "crackling" => Ok(PerkName::Crackling),
            "crystal shield" => Ok(PerkName::CrystalShield),
            "demon bait" => Ok(PerkName::DemonBait),
            "demon slayer" => Ok(PerkName::DemonSlayer),
            "devoted" => Ok(PerkName::Devoted),
            "dragon bait" => Ok(PerkName::DragonBait),
            "dragon slayer" => Ok(PerkName::DragonSlayer),
            "efficient" => Ok(PerkName::Efficient),
            "energising" => Ok(PerkName::Energising),
            "enhanced devoted" => Ok(PerkName::EnhancedDevoted),
            "enhanced efficient" => Ok(PerkName::EnhancedEfficient),
            "enlightened" => Ok(PerkName::Enlightened),
            "equilibrium" => Ok(PerkName::Equilibrium),
            "fatiguing" => Ok(PerkName::Fatiguing),
            "flanking" => Ok(PerkName::Flanking),
            "fortune" => Ok(PerkName::Fortune),
            "furnace" => Ok(PerkName::Furnace),
            "genocidal" => Ok(PerkName::Genocidal),
            "glow worm" => Ok(PerkName::GlowWorm),
            "hallucinogenic" => Ok(PerkName::Hallucinogenic),
            "hoarding" => Ok(PerkName::Hoarding),
            "honed" => Ok(PerkName::Honed),
            "imp souled" => Ok(PerkName::ImpSouled),
            "impatient" => Ok(PerkName::Impatient),
            "inaccurate" => Ok(PerkName::Inaccurate),
            "invigorating" => Ok(PerkName::Invigorating),
            "junk food" => Ok(PerkName::JunkFood),
            "looting" => Ok(PerkName::Looting),
            "lucky" => Ok(PerkName::Lucky),
            "lunging" => Ok(PerkName::Lunging),
            "mediocrity" => Ok(PerkName::Mediocrity),
            "mobile" => Ok(PerkName::Mobile),
            "mysterious" => Ok(PerkName::Mysterious),
            "no effect" => Ok(PerkName::NoEffect),
            "planted feet" => Ok(PerkName::PlantedFeet),
            "polishing" => Ok(PerkName::Polishing),
            "precise" => Ok(PerkName::Precise),
            "preparation" => Ok(PerkName::Preparation),
            "profane" => Ok(PerkName::Profane),
            "prosper" => Ok(PerkName::Prosper),
            "pyromaniac" => Ok(PerkName::Pyromaniac),
            "rapid" => Ok(PerkName::Rapid),
            "refined" => Ok(PerkName::Refined),
            "reflexes" => Ok(PerkName::Reflexes),
            "relentless" => Ok(PerkName::Relentless),
            "ruthless" => Ok(PerkName::Ruthless),
            "scavenging" => Ok(PerkName::Scavenging),
            "shield bashing" => Ok(PerkName::ShieldBashing),
            "spendthrift" => Ok(PerkName::Spendthrift),
            "talking" => Ok(PerkName::Talking),
            "taunting" => Ok(PerkName::Taunting),
            "tinker" => Ok(PerkName::Tinker),
            "trophy taker" => Ok(PerkName::TrophyTaker),
            "trophy-taker's" => Ok(PerkName::TrophyTaker),
            "trophy-taker" => Ok(PerkName::TrophyTaker),
            "turtling" => Ok(PerkName::Turtling),
            "ultimatums" => Ok(PerkName::Ultimatums),
            "undead bait" => Ok(PerkName::UndeadBait),
            "undead slayer" => Ok(PerkName::UndeadSlayer),
            "venomblood" => Ok(PerkName::Venomblood),
            "wise" => Ok(PerkName::Wise),
            _ => Err("Unknown perk")
        }
    }
}

impl From<PerkName> for usize {
    fn from(value: PerkName) -> Self {
        value as usize
    }
}

impl std::default::Default for PerkName {
    fn default() -> Self {
        PerkName::Empty
    }
}