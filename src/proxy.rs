use crate::proxy::GameMode::{Creative, Spectator};
// TODO a nice proc macro that does all this magic LMAO
use crate::tag::Tag;
//  https://minecraft.fandom.com/wiki/Java_Edition_level_format#level.dat_format

pub trait FromTag {
    fn from_tag(tag: Tag) -> Self;
}

pub trait ToTag {
    fn to_tag(self) -> Tag;
}

pub trait FromValue<T> {
    fn from(value: T) -> Option<Self>;
}

struct Point<T> {
    x: T,
    y: T,
    z: T
}

struct Border {
    center: Point<f64>,
    damage_per_block: f64,
    size: f64,
    safe_zone: f64,
    size_lerp_target: f64,
    size_lerp_time: i64,
    warning_blocks: f64,
    warning_time: f64
}

type UUID = [i32; 4];

pub enum Overlay {
    Progress,
    Notched6,
    Notched10,
    Notched12,
    Notched20
}

struct BossEvent {
    id: String,
    players: Vec<UUID>,
    color: String,
    create_world_fog: bool,
    darken_screen: bool,
    max_health: i32,
    current_health: i32,
    name: String,
    overlay: Overlay,
    play_boss_music: bool,
    is_visible: bool
}

type Datapack = String;

struct Datapacks {
    disabled: Vec<Datapack>,
    enabled: Vec<Datapack>
}

struct WeatherData {
    clear_weather_time: i32,
    is_raining: bool,
    rain_time: i32,
    is_thundering: bool,
    thunder_time: i32,
}

pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator
}

impl FromValue<u8> for GameMode {
    fn from(value: u8) -> Option<Self> {
        use GameMode::*;
        match value {
            0 => Some(Survival),
            1 => Some(Creative),
            2 => Some(Adventure),
            3 => Some(Spectator),
            _ => None
        }
    }
}

struct GeneratorDimensionData {  // TODO
    id: String

}


pub enum GeneratorName {
    Default,
    Flat,
    LargeBiomes,
    Amplified,
    Buffet,
    DebugAllBlockStates,
    Default11,
    Customized
}

impl FromValue<String> for GeneratorName {
    fn from(value: String) -> Option<Self> {
        use GeneratorName::*;
        match value.as_str() {
            "default" => Some(Default),
            "flat" => Some(Flat),
            "largeBiomes" => Some(LargeBiomes),
            "amplified" => Some(Amplified),
            "buffet" => Some(Buffet),
            "debug_all_block_states" => Some(DebugAllBlockStates),
            "default_1_1" => Some(Default11),
            "customized" => Some(Customized),
            _ => None
        }
    }
}

struct GeneratorOptions {
    // FFS TODO
}

struct WorldGenerator {
    generator_name: GeneratorName,
    bonus_chest: bool,
    seed: i64,
    generate_features: bool,
    dimensions: Vec<GeneratorDimensionData>,
    options: GeneratorOptions
}

pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard
}

type Percent = f32;

struct WanderingTraderData {
    uuid: UUID,
    spawn_chance: Percent,
    spawn_delay: i32,
}


struct WorldData {
    border: Border,
    allow_commands: bool,
    time_of_day: i64,
    difficulty: Difficulty,
    difficulty_locked: bool,
    world_spawn_point: Point<i32>,
    time: i64,
    game_mode: GameMode,
    is_hardcore: bool,
    seed: i64,
    generator: WorldGenerator,
    player: Option<PlayerData>,
    game_rules: Vec<GameRule>,
    name: String,
    wandering_trader: WanderingTraderData,
    dimension_data: DimensionData
}

struct LevelVersion {
    nbt_version: i32,
    id: i32,
    name: String,
    series: String,
    is_snapchat: bool
}

struct PlayerData {  // TODO

}

pub enum GameRuleValue {
    Bool(bool),
    Number(i128),
    String(String)
}

impl FromValue<String> for GameRuleValue {
    fn from(value: String) -> Option<Self> {
        let s = value.as_str();
        if s == "true" || s == "false" {
            return Some(GameRuleValue::Bool(s == "true"));
        }
        None
    }
}



struct GameRule {
    name: String,
    value: GameRuleValue
}

struct DragonFight {
    exit_portal: Point<i8>,
    gate_ways: Vec<i32>,
    dragon_is_dead: bool,
    dragon_uuid: UUID,
    dragon_previously_killed: bool
}


struct DimensionData {
    end_data: DragonFight
}

struct Data {
    custom_boss_events: Vec<BossEvent>,
    world: WorldData,
    datapacks: Datapacks,
    data_version: i32,
    version: LevelVersion,
    was_modded: bool,
    has_initialized: bool

}

struct Level {
    data: Data
}

