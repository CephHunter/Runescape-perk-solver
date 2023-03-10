use std::{fmt, str::FromStr};
use itertools::Itertools;
use colored::*;
use strum_macros::{EnumIter, EnumCount};

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, EnumIter, EnumCount)]
pub enum MaterialName {
    ArmadylComponents,
    AscendedComponents,
    AvernicComponents,
    BandosComponents,
    BaseParts,
    BladeParts,
    BrassicanComponents,
    ClassicComponents,
    ClearParts,
    ClockworkComponents,
    ConnectorParts,
    CorporealComponents,
    CoverParts,
    CraftedParts,
    CrystalParts,
    CulinaryComponents,
    CywirComponents,
    DeflectingParts,
    DelicateParts,
    DextrousComponents,
    DirectComponents,
    DragonfireComponents,
    EnhancingComponents,
    EtherealComponents,
    EvasiveComponents,
    ExplosiveComponents,
    FacetedComponents,
    FlexibleParts,
    FortunateComponents,
    FungalComponents,
    HarnessedComponents,
    HeadParts,
    HealthyComponents,
    HeavyComponents,
    HistoricComponents,
    IlujankanComponents,
    ImbuedComponents,
    Junk,
    KnightlyComponents,
    LightComponents,
    LivingComponents,
    MagicParts,
    MetallicParts,
    NoxiousComponents,
    OceanicComponents,
    OrganicParts,
    PaddedParts,
    PestiferousComponents,
    PiousComponents,
    PlatedParts,
    PowerfulComponents,
    PreciousComponents,
    PreciseComponents,
    ProtectiveComponents,
    RefinedComponents,
    ResilientComponents,
    RumblingComponents,
    SaradominComponents,
    SerenComponents,
    ShadowComponents,
    SharpComponents,
    ShiftingComponents,
    SilentComponents,
    SimpleParts,
    SmoothParts,
    SpikedParts,
    SpiritualParts,
    StaveParts,
    StrongComponents,
    StunningComponents,
    SubtleComponents,
    SwiftComponents,
    TensileParts,
    ThirdAgeComponents,
    TimewornComponents,
    UndeadComponents,
    VariableComponents,
    VintageComponents,
    ZamorakComponents,
    ZarosComponents,
}

impl MaterialName {
    pub fn vec_to_string(v: &[MaterialName]) -> String {
        let counts = v.iter().counts();
        v.iter().unique().map(|x| {
            let count = *counts.get(x).unwrap();
            format!("{} ?? {}", count, x)
        }).join(", ")
    }

    pub fn vec_to_string_colored(v: &[MaterialName]) -> String {
        let counts = v.iter().counts();
        v.iter().unique().map(|x| {
            let count = *counts.get(x).unwrap();
            format!("{} ?? {}", count, x.to_string().cyan())
        }).join(", ")
    }
}

impl serde::Serialize for MaterialName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.collect_str(&self)
    }
}

impl fmt::Display for MaterialName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaterialName::ArmadylComponents => write!(f, "Armadyl components"),
            MaterialName::AscendedComponents => write!(f, "Ascended components"),
            MaterialName::AvernicComponents => write!(f, "Avernic components"),
            MaterialName::BandosComponents => write!(f, "Bandos components"),
            MaterialName::BaseParts => write!(f, "Base parts"),
            MaterialName::BladeParts => write!(f, "Blade parts"),
            MaterialName::BrassicanComponents => write!(f, "Brassican components"),
            MaterialName::ClassicComponents => write!(f, "Classic components"),
            MaterialName::ClearParts => write!(f, "Clear parts"),
            MaterialName::ClockworkComponents => write!(f, "Clockwork components"),
            MaterialName::ConnectorParts => write!(f, "Connector parts"),
            MaterialName::CorporealComponents => write!(f, "Corporeal components"),
            MaterialName::CoverParts => write!(f, "Cover parts"),
            MaterialName::CraftedParts => write!(f, "Crafted parts"),
            MaterialName::CrystalParts => write!(f, "Crystal parts"),
            MaterialName::CulinaryComponents => write!(f, "Culinary components"),
            MaterialName::CywirComponents => write!(f, "Cywir components"),
            MaterialName::DeflectingParts => write!(f, "Deflecting parts"),
            MaterialName::DelicateParts => write!(f, "Delicate parts"),
            MaterialName::DextrousComponents => write!(f, "Dextrous components"),
            MaterialName::DirectComponents => write!(f, "Direct components"),
            MaterialName::DragonfireComponents => write!(f, "Dragonfire components"),
            MaterialName::EnhancingComponents => write!(f, "Enhancing components"),
            MaterialName::EtherealComponents => write!(f, "Ethereal components"),
            MaterialName::EvasiveComponents => write!(f, "Evasive components"),
            MaterialName::ExplosiveComponents => write!(f, "Explosive components"),
            MaterialName::FacetedComponents => write!(f, "Faceted components"),
            MaterialName::FlexibleParts => write!(f, "Flexible parts"),
            MaterialName::FortunateComponents => write!(f, "Fortunate components"),
            MaterialName::FungalComponents => write!(f, "Fungal components"),
            MaterialName::HarnessedComponents => write!(f, "Harnessed components"),
            MaterialName::HeadParts => write!(f, "Head parts"),
            MaterialName::HealthyComponents => write!(f, "Healthy components"),
            MaterialName::HeavyComponents => write!(f, "Heavy components"),
            MaterialName::HistoricComponents => write!(f, "Historic components"),
            MaterialName::IlujankanComponents => write!(f, "Ilujankan components"),
            MaterialName::ImbuedComponents => write!(f, "Imbued components"),
            MaterialName::Junk => write!(f, "Junk"),
            MaterialName::KnightlyComponents => write!(f, "Knightly components"),
            MaterialName::LightComponents => write!(f, "Light components"),
            MaterialName::LivingComponents => write!(f, "Living components"),
            MaterialName::MagicParts => write!(f, "Magic parts"),
            MaterialName::MetallicParts => write!(f, "Metallic parts"),
            MaterialName::NoxiousComponents => write!(f, "Noxious components"),
            MaterialName::OceanicComponents => write!(f, "Oceanic components"),
            MaterialName::OrganicParts => write!(f, "Organic parts"),
            MaterialName::PaddedParts => write!(f, "Padded parts"),
            MaterialName::PestiferousComponents => write!(f, "Pestiferous components"),
            MaterialName::PiousComponents => write!(f, "Pious components"),
            MaterialName::PlatedParts => write!(f, "Plated parts"),
            MaterialName::PowerfulComponents => write!(f, "Powerful components"),
            MaterialName::PreciousComponents => write!(f, "Precious components"),
            MaterialName::PreciseComponents => write!(f, "Precise components"),
            MaterialName::ProtectiveComponents => write!(f, "Protective components"),
            MaterialName::RefinedComponents => write!(f, "Refined components"),
            MaterialName::ResilientComponents => write!(f, "Resilient components"),
            MaterialName::RumblingComponents => write!(f, "Rumbling components"),
            MaterialName::SaradominComponents => write!(f, "Saradomin components"),
            MaterialName::SerenComponents => write!(f, "Seren components"),
            MaterialName::ShadowComponents => write!(f, "Shadow components"),
            MaterialName::SharpComponents => write!(f, "Sharp components"),
            MaterialName::ShiftingComponents => write!(f, "Shifting components"),
            MaterialName::SilentComponents => write!(f, "Silent components"),
            MaterialName::SimpleParts => write!(f, "Simple parts"),
            MaterialName::SmoothParts => write!(f, "Smooth parts"),
            MaterialName::SpikedParts => write!(f, "Spiked parts"),
            MaterialName::SpiritualParts => write!(f, "Spiritual parts"),
            MaterialName::StaveParts => write!(f, "Stave parts"),
            MaterialName::StrongComponents => write!(f, "Strong components"),
            MaterialName::StunningComponents => write!(f, "Stunning components"),
            MaterialName::SubtleComponents => write!(f, "Subtle components"),
            MaterialName::SwiftComponents => write!(f, "Swift components"),
            MaterialName::TensileParts => write!(f, "Tensile parts"),
            MaterialName::ThirdAgeComponents => write!(f, "Third-age components"),
            MaterialName::TimewornComponents => write!(f, "Timeworn components"),
            MaterialName::UndeadComponents => write!(f, "Undead components"),
            MaterialName::VariableComponents => write!(f, "Variable components"),
            MaterialName::VintageComponents => write!(f, "Vintage components"),
            MaterialName::ZamorakComponents => write!(f, "Zamorak components"),
            MaterialName::ZarosComponents => write!(f, "Zaros components"),
        }
    }
}

impl FromStr for MaterialName {
    type Err = &'static str;

    fn from_str(mat: &str) -> Result<Self, Self::Err> {
        fn find(mat: &str) -> Result<MaterialName, &'static str> {
            match mat.to_lowercase().as_str() {
                "armadyl components" => Ok(MaterialName::ArmadylComponents),
                "ascended components" => Ok(MaterialName::AscendedComponents),
                "avernic components" => Ok(MaterialName::AvernicComponents),
                "bandos components" => Ok(MaterialName::BandosComponents),
                "base parts" => Ok(MaterialName::BaseParts),
                "blade parts" => Ok(MaterialName::BladeParts),
                "brassican components" => Ok(MaterialName::BrassicanComponents),
                "classic components" => Ok(MaterialName::ClassicComponents),
                "clear parts" => Ok(MaterialName::ClearParts),
                "clockwork components" => Ok(MaterialName::ClockworkComponents),
                "connector parts" => Ok(MaterialName::ConnectorParts),
                "corporeal components" => Ok(MaterialName::CorporealComponents),
                "cover parts" => Ok(MaterialName::CoverParts),
                "crafted parts" => Ok(MaterialName::CraftedParts),
                "crystal parts" => Ok(MaterialName::CrystalParts),
                "culinary components" => Ok(MaterialName::CulinaryComponents),
                "cywir components" => Ok(MaterialName::CywirComponents),
                "deflecting parts" => Ok(MaterialName::DeflectingParts),
                "delicate parts" => Ok(MaterialName::DelicateParts),
                "dextrous components" => Ok(MaterialName::DextrousComponents),
                "direct components" => Ok(MaterialName::DirectComponents),
                "dragonfire components" => Ok(MaterialName::DragonfireComponents),
                "enhancing components" => Ok(MaterialName::EnhancingComponents),
                "ethereal components" => Ok(MaterialName::EtherealComponents),
                "evasive components" => Ok(MaterialName::EvasiveComponents),
                "explosive components" => Ok(MaterialName::ExplosiveComponents),
                "faceted components" => Ok(MaterialName::FacetedComponents),
                "flexible parts" => Ok(MaterialName::FlexibleParts),
                "fortunate components" => Ok(MaterialName::FortunateComponents),
                "fungal components" => Ok(MaterialName::FungalComponents),
                "harnessed components" => Ok(MaterialName::HarnessedComponents),
                "head parts" => Ok(MaterialName::HeadParts),
                "healthy components" => Ok(MaterialName::HealthyComponents),
                "heavy components" => Ok(MaterialName::HeavyComponents),
                "historic components" => Ok(MaterialName::HistoricComponents),
                "ilujankan components" => Ok(MaterialName::IlujankanComponents),
                "imbued components" => Ok(MaterialName::ImbuedComponents),
                "junk" => Ok(MaterialName::Junk),
                "knightly components" => Ok(MaterialName::KnightlyComponents),
                "light components" => Ok(MaterialName::LightComponents),
                "living components" => Ok(MaterialName::LivingComponents),
                "magic parts" => Ok(MaterialName::MagicParts),
                "metallic parts" => Ok(MaterialName::MetallicParts),
                "noxious components" => Ok(MaterialName::NoxiousComponents),
                "oceanic components" => Ok(MaterialName::OceanicComponents),
                "organic parts" => Ok(MaterialName::OrganicParts),
                "padded parts" => Ok(MaterialName::PaddedParts),
                "pestiferous components" => Ok(MaterialName::PestiferousComponents),
                "pious components" => Ok(MaterialName::PiousComponents),
                "plated parts" => Ok(MaterialName::PlatedParts),
                "powerful components" => Ok(MaterialName::PowerfulComponents),
                "precious components" => Ok(MaterialName::PreciousComponents),
                "precise components" => Ok(MaterialName::PreciseComponents),
                "protective components" => Ok(MaterialName::ProtectiveComponents),
                "refined components" => Ok(MaterialName::RefinedComponents),
                "resilient components" => Ok(MaterialName::ResilientComponents),
                "rumbling components" => Ok(MaterialName::RumblingComponents),
                "saradomin components" => Ok(MaterialName::SaradominComponents),
                "seren components" => Ok(MaterialName::SerenComponents),
                "shadow components" => Ok(MaterialName::ShadowComponents),
                "sharp components" => Ok(MaterialName::SharpComponents),
                "shifting components" => Ok(MaterialName::ShiftingComponents),
                "silent components" => Ok(MaterialName::SilentComponents),
                "simple parts" => Ok(MaterialName::SimpleParts),
                "smooth parts" => Ok(MaterialName::SmoothParts),
                "spiked parts" => Ok(MaterialName::SpikedParts),
                "spiritual parts" => Ok(MaterialName::SpiritualParts),
                "stave parts" => Ok(MaterialName::StaveParts),
                "strong components" => Ok(MaterialName::StrongComponents),
                "stunning components" => Ok(MaterialName::StunningComponents),
                "subtle components" => Ok(MaterialName::SubtleComponents),
                "swift components" => Ok(MaterialName::SwiftComponents),
                "tensile parts" => Ok(MaterialName::TensileParts),
                "third-age components" => Ok(MaterialName::ThirdAgeComponents),
                "timeworn components" => Ok(MaterialName::TimewornComponents),
                "undead components" => Ok(MaterialName::UndeadComponents),
                "variable components" => Ok(MaterialName::VariableComponents),
                "vintage components" => Ok(MaterialName::VintageComponents),
                "zamorak components" => Ok(MaterialName::ZamorakComponents),
                "zaros components" => Ok(MaterialName::ZarosComponents),
                _ => Err("Unknown material name")
            }
        }

        let mut x = find(mat);
        if x.is_err() {
            x = find(format!("{} parts", mat).as_str());
        }
        if x.is_err() {
            x = find(format!("{} components", mat).as_str());
        }
        x
    }
}

impl From<MaterialName> for usize {
    fn from(value: MaterialName) -> Self {
        value as usize
    }
}

impl std::default::Default for MaterialName {
    fn default() -> Self {
        MaterialName::Junk
    }
}