use std::path::PathBuf;

use shared::HeroKind;

pub struct HeroInfo {
    pub base_stats: shared::HeroStats,
    pub texture_path: PathBuf,
    pub name: String,
}

impl From<HeroKind> for HeroInfo {
    fn from(value: HeroKind) -> Self {
        Self::from(&value)
    }
}

impl From<&HeroKind> for HeroInfo {
    fn from(value: &HeroKind) -> Self {
        use shared::HeroKind::{Centrist, Speed, Strong, Tankie};
        let base_stats = shared::HeroStats::from(value);
        let name = match value {
            Centrist => "Centrist",
            Strong => "Strong",
            Speed => "Speed",
            Tankie => "Tankie",
        }
        .to_string();
        let texture_path = match value {
            Centrist => PathBuf::from("./assets/heroes/centrist.png"),
            Strong => PathBuf::from("./assets/heroes/strong.png"),
            Speed => PathBuf::from("./assets/heroes/speed.png"),
            Tankie => PathBuf::from("./assets/heroes/tankie.png"),
        };
        Self {
            base_stats,
            texture_path,
            name,
        }
    }
}
