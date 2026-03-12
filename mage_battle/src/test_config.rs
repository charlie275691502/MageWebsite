use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use crate::attribute::AttributePoints;
use crate::buff::{Buff, BuffDuration, BuffType};
use crate::character::CharacterType;
use crate::game::{Game, TurnPhase};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub test_scenarios: Vec<TestScenario>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScenario {
    pub scenario_id: String,
    pub description: String,
    pub num_players: usize,
    pub players: Vec<TestPlayer>,
    pub current_player_index: usize,
    pub turn_phase: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestPlayer {
    pub name: String,
    pub character: String,
    pub hp: i32,
    pub shield: u32,
    pub attributes: TestAttributes,
    pub hand: Vec<u32>,
    pub buffs: Vec<TestBuff>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestAttributes {
    pub fire: u8,
    pub wood: u8,
    pub thunder: u8,
    pub water: u8,
    pub wind: u8,
    pub poison: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestBuff {
    pub buff_type: String,
    pub duration: TestBuffDuration,
    pub data: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestBuffDuration {
    Turns { Turns: u8 },
    UntilHit { UntilHit: u8 },
    UntilNextPlayer,
    Permanent,
}

impl TestConfig {
    /// Load test config from file
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: TestConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Get a test scenario by ID
    pub fn get_scenario(&self, scenario_id: &str) -> Option<&TestScenario> {
        self.test_scenarios
            .iter()
            .find(|s| s.scenario_id == scenario_id)
    }

    /// List all available test scenarios
    pub fn list_scenarios(&self) -> Vec<(&str, &str)> {
        self.test_scenarios
            .iter()
            .map(|s| (s.scenario_id.as_str(), s.description.as_str()))
            .collect()
    }
}

impl TestScenario {
    /// Create a Game instance from this test scenario
    pub fn create_game(&self) -> Result<Game, String> {
        if self.players.len() < 2 || self.players.len() > 4 {
            return Err(format!(
                "Invalid number of players: {}. Must be 2-4.",
                self.players.len()
            ));
        }

        if self.num_players != self.players.len() {
            return Err(format!(
                "num_players ({}) doesn't match actual players count ({})",
                self.num_players,
                self.players.len()
            ));
        }

        // Extract player names and character types
        let player_names: Vec<String> = self.players.iter().map(|p| p.name.clone()).collect();

        let character_types: Vec<CharacterType> = self
            .players
            .iter()
            .map(|p| match p.character.as_str() {
                "FlamePoison" => Ok(CharacterType::FlamePoison),
                "WoodWind" => Ok(CharacterType::WoodWind),
                "ThunderPoison" => Ok(CharacterType::ThunderPoison),
                "WaterWind" => Ok(CharacterType::WaterWind),
                _ => Err(format!("Unknown character type: {}", p.character)),
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Create the game
        let mut game = Game::new(player_names, character_types);

        // Apply test configuration to each player
        for (i, test_player) in self.players.iter().enumerate() {
            if i >= game.players.len() {
                return Err(format!("Player index {} out of bounds", i));
            }

            let player = &mut game.players[i];

            // Set HP and shield
            player.hp = test_player.hp;
            player.shield = test_player.shield;

            // Set attributes
            player.attributes = AttributePoints {
                fire: test_player.attributes.fire,
                wood: test_player.attributes.wood,
                thunder: test_player.attributes.thunder,
                water: test_player.attributes.water,
                wind: test_player.attributes.wind,
                poison: test_player.attributes.poison,
            };

            // Set hand cards
            player.hand.clear();
            for &card_id in &test_player.hand {
                player.hand.push(card_id);
            }

            // Apply buffs
            player.buffs.buffs.clear();
            for test_buff in &test_player.buffs {
                let buff_type = parse_buff_type(&test_buff.buff_type)?;
                let duration = parse_buff_duration(&test_buff.duration);

                let buff = if test_buff.data.is_some() {
                    Buff::new_with_data(buff_type, duration, test_buff.data.unwrap())
                } else {
                    Buff::new(buff_type, duration)
                };

                player.buffs.add(buff);
            }
        }

        // Set current player and turn phase
        game.current_player_index = self.current_player_index;
        game.turn_phase = parse_turn_phase(&self.turn_phase)?;

        Ok(game)
    }
}

/// Parse buff type from string
fn parse_buff_type(buff_type: &str) -> Result<BuffType, String> {
    match buff_type {
        "Immune" => Ok(BuffType::Immune),
        "Invincible" => Ok(BuffType::Invincible),
        "Regeneration" => Ok(BuffType::Regeneration),
        "BurningOut" => Ok(BuffType::BurningOut),
        "GuardWoodCarving" => Ok(BuffType::GuardWoodCarving),
        "HealthDrain" => Ok(BuffType::HealthDrain),
        "Paralysis" => Ok(BuffType::Paralysis),
        "Seal" => Ok(BuffType::Seal),
        "Silent" => Ok(BuffType::Silent),
        "MasterDisable" => Ok(BuffType::MasterDisable),
        "DefenseInvalidation" => Ok(BuffType::DefenseInvalidation),
        "Confuse" => Ok(BuffType::Confuse),
        "HealthDrainTarget" => Ok(BuffType::HealthDrainTarget),
        _ => Err(format!("Unknown buff type: {}", buff_type)),
    }
}

/// Parse buff duration from test config
fn parse_buff_duration(duration: &TestBuffDuration) -> BuffDuration {
    match duration {
        TestBuffDuration::Turns { Turns: n } => BuffDuration::Turns(*n),
        TestBuffDuration::UntilHit { UntilHit: n } => BuffDuration::UntilHit(*n),
        TestBuffDuration::UntilNextPlayer => BuffDuration::UntilNextPlayer,
        TestBuffDuration::Permanent => BuffDuration::Permanent,
    }
}

/// Parse turn phase from string
fn parse_turn_phase(phase: &str) -> Result<TurnPhase, String> {
    match phase {
        "AllocateAttribute" => Ok(TurnPhase::AllocateAttribute),
        "PlayCard" => Ok(TurnPhase::PlayCard),
        "DrawCard" => Ok(TurnPhase::DrawCard),
        _ => Err(format!("Unknown turn phase: {}", phase)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config = TestConfig::load_from_file("testing_config.json");
        assert!(config.is_ok(), "Should load test config successfully");

        let config = config.unwrap();
        assert!(!config.test_scenarios.is_empty(), "Should have test scenarios");
    }

    #[test]
    fn test_create_game_from_scenario() {
        let config = TestConfig::load_from_file("testing_config.json").unwrap();
        let scenario = config.get_scenario("test_fire_spells");
        assert!(scenario.is_some(), "Should find test_fire_spells scenario");

        let game = scenario.unwrap().create_game();
        assert!(game.is_ok(), "Should create game from scenario");

        let game = game.unwrap();
        assert_eq!(game.players.len(), 4, "Should have 4 players");
        assert_eq!(
            game.players[0].name, "FireMage",
            "First player should be FireMage"
        );
        assert_eq!(
            game.players[0].attributes.fire, 5,
            "FireMage should have Fire level 5"
        );
    }
}
