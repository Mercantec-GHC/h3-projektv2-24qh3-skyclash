use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct HeroStats {
    pub strength: i64,
    pub agility: i64,
    pub defence: i64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CreateHeroParams {
    pub rfid: String,
    pub hero_type: i64,
    pub base_stats: HeroStats,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateHeroStatsParams {
    pub rfid: String,
    pub stats: HeroStats,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Hero {
    pub id: i64,
    pub rfid: String,
    pub level: i64,
    pub hero_type: i64,
    pub unallocated_skillpoints: i64,
    pub strength_points: i64,
    pub agility_points: i64,
    pub defence_points: i64,
}

pub trait Database {
    async fn create_hero(&mut self, hero: CreateHeroParams) -> Result<(), eyre::Report>;
    async fn update_hero_stats(&mut self, hero: UpdateHeroStatsParams) -> Result<(), eyre::Report>;
    async fn hero_by_rfid(&mut self, rfid: &str) -> Result<Option<Hero>, eyre::Report>;
}
