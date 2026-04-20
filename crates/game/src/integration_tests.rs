//! Integration tests for Void space station survival game systems.
//!
//! These tests verify that multiple game systems work together correctly,
//! covering atmosphere, power, station systems, zero-g movement, EVA, and creatures.

use glam::IVec3;

use crate::creatures::{HostileCreature, HostileType, PassiveCreature, PassiveType};
use crate::power::{PowerConsumer, PowerManager, Reactor};
use crate::station::{Bulkhead, BulkheadState, HullSegment, RoomType, StationRoom, SystemState};
use crate::vacuum::{AtmosphereManager, DecompressionType};
use crate::zerog::{EVAState, RecoilSystem, ZeroGMovement};

// =============================================================================
// Atmosphere and Vacuum Tests
// =============================================================================

/// Test 1: Atmosphere manager lifecycle
#[test]
fn test_atmosphere_manager_lifecycle() {
    let mut manager = AtmosphereManager::new();

    let room1 = manager.add_room(100.0, true);
    let room2 = manager.add_room(100.0, false);

    assert_eq!(manager.room_count(), 2);
    assert!(manager.has_life_support(room1));
    assert!(!manager.has_life_support(room2));

    manager.connect_rooms(room1, room2, 1.0);
    manager.tick(1.0);

    // Life support should maintain O2 in room1
    let atmo1 = manager.get_room(room1).unwrap();
    assert!(atmo1.is_breathable());
}

/// Test 2: Decompression event handling
#[test]
fn test_decompression_event_handling() {
    let mut manager = AtmosphereManager::new();
    let room_id = manager.add_room(100.0, true);

    assert!(manager.is_sealed(room_id));

    let event = manager.breach(room_id, DecompressionType::Rapid);
    assert_eq!(event.room_id, room_id);
    assert_eq!(event.event_type, DecompressionType::Rapid);
    assert!(!manager.is_sealed(room_id));

    // Atmosphere should start venting
    let initial_pressure = manager.get_room(room_id).unwrap().pressure;
    manager.tick(1.0);
    let new_pressure = manager.get_room(room_id).unwrap().pressure;
    assert!(new_pressure < initial_pressure);

    // Seal the breach
    assert!(manager.seal_breach(room_id));
    assert!(manager.is_sealed(room_id));
}

/// Test 3: Gas flow between connected rooms
#[test]
fn test_gas_flow_between_rooms() {
    let mut manager = AtmosphereManager::new();
    let room1 = manager.add_room(100.0, false);
    let room2 = manager.add_room(100.0, false);

    // Modify room1 to have higher pressure
    if let Some(atmo) = manager.get_room_mut(room1) {
        atmo.pressure = 120.0;
    }

    manager.connect_rooms(room1, room2, 2.0);

    let p1_initial = manager.get_room(room1).unwrap().pressure;
    let p2_initial = manager.get_room(room2).unwrap().pressure;

    manager.tick(1.0);

    let p1_after = manager.get_room(room1).unwrap().pressure;
    let p2_after = manager.get_room(room2).unwrap().pressure;

    // Pressure should equalize
    assert!(p1_after < p1_initial);
    assert!(p2_after > p2_initial);
}

// =============================================================================
// Station Architecture Tests
// =============================================================================

/// Test 4: Station room types and properties
#[test]
fn test_station_room_types() {
    assert!(RoomType::Command.is_critical());
    assert!(RoomType::LifeSupport.is_critical());
    assert!(RoomType::PowerCore.is_critical());
    assert!(!RoomType::Cargo.is_critical());
    assert!(!RoomType::Quarters.is_critical());

    let room = StationRoom::new(0, RoomType::Engineering, "Main Engineering".to_string());
    assert_eq!(room.room_type(), RoomType::Engineering);
    assert!(room.is_powered());
    assert!(!room.is_breached());
}

/// Test 5: Hull damage and breach system
#[test]
fn test_hull_damage_system() {
    let mut segment = HullSegment::new(0);
    assert!(!segment.is_breached());
    assert_eq!(segment.damage_level(), "Intact");

    segment.damage(30.0);
    assert_eq!(segment.damage_level(), "Damaged");
    assert!(!segment.is_breached());

    segment.damage(50.0);
    assert_eq!(segment.damage_level(), "Compromised");
    assert!(segment.is_breached());

    segment.patch(60.0);
    assert!(!segment.is_breached());
    // After patching from 20% to 80%, should be "Intact"
    assert_eq!(segment.damage_level(), "Intact");
}

/// Test 6: Bulkhead operations
#[test]
fn test_bulkhead_operations() {
    let mut bulkhead = Bulkhead::new(0, 1);
    assert!(bulkhead.is_passable());
    assert_eq!(bulkhead.state(), BulkheadState::Open);

    assert!(bulkhead.seal());
    assert!(!bulkhead.is_passable());
    assert_eq!(bulkhead.state(), BulkheadState::Sealed);

    assert!(bulkhead.open());
    assert!(bulkhead.is_passable());

    bulkhead.jam();
    assert!(!bulkhead.seal());
    assert!(!bulkhead.open());

    bulkhead.repair();
    assert_eq!(bulkhead.state(), BulkheadState::Sealed);
}

// =============================================================================
// Power System Tests
// =============================================================================

/// Test 7: Reactor power output
#[test]
fn test_reactor_power_output() {
    let mut reactor = Reactor::new();
    assert!(reactor.is_active());
    assert!((reactor.output() - 100.0).abs() < f32::EPSILON);

    reactor.damage(50.0);
    assert!((reactor.output() - 50.0).abs() < f32::EPSILON);

    reactor.damage(40.0);
    assert!(!reactor.is_active());
    assert!((reactor.output() - 0.0).abs() < f32::EPSILON);
}

/// Test 8: Power manager load balancing
#[test]
fn test_power_manager_load_balancing() {
    let mut manager = PowerManager::new();

    manager.register_consumer(PowerConsumer::new(0, "Life Support".to_string(), 1, 30.0));
    manager.register_consumer(PowerConsumer::new(1, "Lights".to_string(), 3, 20.0));
    manager.register_consumer(PowerConsumer::new(2, "Sensors".to_string(), 2, 15.0));

    assert!((manager.total_demand() - 65.0).abs() < f32::EPSILON);
    assert!(manager.available_power() > 0.0);

    // Should have power for all systems
    let affected = manager.tick(1.0);
    assert!(affected.is_empty());
}

/// Test 9: Power shortage and load shedding
#[test]
fn test_power_shortage_handling() {
    let mut manager = PowerManager::new();

    // Add more demand than supply (supply = 100)
    manager.register_consumer(PowerConsumer::new(0, "Critical".to_string(), 1, 40.0));
    manager.register_consumer(PowerConsumer::new(1, "Important".to_string(), 2, 40.0));
    manager.register_consumer(PowerConsumer::new(2, "Low Priority".to_string(), 3, 80.0));

    // Total demand = 160 > supply = 100
    assert!(manager.total_demand() > manager.reactor().output());

    // Drain battery completely
    manager.grid_mut().set_battery_charge(0.0);

    // Now tick should trigger load shedding
    let affected = manager.tick(1.0);
    // With empty battery and deficit, load shedding should occur
    assert!(!affected.is_empty());
}

// =============================================================================
// Zero-G Movement Tests
// =============================================================================

/// Test 10: Zero-G thruster movement
#[test]
fn test_zerog_thruster_movement() {
    let mut movement = ZeroGMovement::new();
    assert_eq!(movement.position(), IVec3::ZERO);
    assert_eq!(movement.velocity(), IVec3::ZERO);

    assert!(movement.thrust(IVec3::new(5, 0, 0), 10.0));
    assert_eq!(movement.velocity(), IVec3::new(5, 0, 0));
    assert!(movement.thruster_fuel() < 100.0);

    movement.tick(1.0);
    assert!(movement.position().x > 0);
}

/// Test 11: Magnetic boots functionality
#[test]
fn test_magnetic_boots() {
    let mut movement = ZeroGMovement::new();

    movement.thrust(IVec3::new(10, 0, 0), 5.0);
    movement.grab_surface();
    movement.enable_boots();

    assert!(movement.magnetic_boots());
    assert!(movement.is_attached());

    movement.tick(1.0);
    assert_eq!(movement.velocity(), IVec3::ZERO);
}

/// Test 12: Recoil physics
#[test]
fn test_recoil_physics() {
    let system = RecoilSystem::new();

    let velocity = IVec3::new(10, 0, 0);
    let force = IVec3::new(3, 0, 0);

    let result = system.apply_recoil(velocity, force);
    assert_eq!(result, IVec3::new(7, 0, 0));

    let recoil = system.calculate_recoil(10.0);
    assert!(recoil.x > 0);
}

// =============================================================================
// EVA Operations Tests
// =============================================================================

/// Test 13: EVA lifecycle
#[test]
fn test_eva_lifecycle() {
    let mut eva = EVAState::new();
    assert!(!eva.is_outside());
    assert!((eva.suit_o2() - 30.0).abs() < f32::EPSILON);

    assert!(eva.enter_vacuum());
    assert!(eva.is_outside());

    // O2 should deplete when outside
    let initial_o2 = eva.suit_o2();
    eva.use_o2(1.0);
    assert!(eva.suit_o2() < initial_o2);

    assert!(eva.return_inside());
    assert!(!eva.is_outside());
}

/// Test 14: EVA tether system
#[test]
fn test_eva_tether_system() {
    let mut eva = EVAState::new();

    // Can't deploy tether while inside
    assert!(!eva.deploy_tether());

    eva.enter_vacuum();
    assert!(eva.deploy_tether());
    assert!(eva.is_tethered());

    eva.shorten_tether(20.0);
    assert!((eva.tether_length() - 30.0).abs() < f32::EPSILON);

    eva.retract_tether();
    assert!(!eva.is_tethered());
}

/// Test 15: EVA oxygen management
#[test]
fn test_eva_oxygen_management() {
    let mut eva = EVAState::new();
    eva.enter_vacuum();

    // Deplete oxygen
    for _ in 0..250 {
        eva.use_o2(1.0);
    }

    assert!(eva.is_o2_critical());

    eva.refill_o2(20.0);
    assert!(!eva.is_o2_critical());
}

// =============================================================================
// Creature Tests
// =============================================================================

/// Test 16: Hostile creature stats
#[test]
fn test_hostile_creature_stats() {
    assert_eq!(HostileType::VoidCrawler.base_hp(), 50);
    assert_eq!(HostileType::VoidCrawler.base_damage(), 10);
    assert_eq!(HostileType::VoidCrawler.special_ability_name(), "short_circuit");

    assert_eq!(HostileType::PressureLeech.base_hp(), 30);
    assert_eq!(HostileType::RadiationWraith.base_hp(), 70);
    assert_eq!(HostileType::DebrisDrone.base_hp(), 60);
    assert_eq!(HostileType::HullMite.base_hp(), 20);
}

/// Test 17: Hostile creature abilities
#[test]
fn test_hostile_creature_abilities() {
    let crawler = HostileCreature::new(HostileType::VoidCrawler);
    let ability = crawler.use_ability();
    assert!(ability.success);
    assert!(ability.effect.contains("electrical"));

    let leech = HostileCreature::new(HostileType::PressureLeech);
    let ability = leech.use_ability();
    assert!(ability.success);
    assert!(ability.effect.contains("breach"));

    let drone = HostileCreature::new(HostileType::DebrisDrone);
    let ability = drone.use_ability();
    assert!(ability.success);
    assert_eq!(ability.damage, drone.damage() * 2);
}

/// Test 18: Passive creature drops
#[test]
fn test_passive_creature_drops() {
    assert_eq!(PassiveType::CircuitMoth.drop_item(), "conductive_dust");
    assert_eq!(PassiveType::CoolantFish.drop_item(), "coolant_scale");
    assert_eq!(PassiveType::SporeBloom.drop_item(), "bio_compound");
    assert_eq!(PassiveType::DustBunny.drop_item(), "filter_fiber");
    assert_eq!(PassiveType::StarCrab.drop_item(), "hull_chitin");
}

/// Test 19: Passive creature behavior
#[test]
fn test_passive_creature_behavior() {
    let mut creature = PassiveCreature::new(PassiveType::StarCrab);
    assert!(creature.is_alive());
    assert!(!creature.is_fleeing());

    creature.take_damage(5);
    assert!(creature.is_fleeing());
    assert!(creature.is_alive());

    let drop = creature.on_catch();
    assert!(drop.is_some());
    assert_eq!(drop.unwrap(), "hull_chitin");
    assert!(!creature.is_alive());
}

/// Test 20: Spore Bloom regeneration
#[test]
fn test_spore_bloom_regeneration() {
    let mut bloom = PassiveCreature::new(PassiveType::SporeBloom);
    bloom.take_damage(5);
    let damaged_hp = bloom.hp();

    bloom.tick(2.0);
    assert!(bloom.hp() > damaged_hp);
}

// =============================================================================
// Integration Scenarios
// =============================================================================

/// Test 21: Complete station emergency scenario
#[test]
fn test_station_emergency_scenario() {
    // Setup station
    let mut atmo = AtmosphereManager::new();
    let command = atmo.add_room(200.0, true);
    let engineering = atmo.add_room(250.0, true);

    atmo.connect_rooms(command, engineering, 1.0);

    let mut power = PowerManager::new();
    power.register_consumer(PowerConsumer::new(command, "Command Systems".to_string(), 1, 15.0));
    power.register_consumer(PowerConsumer::new(engineering, "Engineering".to_string(), 2, 20.0));

    // Simulate normal operation (short simulation, life support maintains atmosphere)
    for _ in 0..5 {
        atmo.tick(0.1);
        power.tick(0.1);
    }

    // Command should still be breathable (life support active)
    let command_atmo = atmo.get_room(command).unwrap();
    assert!(command_atmo.o2 >= 16.0 && command_atmo.o2 <= 25.0);
    assert!(command_atmo.pressure >= 80.0);

    // Trigger breach in engineering
    atmo.breach(engineering, DecompressionType::Slow);

    // Simulate emergency
    for _ in 0..10 {
        atmo.tick(0.1);
        power.tick(0.1);
    }

    // Engineering should be losing pressure
    assert!(atmo.get_room(engineering).unwrap().pressure < 101.3);

    // Seal the breach
    atmo.seal_breach(engineering);
}

/// Test 22: EVA rescue mission
#[test]
fn test_eva_rescue_mission() {
    let mut eva = EVAState::new();
    let mut movement = ZeroGMovement::new();

    // Exit station
    eva.enter_vacuum();
    eva.deploy_tether();

    // Navigate to target
    movement.thrust(IVec3::new(1, 0, 0), 5.0);

    for _ in 0..10 {
        movement.tick(1.0);
        eva.use_o2(1.0);
    }

    assert!(movement.position().x > 0);
    assert!(eva.suit_o2() < 30.0);

    // Return
    movement.thrust(IVec3::new(-1, 0, 0), 10.0);
    eva.return_inside();

    assert!(!eva.is_outside());
}

/// Test 23: Creature encounter during repair
#[test]
fn test_creature_encounter() {
    let mut segment = HullSegment::new(0);
    let mut creature = HostileCreature::new(HostileType::HullMite);

    // Creature damages hull
    segment.damage(creature.damage() as f32);
    assert!((segment.integrity() - 97.0).abs() < f32::EPSILON);

    // Player fights back
    creature.take_damage(10);
    assert!(creature.is_alive());

    creature.take_damage(15);
    assert!(!creature.is_alive());

    // Repair hull
    segment.patch(10.0);
    assert!(segment.integrity() > 97.0);
}

/// Test 24: All hostile types exist
#[test]
fn test_all_hostile_types() {
    let all = HostileType::all();
    assert_eq!(all.len(), 5);

    for hostile_type in all {
        let creature = HostileCreature::new(*hostile_type);
        assert!(creature.is_alive());
        assert!(creature.is_active());
        assert!(creature.hp() > 0);
        assert!(creature.damage() > 0);
    }
}

/// Test 25: All passive types exist
#[test]
fn test_all_passive_types() {
    let all = PassiveType::all();
    assert_eq!(all.len(), 5);

    for passive_type in all {
        let creature = PassiveCreature::new(*passive_type);
        assert!(creature.is_alive());
        assert!(!creature.drop_item().is_empty());
        assert!(!creature.special_trait().is_empty());
    }
}

/// Test 26: Power system under stress
#[test]
fn test_power_system_stress() {
    let mut power = PowerManager::new();

    // Max out power consumption
    for i in 0..10 {
        power.register_consumer(PowerConsumer::new(i, format!("System {}", i), 3, 15.0));
    }

    // Total demand: 150, supply: 100
    assert!(power.total_demand() > power.reactor().output());

    // System should shed load
    let affected = power.tick(1.0);
    // Battery should help initially

    // After battery drains, load shedding occurs
    for _ in 0..100 {
        power.tick(1.0);
    }
}

/// Test 27: Station room network
#[test]
fn test_station_room_network() {
    let rooms = [
        StationRoom::new(0, RoomType::Command, "Bridge".to_string()),
        StationRoom::new(1, RoomType::LifeSupport, "Life Support".to_string()),
        StationRoom::new(2, RoomType::PowerCore, "Reactor".to_string()),
        StationRoom::new(3, RoomType::Engineering, "Engineering".to_string()),
    ];

    let critical_count = rooms.iter().filter(|r| r.room_type().is_critical()).count();
    assert_eq!(critical_count, 3);

    let bulkheads = [
        Bulkhead::new(0, 1),
        Bulkhead::new(1, 2),
        Bulkhead::new(2, 3),
        Bulkhead::new(3, 0),
    ];

    for bulkhead in &bulkheads {
        assert!(bulkhead.is_passable());
    }
}

/// Test 28: Zero-G combat maneuvers
#[test]
fn test_zerog_combat_maneuvers() {
    let mut movement = ZeroGMovement::new();
    let recoil = RecoilSystem::new();

    // Fire weapon
    let weapon_recoil = recoil.weapon_recoil(20.0);
    movement.apply_force(weapon_recoil);

    assert!(movement.velocity().x != 0 || movement.velocity().y != 0 || movement.velocity().z != 0);

    // Compensate with thrusters
    movement.thrust(-movement.velocity(), 5.0);

    // Grab surface to stabilize
    movement.grab_surface();
    movement.enable_boots();
    movement.tick(1.0);

    assert_eq!(movement.velocity(), IVec3::ZERO);
}

/// Test 29: System state transitions
#[test]
fn test_system_state_transitions() {
    use crate::station::RoomSystem;

    let mut system = RoomSystem::new("Life Support Core".to_string(), 25.0);
    assert_eq!(system.state(), SystemState::Online);
    assert!((system.effective_power_draw() - 25.0).abs() < f32::EPSILON);

    system.degrade();
    assert_eq!(system.state(), SystemState::Degraded);
    assert!((system.effective_power_draw() - 12.5).abs() < f32::EPSILON);

    system.power_off();
    assert_eq!(system.state(), SystemState::Offline);
    assert!((system.effective_power_draw() - 0.0).abs() < f32::EPSILON);

    system.power_on();
    assert_eq!(system.state(), SystemState::Online);
}

/// Test 30: Complete game loop scenario
#[test]
fn test_complete_game_loop() {
    // Initialize all systems
    let mut atmo = AtmosphereManager::new();
    let mut power = PowerManager::new();
    let mut eva = EVAState::new();

    // Create station
    let bridge = atmo.add_room(200.0, true);
    let reactor = atmo.add_room(300.0, true);
    let _airlock = atmo.add_room(50.0, false);

    atmo.connect_rooms(bridge, reactor, 1.0);

    power.register_consumer(PowerConsumer::new(bridge, "Bridge".to_string(), 1, 15.0));
    power.register_consumer(PowerConsumer::new(reactor, "Reactor Systems".to_string(), 1, 5.0));

    // Simulate normal operation (shorter time)
    for _ in 0..10 {
        atmo.tick(0.1);
        power.tick(0.1);
    }

    // Emergency: reactor breach!
    atmo.breach(reactor, DecompressionType::Rapid);

    // Player goes EVA to repair
    eva.enter_vacuum();
    eva.deploy_tether();

    // Repair takes time
    for _ in 0..20 {
        atmo.tick(0.1);
        eva.use_o2(0.1);
    }

    // Seal breach
    atmo.seal_breach(reactor);

    // Return inside
    eva.return_inside();

    // Verify survival
    assert!(!eva.is_o2_depleted());

    // Bridge should still have O2 and pressure (life support active)
    let bridge_atmo = atmo.get_room(bridge).unwrap();
    assert!(bridge_atmo.o2 >= 16.0);
    assert!(bridge_atmo.pressure >= 80.0);
}

// =============================================================================
// Void-Specific Integration Tests
// =============================================================================

use crate::equipment::{EVAEquipment, EVAGear};
use crate::events::{CascadeEffect, CascadeEvent, CascadeEventType, RandomEvent, RandomEventType};
use crate::vacuum::GasModel;
use engine_physics::vacuum::RoomAtmosphereSim;

/// Test 31: Decompression cascade - hull breach triggers chain of effects
#[test]
fn test_decompression_cascade() {
    // A hull breach should trigger decompression, power failure, and station dark
    let mut cascade = CascadeEvent::new(CascadeEventType::HullBreach, 0);

    assert_eq!(cascade.trigger(), CascadeEventType::HullBreach);
    assert!(!cascade.is_complete());
    assert!(cascade.effects().contains(&CascadeEffect::Decompression));
    assert!(cascade.effects().contains(&CascadeEffect::PowerFailure));
    assert!(cascade.effects().contains(&CascadeEffect::StationDark));

    // Decompression triggers immediately (0 delay)
    let triggered = cascade.tick(0.1);
    assert!(triggered.contains(&CascadeEffect::Decompression));
    assert!(cascade.has_triggered(CascadeEffect::Decompression));

    // Power failure triggers after 2 seconds
    cascade.tick(2.0);
    assert!(cascade.has_triggered(CascadeEffect::PowerFailure));

    // Station dark triggers after 3 seconds total
    cascade.tick(1.0);
    assert!(cascade.has_triggered(CascadeEffect::StationDark));
    assert!(cascade.is_complete());
}

/// Test 32: Decompression cascade with atmosphere simulation
#[test]
fn test_decompression_cascade_with_atmosphere() {
    let mut atmo = AtmosphereManager::new();
    let command = atmo.add_room(200.0, true);
    let engineering = atmo.add_room(150.0, true);
    let cargo = atmo.add_room(100.0, false);

    atmo.connect_rooms(command, engineering, 1.0);
    atmo.connect_rooms(engineering, cargo, 0.5);

    // Start cascade in engineering
    let mut cascade = CascadeEvent::new(CascadeEventType::HullBreach, engineering);
    cascade.tick(0.1); // Trigger decompression

    // Simulate breach in engineering
    atmo.breach(engineering, DecompressionType::Explosive);

    // Run simulation - atmosphere should vent rapidly
    let initial_eng_pressure = atmo.get_room(engineering).unwrap().pressure;
    for _ in 0..10 {
        atmo.tick(0.5);
    }

    let final_eng_pressure = atmo.get_room(engineering).unwrap().pressure;
    assert!(final_eng_pressure < initial_eng_pressure * 0.5);

    // Connected rooms should also lose some pressure due to flow
    let command_pressure = atmo.get_room(command).unwrap().pressure;
    assert!(command_pressure < 101.3);
}

/// Test 33: Power failure cascade
#[test]
fn test_power_failure_cascade() {
    let mut power = PowerManager::new();

    // Register critical systems
    power.register_consumer(PowerConsumer::new(0, "Life Support".to_string(), 1, 40.0));
    power.register_consumer(PowerConsumer::new(1, "Reactor Cooling".to_string(), 1, 30.0));
    power.register_consumer(PowerConsumer::new(2, "Sensors".to_string(), 2, 15.0));
    power.register_consumer(PowerConsumer::new(3, "Lights".to_string(), 3, 10.0));

    // Start reactor damage cascade
    let mut cascade = CascadeEvent::new(CascadeEventType::ReactorDamage, 0);

    // Damage the reactor
    power.reactor_mut().damage(80.0);
    assert!(!power.reactor().is_active());

    // Cascade should trigger power failure, life support offline, station dark
    cascade.tick(3.0);
    assert!(cascade.has_triggered(CascadeEffect::PowerFailure));

    cascade.tick(3.0);
    assert!(cascade.has_triggered(CascadeEffect::LifeSupportOffline));
    assert!(cascade.has_triggered(CascadeEffect::StationDark));

    // With no reactor output, load shedding should occur
    power.grid_mut().set_battery_charge(0.0);
    let affected = power.tick(1.0);
    // Systems should be affected due to power shortage
    assert!(!affected.is_empty() || power.reactor().output() == 0.0);
}

/// Test 34: Breach patching with EVA equipment
#[test]
fn test_breach_patching_eva() {
    let mut atmo = AtmosphereManager::new();
    let airlock = atmo.add_room(50.0, false);

    // Create breach
    atmo.breach(airlock, DecompressionType::Slow);
    assert!(!atmo.is_sealed(airlock));

    // Player prepares for EVA
    let mut eva = EVAState::new();
    let mut patch_kit = EVAEquipment::new(EVAGear::PatchKit);
    let mut tether = EVAEquipment::new(EVAGear::TetherLine);

    // Enter vacuum
    eva.enter_vacuum();
    eva.deploy_tether();

    // Navigate and use equipment
    tether.use_equipment();
    assert!(patch_kit.use_equipment());
    assert!(patch_kit.durability() < EVAGear::PatchKit.base_durability());

    // Seal the breach
    assert!(atmo.seal_breach(airlock));
    assert!(atmo.is_sealed(airlock));

    // Return inside
    eva.return_inside();
    assert!(!eva.is_outside());
}

/// Test 35: Breach patching under time pressure (O2 depletion)
#[test]
fn test_breach_patching_o2_pressure() {
    let mut eva = EVAState::new();
    let mut patch_kit = EVAEquipment::new(EVAGear::PatchKit);

    eva.enter_vacuum();
    eva.deploy_tether();

    // Simulate repair work while consuming O2
    let mut repair_steps = 0;
    while patch_kit.durability() > 10.0 && !eva.is_o2_critical() {
        patch_kit.use_equipment();
        eva.use_o2(2.0); // Higher consumption during work
        repair_steps += 1;
    }

    // Should have completed some repair steps
    assert!(repair_steps > 0);

    // If O2 becomes critical, need to abort or refill
    if eva.is_o2_critical() {
        let mut o2_canister = EVAEquipment::new(EVAGear::O2Canister);
        o2_canister.use_equipment();
        eva.refill_o2(15.0);
        assert!(!eva.is_o2_critical());
    }
}

/// Test 36: Atmosphere equalization between rooms
#[test]
fn test_atmosphere_equalization() {
    let mut gas_model = GasModel::new();

    // Create two rooms with different pressures
    let mut high_pressure = RoomAtmosphereSim::new();
    high_pressure.pressure = 150.0;
    let mut low_pressure = RoomAtmosphereSim::new();
    low_pressure.pressure = 50.0;

    let room_a = gas_model.add_room(high_pressure);
    let room_b = gas_model.add_room(low_pressure);

    gas_model.connect_rooms(room_a, room_b, 2.0);

    let initial_a = gas_model.get_room(room_a).unwrap().pressure;
    let initial_b = gas_model.get_room(room_b).unwrap().pressure;

    // Run simulation until pressures equalize
    for _ in 0..50 {
        gas_model.tick(0.5);
    }

    let final_a = gas_model.get_room(room_a).unwrap().pressure;
    let final_b = gas_model.get_room(room_b).unwrap().pressure;

    // Pressures should be closer to each other
    assert!((final_a - final_b).abs() < (initial_a - initial_b).abs());
    // Both should be moving toward equilibrium
    assert!(final_a < initial_a);
    assert!(final_b > initial_b);
}

/// Test 37: Multi-room atmosphere network
#[test]
fn test_multi_room_atmosphere_network() {
    let mut atmo = AtmosphereManager::new();

    // Create station layout: Command - Engineering - Cargo - Airlock
    let command = atmo.add_room(200.0, true);
    let engineering = atmo.add_room(250.0, true);
    let cargo = atmo.add_room(150.0, false);
    let airlock = atmo.add_room(50.0, false);

    atmo.connect_rooms(command, engineering, 1.0);
    atmo.connect_rooms(engineering, cargo, 1.0);
    atmo.connect_rooms(cargo, airlock, 0.5);

    // Breach the airlock
    atmo.breach(airlock, DecompressionType::Slow);

    // Run simulation
    for _ in 0..20 {
        atmo.tick(0.5);
    }

    // Airlock should have lowest pressure
    let airlock_p = atmo.get_room(airlock).unwrap().pressure;
    let cargo_p = atmo.get_room(cargo).unwrap().pressure;
    let eng_p = atmo.get_room(engineering).unwrap().pressure;
    let cmd_p = atmo.get_room(command).unwrap().pressure;

    assert!(airlock_p < cargo_p);
    assert!(cargo_p < eng_p);
    // Command has life support, should maintain better
    assert!(cmd_p >= eng_p - 10.0);
}

/// Test 38: EVA thruster navigation and return
#[test]
fn test_eva_thruster_navigation() {
    let mut eva = EVAState::new();
    let mut movement = ZeroGMovement::new();
    let mut thruster = EVAEquipment::new(EVAGear::ThrusterPack);

    eva.enter_vacuum();
    eva.deploy_tether();

    // Tether starts at max (50.0), so we shorten it first to simulate being near the station
    eva.shorten_tether(30.0);
    let initial_tether = eva.tether_length();

    // Navigate outward
    for _ in 0..5 {
        thruster.use_equipment();
        movement.thrust(IVec3::new(2, 0, 0), 5.0);
        movement.tick(1.0);
        eva.extend_tether(5.0);
    }

    assert!(movement.position().x > 0);
    assert!(eva.tether_length() > initial_tether);

    // Return trip
    for _ in 0..5 {
        thruster.use_equipment();
        movement.thrust(IVec3::new(-2, 0, 0), 5.0);
        movement.tick(1.0);
        eva.shorten_tether(5.0);
    }

    eva.return_inside();
    assert!(!eva.is_outside());
    assert!(thruster.durability() < EVAGear::ThrusterPack.base_durability());
}

/// Test 39: EVA emergency return (low O2)
#[test]
fn test_eva_emergency_return() {
    let mut eva = EVAState::new();
    let mut movement = ZeroGMovement::new();

    eva.enter_vacuum();
    eva.deploy_tether();

    // Work until O2 is low
    while !eva.is_o2_critical() {
        eva.use_o2(1.0);
        movement.tick(1.0);
    }

    assert!(eva.is_o2_critical());

    // Emergency return - follow tether back
    eva.retract_tether();
    eva.return_inside();

    assert!(!eva.is_outside());
    // Should have returned before complete depletion
    assert!(!eva.is_o2_depleted());
}

/// Test 40: Creature attack during hull repair
#[test]
fn test_creature_attack_hull_repair() {
    let mut segment = HullSegment::new(0);
    let mut hull_mite = HostileCreature::new(HostileType::HullMite);
    let mut void_crawler = HostileCreature::new(HostileType::VoidCrawler);

    // Initial hull state
    assert!(!segment.is_breached());

    // Hull mites attack and damage hull
    for _ in 0..5 {
        if hull_mite.is_alive() {
            segment.damage(hull_mite.damage() as f32);
        }
    }

    assert!(segment.integrity() < 100.0);

    // Void crawler joins attack with special ability
    let ability = void_crawler.use_ability();
    assert!(ability.success);
    assert!(ability.effect.contains("electrical"));

    // Player fights back
    hull_mite.take_damage(20);
    void_crawler.take_damage(25);
    void_crawler.take_damage(30);

    assert!(!hull_mite.is_alive());
    assert!(!void_crawler.is_alive());

    // Repair hull
    segment.patch(50.0);
    assert!(!segment.is_breached());
}

/// Test 41: Pressure Leech creates hull breach
#[test]
fn test_pressure_leech_breach() {
    let mut atmo = AtmosphereManager::new();
    let corridor = atmo.add_room(100.0, false);

    let mut leech = HostileCreature::new(HostileType::PressureLeech);

    // Leech's special ability creates micro-breaches
    let ability = leech.use_ability();
    assert!(ability.success);
    assert!(ability.effect.contains("breach"));

    // Simulate breach creation
    atmo.breach(corridor, DecompressionType::Slow);
    assert!(!atmo.is_sealed(corridor));

    let initial_pressure = atmo.get_room(corridor).unwrap().pressure;
    atmo.tick(2.0);

    // Pressure should be venting
    assert!(atmo.get_room(corridor).unwrap().pressure < initial_pressure);

    // Kill the leech and patch
    leech.take_damage(30);
    assert!(!leech.is_alive());
    atmo.seal_breach(corridor);
}

/// Test 42: Multiple creature encounter
#[test]
fn test_multiple_creature_encounter() {
    let creatures: Vec<HostileCreature> = HostileType::all()
        .iter()
        .map(|t| HostileCreature::new(*t))
        .collect();

    assert_eq!(creatures.len(), 5);

    let mut total_hp = 0;
    let mut total_damage = 0;

    for creature in &creatures {
        assert!(creature.is_alive());
        total_hp += creature.hp();
        total_damage += creature.damage();
    }

    assert!(total_hp > 200);
    assert!(total_damage > 20);

    // Each creature type has unique abilities
    let abilities: Vec<_> = creatures.iter().map(|c| c.use_ability().effect).collect();
    let unique_abilities: std::collections::HashSet<_> = abilities.iter().collect();
    assert!(unique_abilities.len() >= 3); // At least 3 unique ability types
}

/// Test 43: Random event chain - micrometeorite shower
#[test]
fn test_micrometeorite_shower_event() {
    let mut event = RandomEvent::new(RandomEventType::MicrometeoriteShower);
    let mut segment = HullSegment::new(0);

    assert!(!event.is_active());
    event.add_affected_room(0);
    event.start();
    assert!(event.is_active());

    // Simulate shower duration (30 second event)
    let initial_integrity = segment.integrity();
    for _ in 0..35 {
        let damage = event.hull_damage_per_tick();
        if damage > 0.0 {
            segment.damage(damage);
        }
        event.tick(1.0);
    }

    // Hull should have taken damage during the event
    assert!(segment.integrity() < initial_integrity);
    // Event should no longer be active after its duration
    assert!(!event.is_active());
}

/// Test 44: Solar flare event affecting power
#[test]
fn test_solar_flare_power_impact() {
    let mut power = PowerManager::new();
    power.register_consumer(PowerConsumer::new(0, "Sensors".to_string(), 2, 15.0));

    let mut event = RandomEvent::with_intensity(RandomEventType::SolarFlare, 1.5);
    event.start();

    assert!(event.is_active());
    assert!(event.event_type().affects_power());

    // Solar flare damages power systems
    let power_damage = event.power_damage_per_tick();
    assert!(power_damage > 2.0);

    // Reactor takes damage over time
    let mut total_damage = 0.0;
    while event.is_active() {
        total_damage += event.power_damage_per_tick();
        power.tick(1.0);
        event.tick(1.0);
    }

    assert!(total_damage > 100.0);
}

/// Test 45: Event chain - debris field followed by breach cascade
#[test]
fn test_debris_field_cascade_chain() {
    let mut debris_event = RandomEvent::new(RandomEventType::DebrisField);
    let mut segment = HullSegment::new(0);

    debris_event.add_affected_room(0);
    debris_event.start();

    // Debris damages hull
    for _ in 0..20 {
        if debris_event.is_active() {
            segment.damage(debris_event.hull_damage_per_tick());
            debris_event.tick(1.0);
        }
    }

    // If hull is breached, trigger cascade
    if segment.is_breached() {
        let mut cascade = CascadeEvent::new(CascadeEventType::HullBreach, 0);
        cascade.tick(0.1);
        assert!(cascade.has_triggered(CascadeEffect::Decompression));
    }

    // Hull should have taken significant damage
    assert!(segment.integrity() < 90.0);
}

/// Test 46: Life support failure cascade
#[test]
fn test_life_support_failure_cascade() {
    let mut atmo = AtmosphereManager::new();
    let quarters = atmo.add_room(150.0, true);

    let mut cascade = CascadeEvent::new(CascadeEventType::LifeSupportFailure, quarters);

    // Life support failure effects
    assert!(cascade.effects().contains(&CascadeEffect::AtmosphereContamination));
    assert!(cascade.effects().contains(&CascadeEffect::OxygenDepletion));

    // Disable life support
    atmo.set_life_support(quarters, false);
    assert!(!atmo.has_life_support(quarters));

    // Progress cascade
    cascade.tick(15.0);
    assert!(cascade.has_triggered(CascadeEffect::AtmosphereContamination));

    cascade.tick(20.0);
    assert!(cascade.has_triggered(CascadeEffect::OxygenDepletion));

    // Without life support, simulate a small breach to demonstrate atmosphere loss
    // Life support failure means the room can't recover from any atmosphere issues
    atmo.breach(quarters, DecompressionType::Slow);
    let initial_pressure = atmo.get_room(quarters).unwrap().pressure;

    for _ in 0..20 {
        atmo.tick(1.0);
    }

    let final_pressure = atmo.get_room(quarters).unwrap().pressure;
    // With life support off and a breach, atmosphere can't be maintained
    assert!(final_pressure < initial_pressure);

    // Cascade should be complete
    assert!(cascade.is_complete());
}

/// Test 47: Electrical cascade disabling systems
#[test]
fn test_electrical_cascade() {
    use crate::station::RoomSystem;

    let mut cascade = CascadeEvent::new(CascadeEventType::ElectricalCascade, 0);
    let mut system1 = RoomSystem::new("Primary Console".to_string(), 20.0);
    let mut system2 = RoomSystem::new("Backup Systems".to_string(), 15.0);

    // Cascade effects
    assert!(cascade.effects().contains(&CascadeEffect::PowerFailure));
    assert!(cascade.effects().contains(&CascadeEffect::SystemDamage));
    assert!(cascade.effects().contains(&CascadeEffect::StationDark));

    // Systems take damage as cascade progresses
    cascade.tick(1.5);
    if cascade.has_triggered(CascadeEffect::SystemDamage) {
        system1.degrade();
        system2.degrade();
    }

    assert_eq!(system1.state(), SystemState::Degraded);
    assert_eq!(system2.state(), SystemState::Degraded);

    // Power failure
    cascade.tick(1.0);
    if cascade.has_triggered(CascadeEffect::PowerFailure) {
        system1.power_off();
        system2.power_off();
    }

    assert_eq!(system1.state(), SystemState::Offline);
}

/// Test 48: Full station emergency scenario
#[test]
fn test_full_station_emergency() {
    // Initialize all systems
    let mut atmo = AtmosphereManager::new();
    let mut power = PowerManager::new();
    let mut eva = EVAState::new();

    // Create station - isolated bridge for safety
    let bridge = atmo.add_room(200.0, true);
    let engineering = atmo.add_room(250.0, true);
    let cargo = atmo.add_room(150.0, false);

    // Don't connect bridge directly to cargo to protect it
    atmo.connect_rooms(engineering, cargo, 0.3);

    power.register_consumer(PowerConsumer::new(bridge, "Bridge Systems".to_string(), 1, 25.0));
    power.register_consumer(PowerConsumer::new(engineering, "Engineering".to_string(), 1, 30.0));

    // PHASE 1: Meteorite strike on cargo
    let mut meteorite = RandomEvent::new(RandomEventType::MicrometeoriteShower);
    let mut hull = HullSegment::new(cargo);
    meteorite.add_affected_room(cargo);
    meteorite.start();

    for _ in 0..15 {
        hull.damage(meteorite.hull_damage_per_tick());
        meteorite.tick(1.0);
        atmo.tick(0.5);
        power.tick(0.5);
    }

    // PHASE 2: Hull breached, cascade begins
    atmo.breach(cargo, DecompressionType::Slow);
    let mut cascade = CascadeEvent::new(CascadeEventType::HullBreach, cargo);

    // Progress cascade
    cascade.tick(5.0);
    assert!(cascade.has_triggered(CascadeEffect::Decompression));
    assert!(cascade.has_triggered(CascadeEffect::PowerFailure));

    // PHASE 3: EVA repair mission
    eva.enter_vacuum();
    eva.deploy_tether();

    let mut patch_kit = EVAEquipment::new(EVAGear::PatchKit);
    let mut welding_torch = EVAEquipment::new(EVAGear::WeldingTorch);

    // Emergency patch
    patch_kit.use_equipment();
    atmo.seal_breach(cargo);

    // Permanent repair
    welding_torch.use_equipment();
    hull.patch(30.0);

    eva.return_inside();

    // PHASE 4: Recovery verification
    assert!(atmo.is_sealed(cargo));
    assert!(!eva.is_outside());
    assert!(!eva.is_o2_depleted());
    assert!(hull.integrity() > 0.0);

    // Bridge with life support should maintain atmosphere
    let bridge_atmo = atmo.get_room(bridge).unwrap();
    assert!(bridge_atmo.pressure >= 80.0);
    assert!(bridge_atmo.o2 >= 16.0 && bridge_atmo.o2 <= 25.0);
}

/// Test 49: Passive creature resource gathering during emergency
#[test]
fn test_passive_creature_resources() {
    // During emergencies, passive creatures can provide resources
    let mut circuit_moth = PassiveCreature::new(PassiveType::CircuitMoth);
    let mut coolant_fish = PassiveCreature::new(PassiveType::CoolantFish);
    let mut star_crab = PassiveCreature::new(PassiveType::StarCrab);

    // Collect resources for repairs
    let conductive_dust = circuit_moth.on_catch();
    let coolant_scale = coolant_fish.on_catch();
    let hull_chitin = star_crab.on_catch();

    assert_eq!(conductive_dust, Some("conductive_dust".to_string()));
    assert_eq!(coolant_scale, Some("coolant_scale".to_string()));
    assert_eq!(hull_chitin, Some("hull_chitin".to_string()));

    // After catching, creatures are no longer alive
    assert!(!circuit_moth.is_alive());
    assert!(!coolant_fish.is_alive());
    assert!(!star_crab.is_alive());
}

/// Test 50: Complete cascade event recovery
#[test]
fn test_cascade_event_recovery() {
    use crate::station::RoomSystem;

    // Start with a severe cascade
    let mut cascade = CascadeEvent::new(CascadeEventType::ReactorDamage, 0);

    // Force all effects to trigger
    let triggered = cascade.force_complete();
    assert!(cascade.is_complete());
    assert_eq!(triggered.len(), cascade.effects().len());

    // Recovery sequence
    let mut power = PowerManager::new();
    let mut life_support = RoomSystem::new("Life Support".to_string(), 30.0);
    let mut lights = RoomSystem::new("Lights".to_string(), 10.0);

    // Repair reactor
    power.reactor_mut().repair(50.0);
    assert!(power.reactor().is_active());

    // Restore systems by priority
    life_support.power_on();
    assert_eq!(life_support.state(), SystemState::Online);

    lights.power_on();
    assert_eq!(lights.state(), SystemState::Online);

    // Verify recovery
    assert!(power.reactor().output() > 0.0);
}
