//! Integration tests for Cascade time-loop survival game systems.
//!
//! These tests verify that multiple game systems work together correctly,
//! covering loop mechanics, state persistence, paradox detection,
//! temporal chests, knowledge retention, and difficulty scaling.

use std::collections::HashMap;

use engine_physics::temporal::{LoopPhase, ParadoxResolution, ParadoxType};
use glam::IVec3;

use crate::balance::{BalanceMeter, Falling};
use crate::crafting::{
    NeuralInterface, OrganicLab, ShellForge, ThermalConverter, RECIPE_COAGULANT,
    RECIPE_SHELL_INGOT,
};
use crate::creatures::{HostileCreature, HostileType, PassiveCreature, PassiveType};
use crate::knowledge::discoveries::{DiscoveryID, KnowledgeCategory, KnowledgeSystem};
use crate::networking::{
    deserialize_loop_state, deserialize_persistent_state, deserialize_positions,
    deserialize_titan_state, serialize_loop_state, serialize_persistent_state, serialize_positions,
    serialize_titan_state, ChestStatePacket, LoopSync, PersistentStatePacket, PersistentSync,
    PositionSync, TitanSync,
};
use crate::temporal::loop_manager::LoopManager;
use crate::temporal::paradox::ParadoxHandler;
use crate::temporal::state_persistence::{StateCategory, StatePersistence, StateType};
use crate::temporal_chest::chest::{ItemStack, TemporalChest};
use crate::titan::{
    TitanBehavior, TitanMood, TitanMovement, TitanPhase, TitanZone, ZoneProperties,
    AGITATED_THRESHOLD, ENRAGED_THRESHOLD, MAX_TITAN_HP,
};

// =============================================================================
// Cascade-Specific Integration Tests
// =============================================================================

/// Test 1: Full loop cycle - dawn -> day -> dusk -> midnight -> reset
#[test]
fn test_full_loop_cycle() {
    let mut manager = LoopManager::new();
    assert_eq!(manager.current_loop(), 1);
    assert_eq!(manager.current_phase(), LoopPhase::Dawn);

    // Track phases encountered
    let mut phases_seen = vec![LoopPhase::Dawn];

    // Run through a complete loop (Dawn:30 + Day:480 + Dusk:30 + Midnight:60 = 600s)
    // Use larger time steps to speed up the test
    for _ in 0..70 {
        if let Some(new_phase) = manager.tick(10.0) {
            if !phases_seen.contains(&new_phase) {
                phases_seen.push(new_phase);
            }
        }
    }

    // Should have seen all phases
    assert!(phases_seen.contains(&LoopPhase::Day), "Should see Day phase");
    assert!(
        phases_seen.contains(&LoopPhase::Dusk),
        "Should see Dusk phase"
    );
    assert!(
        phases_seen.contains(&LoopPhase::Midnight),
        "Should see Midnight phase"
    );
}

/// Test 2: State persistence categories
#[test]
fn test_state_persistence_categories() {
    let persistence = StatePersistence::new();

    // Persistent states survive all loops
    assert_eq!(
        persistence.category_of(StateType::TemporalChestContents),
        StateCategory::Persistent
    );
    assert_eq!(
        persistence.category_of(StateType::Messages),
        StateCategory::Persistent
    );
    assert_eq!(
        persistence.category_of(StateType::Knowledge),
        StateCategory::Persistent
    );

    // Semi-persistent states have 50% chance to regenerate
    assert_eq!(
        persistence.category_of(StateType::TerrainModification),
        StateCategory::SemiPersistent
    );
    assert_eq!(
        persistence.category_of(StateType::BuiltStructure),
        StateCategory::SemiPersistent
    );

    // Volatile states reset every loop
    assert_eq!(
        persistence.category_of(StateType::CreaturePositions),
        StateCategory::Volatile
    );
    assert_eq!(
        persistence.category_of(StateType::Weather),
        StateCategory::Volatile
    );
}

/// Test 3: Paradox detection and resolution
#[test]
fn test_paradox_detection() {
    let mut handler = ParadoxHandler::new();

    // Detect a terrain conflict paradox
    let index = handler.detect(IVec3::new(10, 64, 20), ParadoxType::TerrainConflict, 60.0);
    assert!(index.is_some());
    assert_eq!(handler.paradox_count(), 1);

    // Check thresholds
    let paradoxes = handler.paradoxes();
    assert!(paradoxes[0].causes_distortion());
    assert!(!paradoxes[0].is_damaging());

    // Detect a damaging paradox
    handler.detect(IVec3::new(50, 64, 50), ParadoxType::StateOverlap, 120.0);
    let paradoxes = handler.paradoxes();
    assert!(paradoxes[1].is_damaging());
}

/// Test 4: Paradox exploitation for energy
#[test]
fn test_paradox_exploitation() {
    let mut handler = ParadoxHandler::new();

    handler.detect(IVec3::ZERO, ParadoxType::ResourceDuplication, 50.0);

    let energy = handler.resolve(0, ParadoxResolution::Exploit);
    assert!(energy > 0.0, "Should gain energy from exploitation");
    assert!(handler.harvested_energy() > 0.0);
    assert_eq!(handler.paradox_count(), 0);
}

/// Test 5: Temporal chest persistence across loops
#[test]
fn test_chest_persistence_across_loops() {
    let mut chest = TemporalChest::new(IVec3::new(100, 64, 100));

    chest.insert(0, ItemStack::new("time_crystal", 5));
    chest.insert(1, ItemStack::new("loop_key", 1));

    chest.persist_across_loop();

    let ghost = chest.ghost_preview();
    assert!(ghost[0].is_some());
    assert_eq!(ghost[0].as_ref().unwrap().item_type, "time_crystal");
    assert!(ghost[1].is_some());
    assert_eq!(ghost[1].as_ref().unwrap().item_type, "loop_key");

    assert!(!chest.is_empty());
    assert_eq!(chest.used_slots(), 2);
}

/// Test 6: Knowledge retention across loops
#[test]
fn test_knowledge_retention_across_loops() {
    let mut knowledge = KnowledgeSystem::new();

    knowledge.discover(DiscoveryID::new(KnowledgeCategory::Map, 1));
    knowledge.discover(DiscoveryID::new(KnowledgeCategory::Map, 2));
    knowledge.discover(DiscoveryID::new(KnowledgeCategory::Trap, 1));
    knowledge.discover(DiscoveryID::new(KnowledgeCategory::Creature, 1));

    assert_eq!(knowledge.total_discoveries(), 4);
    assert_eq!(knowledge.category_count(KnowledgeCategory::Map), 2);
    assert_eq!(knowledge.category_count(KnowledgeCategory::Trap), 1);
    assert_eq!(knowledge.category_count(KnowledgeCategory::Creature), 1);

    assert!(knowledge.is_discovered(DiscoveryID::new(KnowledgeCategory::Map, 1)));
    assert!(knowledge.is_discovered(DiscoveryID::new(KnowledgeCategory::Trap, 1)));
}

/// Test 7: Difficulty scaling with loop count
#[test]
fn test_difficulty_scaling() {
    let mut manager = LoopManager::new();

    assert!((manager.difficulty() - 1.0).abs() < f32::EPSILON);

    manager.on_death();
    manager.on_death();
    manager.on_death();
    manager.on_death();
    manager.on_death();

    assert!(manager.difficulty() > 1.0);
    assert!((manager.difficulty() - 1.5).abs() < f32::EPSILON);
}

/// Test 8: Loop-aware hostile creature scaling
#[test]
fn test_hostile_creature_loop_scaling() {
    let creature_loop1 = HostileCreature::new(HostileType::TimeWraith);
    let base_hp = creature_loop1.hp();
    let base_damage = creature_loop1.damage();

    let creature_loop5 = HostileCreature::new_with_loop(HostileType::TimeWraith, 5);

    assert!(creature_loop5.hp() > base_hp);
    assert!(creature_loop5.damage() > base_damage);
    assert_eq!(creature_loop5.loop_count(), 5);
}

/// Test 9: Time-based hostile creature abilities
#[test]
fn test_hostile_creature_abilities() {
    let wraith = HostileCreature::new(HostileType::TimeWraith);
    let ability = wraith.use_ability();
    assert!(ability.success);
    assert!(ability.effect.contains("loop counter"));

    let stalker = HostileCreature::new(HostileType::LoopStalker);
    let ability = stalker.use_ability();
    assert!(ability.success);
    assert!(ability.effect_duration > 0);

    let mut beast = HostileCreature::new(HostileType::EchoBeast);
    beast.register_death_at_location();
    beast.register_death_at_location();
    let ability = beast.use_ability();
    assert!(ability.success);
    assert!(ability.damage > beast.damage());

    let spider = HostileCreature::new_with_loop(HostileType::ChronoSpider, 3);
    let ability = spider.use_ability();
    assert!(ability.success);
    assert_eq!(ability.effect_duration, 3);
}

/// Test 10: Passive creature loop-dependent spawning
#[test]
fn test_passive_creature_loop_spawning() {
    assert!(!PassiveType::LoopFish.can_spawn_on_loop(1));
    assert!(PassiveType::LoopFish.can_spawn_on_loop(2));
    assert!(!PassiveType::LoopFish.can_spawn_on_loop(3));
    assert!(PassiveType::LoopFish.can_spawn_on_loop(4));

    assert!(!PassiveType::PhaseDeer.can_spawn_on_loop(1));
    assert!(!PassiveType::PhaseDeer.can_spawn_on_loop(2));
    assert!(PassiveType::PhaseDeer.can_spawn_on_loop(3));
    assert!(PassiveType::PhaseDeer.can_spawn_on_loop(10));

    assert!(PassiveType::MemoryMoth.can_spawn_on_loop(1));
    assert!(PassiveType::MemoryMoth.can_spawn_on_loop(100));
}

/// Test 11: Passive creature temporal drops
#[test]
fn test_passive_creature_temporal_drops() {
    let moth = PassiveCreature::new(PassiveType::MemoryMoth);
    assert_eq!(moth.drop_item(), "temporal_dust");

    let fish = PassiveCreature::new(PassiveType::LoopFish);
    assert_eq!(fish.drop_item(), "time_scale");

    let rabbit = PassiveCreature::new(PassiveType::EchoRabbit);
    assert_eq!(rabbit.drop_item(), "loop_fiber");

    let deer = PassiveCreature::new(PassiveType::PhaseDeer);
    assert_eq!(deer.drop_item(), "phase_antler");

    let turtle = PassiveCreature::new(PassiveType::AnchorTurtle);
    assert_eq!(turtle.drop_item(), "anchor_shell");
}

/// Test 12: Phase Deer phasing mechanic
#[test]
fn test_phase_deer_phasing() {
    let mut deer = PassiveCreature::new(PassiveType::PhaseDeer);
    assert!(!deer.is_phased());

    deer.toggle_phase();
    assert!(deer.is_phased());

    let dealt = deer.take_damage(100);
    assert_eq!(dealt, 0);
    assert_eq!(deer.hp(), deer.max_hp());

    let drop = deer.on_catch();
    assert!(drop.is_none());
    assert!(deer.is_alive());
}

/// Test 13: Loop state network synchronization
#[test]
fn test_loop_state_network_sync() {
    let mut sync = LoopSync::new();

    let data = serialize_loop_state(5, LoopPhase::Dusk, 15.0, 1.4);

    let packet = deserialize_loop_state(&data).unwrap();
    assert_eq!(packet.loop_count, 5);
    assert_eq!(packet.phase(), LoopPhase::Dusk);
    assert!((packet.time_remaining - 15.0).abs() < f32::EPSILON);
    assert!((packet.difficulty - 1.4).abs() < f32::EPSILON);

    sync.update_from_network(&packet);
    assert_eq!(sync.loop_count(), 5);
    assert_eq!(sync.phase(), LoopPhase::Dusk);
}

/// Test 14: Persistent state network synchronization
#[test]
fn test_persistent_state_network_sync() {
    let mut packet = PersistentStatePacket::new();

    let mut chest = ChestStatePacket::new(1);
    chest.add_slot(0, "time_crystal".to_string(), 10);
    packet.add_chest(chest);

    packet.knowledge.add_discovery(KnowledgeCategory::Map, 1);
    packet.message_count = 3;

    let data = serialize_persistent_state(&packet);
    let decoded = deserialize_persistent_state(&data).unwrap();

    assert_eq!(decoded.chests.len(), 1);
    assert_eq!(decoded.chests[0].slots.len(), 1);
    assert_eq!(decoded.knowledge.discoveries.len(), 1);
    assert_eq!(decoded.message_count, 3);
}

/// Test 15: Paradox aging across loops
#[test]
fn test_paradox_aging() {
    let mut handler = ParadoxHandler::new();

    handler.detect(IVec3::new(0, 0, 0), ParadoxType::TerrainConflict, 50.0);
    handler.detect(IVec3::new(100, 0, 0), ParadoxType::StateOverlap, 50.0);

    handler.age_all();

    let paradoxes = handler.paradoxes();
    assert_eq!(paradoxes[0].loop_age, 1);
    assert_eq!(paradoxes[1].loop_age, 1);

    handler.age_all();
    let paradoxes = handler.paradoxes();
    assert_eq!(paradoxes[0].loop_age, 2);
}

/// Test 16: Loop manager death and midnight resets
#[test]
fn test_loop_manager_resets() {
    let mut manager = LoopManager::new();

    manager.tick(50.0);

    let new_loop = manager.on_death();
    assert_eq!(new_loop, 2);
    assert_eq!(manager.current_phase(), LoopPhase::Dawn);

    manager.tick(100.0);

    let new_loop = manager.on_midnight();
    assert_eq!(new_loop, 3);
    assert_eq!(manager.current_phase(), LoopPhase::Dawn);
}

/// Test 17: Temporal Parasite phase ability
#[test]
fn test_temporal_parasite_phase_target() {
    let parasite = HostileCreature::new(HostileType::TemporalParasite);
    let current_pos = IVec3::new(0, 64, 0);

    let chests = vec![
        IVec3::new(100, 64, 0),
        IVec3::new(30, 64, 0),
        IVec3::new(50, 64, 50),
    ];

    let target = parasite.get_phase_target(&chests, current_pos);
    assert_eq!(target, Some(IVec3::new(30, 64, 0)));
}

/// Test 18: Knowledge discovery tracking
#[test]
fn test_knowledge_discovery_tracking() {
    let mut handler = ParadoxHandler::new();

    handler.detect(IVec3::ZERO, ParadoxType::TerrainConflict, 50.0);
    handler.detect(IVec3::new(10, 0, 0), ParadoxType::StateOverlap, 50.0);

    assert_eq!(handler.discovered_count(), 0);

    handler.discover(0);
    assert_eq!(handler.discovered_count(), 1);

    handler.discover(1);
    assert_eq!(handler.discovered_count(), 2);
}

/// Test 19: Complete temporal gameplay scenario
#[test]
fn test_complete_temporal_gameplay_scenario() {
    let mut loop_manager = LoopManager::new();
    let mut paradox_handler = ParadoxHandler::new();
    let mut knowledge = KnowledgeSystem::new();
    let mut chest = TemporalChest::new(IVec3::new(0, 64, 0));

    for tick in 0..100u32 {
        let dt = 1.0;

        loop_manager.tick(dt);

        if tick == 30 {
            paradox_handler.detect(
                IVec3::new(tick as i32, 64, 0),
                ParadoxType::TerrainConflict,
                40.0,
            );
        }

        if tick == 50 {
            knowledge.discover(DiscoveryID::new(KnowledgeCategory::Map, 1));
        }
    }

    chest.insert(0, ItemStack::new("loot", 5));
    chest.persist_across_loop();

    let new_loop = loop_manager.on_death();
    assert_eq!(new_loop, 2);

    assert!(!chest.is_empty());
    assert!(knowledge.is_discovered(DiscoveryID::new(KnowledgeCategory::Map, 1)));
    assert!(paradox_handler.paradox_count() > 0);
}

/// Test 20: Loop break (win condition)
#[test]
fn test_loop_break_win_condition() {
    let mut manager = LoopManager::new();

    manager.on_death();
    manager.on_death();
    manager.on_death();
    assert_eq!(manager.current_loop(), 4);

    manager.break_loop();
    assert_eq!(manager.current_loop(), 0);
}

// =============================================================================
// Legacy Titan/Balance Tests
// =============================================================================

/// Test 21: Full movement cycle
#[test]
fn test_titan_full_movement_cycle() {
    let mut movement = TitanMovement::new();
    assert_eq!(movement.current_phase(), TitanPhase::Resting);

    let mut phases_seen = vec![TitanPhase::Resting];

    for _ in 0..1000 {
        if let Some(new_phase) = movement.tick(1.0) {
            if !phases_seen.contains(&new_phase) {
                phases_seen.push(new_phase);
            }
        }
    }

    assert!(phases_seen.contains(&TitanPhase::Walking));
}

/// Test 22: Balance recovery during phases
#[test]
fn test_balance_recovery_during_phases() {
    let mut balance = BalanceMeter::new();

    balance.modify(-50.0);
    assert!((balance.balance() - 50.0).abs() < f32::EPSILON);

    balance.tick(1.0, TitanPhase::Resting);
    assert!(balance.balance() > 50.0);
}

/// Test 23: Titan mood changes
#[test]
fn test_titan_mood_changes() {
    let mut behavior = TitanBehavior::new();
    assert_eq!(behavior.current_mood(), TitanMood::Calm);

    for _ in 0..4 {
        behavior.harvest_tissue();
    }
    assert_eq!(behavior.current_mood(), TitanMood::Agitated);

    for _ in 0..4 {
        behavior.harvest_tissue();
    }
    assert_eq!(behavior.current_mood(), TitanMood::Enraged);
}

/// Test 24: Crafting stations
#[test]
fn test_crafting_stations() {
    let forge = ShellForge::new();
    assert!(forge.get_recipe(RECIPE_SHELL_INGOT).is_some());

    let lab = OrganicLab::new();
    assert!(lab.get_recipe(RECIPE_COAGULANT).is_some());
}

/// Test 25: Network sync roundtrip
#[test]
fn test_network_serialization_roundtrip() {
    let hp = 7500.5;
    let mood = 2u8;
    let phase = 1u8;
    let day = 42u32;

    let data = serialize_titan_state(hp, mood, phase, day);
    let (hp2, mood2, phase2, day2) = deserialize_titan_state(&data).unwrap();

    assert!((hp - hp2).abs() < f32::EPSILON);
    assert_eq!(mood, mood2);
    assert_eq!(phase, phase2);
    assert_eq!(day, day2);

    let positions = vec![
        (100u64, IVec3::new(i32::MIN, 0, i32::MAX)),
        (u64::MAX, IVec3::new(0, -1000, 1000)),
    ];

    let data = serialize_positions(&positions);
    let positions2 = deserialize_positions(&data);

    assert_eq!(positions.len(), positions2.len());
    assert_eq!(positions[0], positions2[0]);
    assert_eq!(positions[1], positions2[1]);
}
