use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::primitives::Byte;

#[repr(u8)]
#[derive(Default, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum RacePreference {
    Human = 0x01,
    Orc = 0x02,
    Nightelf = 0x04,
    Undead = 0x08,
    Demon = 0x10,
    #[default]
    Random = 0x20,
    UserSelectable = 0x40,
}

impl From<Byte> for RacePreference {
    fn from(value: Byte) -> Self {
        use RacePreference::*;
        // UserSelectable is masked into the value in memory, so just add its value 0x40
        match value.get_byte() {
            0x41 | 0x01 => Human,
            0x42 | 0x02 => Orc,
            0x44 | 0x04 => Nightelf,
            0x48 | 0x08 => Undead,
            0x50 | 0x10 => Demon,
            0x60 | 0x20 => Random,
            _ => Self::default(),
        }
    }
}

#[repr(u8)]
#[derive(Default, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum PlayerRace {
    #[default]
    Unknown = 0,
    Human = 1,
    Orc = 2,
    Undead = 3,
    NightElf = 4,
    Demon = 5,
    Last = 6,
    Other = 7,
    Creep = 8,
    Commoner = 9,
    Critter = 10,
    Naga = 11,
}

impl From<Byte> for PlayerRace {
    fn from(value: Byte) -> Self {
        use PlayerRace::*;

        match value.get_byte() {
            0 => Unknown,
            1 => Human,
            2 => Orc,
            3 => Undead,
            4 => NightElf,
            5 => Demon,
            6 => Last,
            7 => Other,
            8 => Creep,
            9 => Commoner,
            10 => Critter,
            11 => Naga,
            _ => Unknown,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum PlayerType {
    Empty = 0,
    Player = 1,
    Computer = 2,
    Neutral = 3,
    Observer = 4,
    None = 5,
    Other = 6,
}

impl From<Byte> for PlayerType {
    fn from(value: Byte) -> Self {
        use PlayerType::*;

        match value.get_byte() {
            0 => Empty,
            1 => Player,
            2 => Computer,
            3 => Neutral,
            4 => Observer,
            5 => None,
            6 => Other,
            _ => Empty,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum PlayerGameResult {
    Victory = 0,
    Defeat = 1,
    Tie = 2,
    Neutral = 3,
}

impl From<Byte> for PlayerGameResult {
    fn from(value: Byte) -> Self {
        use PlayerGameResult::*;

        match value.get_byte() {
            0 => Victory,
            1 => Defeat,
            2 => Tie,
            3 => Neutral,
            _ => Neutral,
        }
    }
}

#[repr(u8)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum PlayerSlotState {
    #[default]
    Empty = 0,
    Playing = 1,
    Left = 2,
}

impl From<Byte> for PlayerSlotState {
    fn from(value: Byte) -> Self {
        use PlayerSlotState::*;

        match value.get_byte() {
            0 => Empty,
            1 => Playing,
            2 => Left,
            _ => Self::default(),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AiDifficultyPreference {
    Newbie = 0,
    Normal = 1,
    Insane = 2,
}

impl From<Byte> for AiDifficultyPreference {
    fn from(value: Byte) -> Self {
        use AiDifficultyPreference::*;

        match value.get_byte() {
            0 => Newbie,
            1 => Normal,
            2 => Insane,
            _ => Newbie,
        }
    }
}

#[repr(u8)]
#[derive(Default, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum PlayerColor {
    #[default]
    Red = 0,
    Blue = 1,
    Teal = 2,
    Purple = 3,
    Yellow = 4,
    Orange = 5,
    Green = 6,
    Pink = 7,
    Gray = 8,
    LightBlue = 9,
    DarkGreen = 10,
    Brown = 11,
    Maroon = 12,
    Navy = 13,
    Turquoise = 14,
    Violet = 15,
    Wheat = 16,
    Peach = 17,
    Mint = 18,
    Lavender = 19,
    Coal = 20,
    Snow = 21,
    Emerald = 22,
    Peanut = 23,
}

impl PlayerColor {
    pub fn color_code(&self) -> &'static str {
        match self {
            PlayerColor::Red => "cffff0303",
            PlayerColor::Blue => "cff0042ff",
            PlayerColor::Teal => "cff1be7ba",
            PlayerColor::Purple => "cff550081",
            PlayerColor::Yellow => "cfffefc00",
            PlayerColor::Orange => "cfffe890d",
            PlayerColor::Green => "cff21bf00",
            PlayerColor::Pink => "cffe45caf",
            PlayerColor::Gray => "cff939596",
            PlayerColor::LightBlue => "cff7ebff1",
            PlayerColor::DarkGreen => "cff106247",
            PlayerColor::Brown => "cff4f2b05",
            PlayerColor::Maroon => "cff9c0000",
            PlayerColor::Navy => "cff0000c3",
            PlayerColor::Turquoise => "cff00ebff",
            PlayerColor::Violet => "cffbd00ff",
            PlayerColor::Wheat => "cffecce87",
            PlayerColor::Peach => "cfff7a58b",
            PlayerColor::Mint => "cffbfff81",
            PlayerColor::Lavender => "cffdbb8eb",
            PlayerColor::Coal => "cff4f5055",
            PlayerColor::Snow => "cffecf0ff",
            PlayerColor::Emerald => "cff00781e",
            PlayerColor::Peanut => "cffa56f34",
        }
    }

    pub fn rgba(&self) -> [f32; 4] {
        let code = self.color_code();
        let hex = &code[3..9];
        let red = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
        let green = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
        let blue = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);

        [
            f32::from(red) / 255.0,
            f32::from(green) / 255.0,
            f32::from(blue) / 255.0,
            1.0,
        ]
    }
}

impl From<Byte> for PlayerColor {
    fn from(value: Byte) -> Self {
        use PlayerColor::*;

        match value.get_byte() {
            0 => Red,
            1 => Blue,
            2 => Teal,
            3 => Purple,
            4 => Yellow,
            5 => Orange,
            6 => Green,
            7 => Pink,
            8 => Gray,
            9 => LightBlue,
            10 => DarkGreen,
            11 => Brown,
            12 => Maroon,
            13 => Navy,
            14 => Turquoise,
            15 => Violet,
            16 => Wheat,
            17 => Peach,
            18 => Mint,
            19 => Lavender,
            20 => Coal,
            21 => Snow,
            22 => Emerald,
            23 => Peanut,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchType {
    Melee(MeleeMatchType),
    Custom(CustomMatchType),
    Campaign(CampaignMatchType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeleeMatchType {
    OneVsOne,
    TwoVsTwo,
    ThreeVsThree,
    FourVsFour,
    FreeForAll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomMatchType {
    DirectStrike,
    Legion,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CampaignMatchType {}

pub type Team = BTreeMap<u32, TeamPlayer>;
pub type Teams = BTreeMap<u8, Team>;

#[derive(Default, Debug, Clone, Serialize)]
pub struct TeamPlayer {
    name: String,
    race_preference: RacePreference,
    state: PlayerSlotState,
    color: PlayerColor,
}

impl TeamPlayer {
    pub fn new(
        name: String,
        race_preference: RacePreference,
        state: PlayerSlotState,
        color: PlayerColor,
    ) -> Self {
        Self {
            name,
            race_preference,
            state,
            color,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn race_preference(&self) -> RacePreference {
        self.race_preference
    }

    pub fn state(&self) -> PlayerSlotState {
        self.state
    }

    pub fn color(&self) -> PlayerColor {
        self.color
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::Byte;

    #[test]
    fn player_color_rgba_alpha_is_always_one() {
        let colors = [
            PlayerColor::Red,
            PlayerColor::Blue,
            PlayerColor::Teal,
            PlayerColor::Peanut,
        ];
        for color in colors {
            let rgba = color.rgba();
            assert_eq!(rgba[3], 1.0, "alpha should be 1.0 for {color:?}");
        }
    }

    #[test]
    fn player_color_rgba_channels_are_zero_to_one() {
        for channel in PlayerColor::Red.rgba() {
            assert!((0.0..=1.0).contains(&channel));
        }
    }

    #[test]
    fn player_color_from_byte_round_trips_known_values() {
        assert!(matches!(
            PlayerColor::from(Byte::from(0u8)),
            PlayerColor::Red
        ));
        assert!(matches!(
            PlayerColor::from(Byte::from(1u8)),
            PlayerColor::Blue
        ));
        assert!(matches!(
            PlayerColor::from(Byte::from(23u8)),
            PlayerColor::Peanut
        ));
    }

    #[test]
    fn race_preference_from_byte_handles_user_selectable_mask() {
        // 0x41 = Human | UserSelectable
        assert!(matches!(
            RacePreference::from(Byte::from(0x41u8)),
            RacePreference::Human
        ));
        assert!(matches!(
            RacePreference::from(Byte::from(0x01u8)),
            RacePreference::Human
        ));
    }

    #[test]
    fn player_slot_state_default_is_empty() {
        assert_eq!(PlayerSlotState::default(), PlayerSlotState::Empty);
    }

    #[test]
    fn player_slot_state_from_byte_round_trips() {
        assert!(matches!(
            PlayerSlotState::from(Byte::from(1u8)),
            PlayerSlotState::Playing
        ));
        assert!(matches!(
            PlayerSlotState::from(Byte::from(2u8)),
            PlayerSlotState::Left
        ));
    }

    #[test]
    fn team_player_accessors_return_stored_values() {
        let player = TeamPlayer::new(
            String::from("Alice"),
            RacePreference::Human,
            PlayerSlotState::Playing,
            PlayerColor::Teal,
        );
        assert_eq!(player.name(), "Alice");
        assert!(matches!(player.state(), PlayerSlotState::Playing));
        assert!(matches!(player.color(), PlayerColor::Teal));
    }
}
