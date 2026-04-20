//! Integration tests for the Void space station survival game.
//!
//! Tests core functionality across multiple systems.

use void_game::creatures::{HostileCreature, HostileType, PassiveCreature, PassiveType};
use void_game::power::{PowerConsumer, PowerManager, Reactor};
use void_game::station::{Bulkhead, HullSegment, RoomType, StationRoom};
use void_game::vacuum::{AtmosphereManager, DecompressionType};
use void_game::zerog::{EVAState, ZeroGMovement};

/// Test atmosphere systems.
mod atmosphere_tests {
    use super::*;

    #[test]
    fn test_atmosphere_normal_operation() {
        let mut manager = AtmosphereManager::new();
        let room = manager.add_room(100.0, true);

        // Room should start breathable
        assert!(manager.get_room(room).unwrap().is_breathable());

        // Life support should maintain atmosphere (short simulation)
        for _ in 0..5 {
            manager.tick(0.1);
        }

        // O2 and pressure should be maintained
        let atmo = manager.get_room(room).unwrap();
        assert!(atmo.o2 >= 16.0);
        assert!(atmo.pressure >= 80.0);
    }

    #[test]
    fn test_atmosphere_breach_sequence() {
        let mut manager = AtmosphereManager::new();
        let room = manager.add_room(100.0, true);

        // Breach the room
        manager.breach(room, DecompressionType::Rapid);
        assert!(!manager.is_sealed(room));

        let initial_pressure = manager.get_room(room).unwrap().pressure;

        // Pressure should drop
        for _ in 0..20 {
            manager.tick(1.0);
        }

        assert!(manager.get_room(room).unwrap().pressure < initial_pressure);
    }

    #[test]
    fn test_atmosphere_repair() {
        let mut manager = AtmosphereManager::new();
        let room = manager.add_room(100.0, true);

        manager.breach(room, DecompressionType::Slow);

        // Seal the breach
        assert!(manager.seal_breach(room));
        assert!(manager.is_sealed(room));
    }
}

/// Test power systems.
mod power_tests {
    use super::*;

    #[test]
    fn test_reactor_basic() {
        let reactor = Reactor::new();
        assert!(reactor.is_active());
        assert!((reactor.output() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_reactor_damage_affects_output() {
        let mut reactor = Reactor::new();
        reactor.damage(50.0);
        assert!((reactor.output() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_power_manager_consumers() {
        let mut manager = PowerManager::new();

        manager.register_consumer(PowerConsumer::new(0, "System A".to_string(), 1, 20.0));
        manager.register_consumer(PowerConsumer::new(1, "System B".to_string(), 2, 30.0));

        assert_eq!(manager.consumer_count(), 2);
        assert!((manager.total_demand() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_power_manager_balanced() {
        let mut manager = PowerManager::new();
        manager.register_consumer(PowerConsumer::new(0, "Light".to_string(), 3, 20.0));

        let affected = manager.tick(1.0);
        assert!(affected.is_empty());
    }
}

/// Test station architecture.
mod station_tests {
    use super::*;

    #[test]
    fn test_room_types() {
        assert!(RoomType::Command.is_critical());
        assert!(RoomType::LifeSupport.is_critical());
        assert!(RoomType::PowerCore.is_critical());
        assert!(!RoomType::Cargo.is_critical());
    }

    #[test]
    fn test_station_room() {
        let room = StationRoom::new(0, RoomType::Engineering, "Engineering Bay".to_string());
        assert_eq!(room.name(), "Engineering Bay");
        assert!(room.is_powered());
        assert!(!room.is_breached());
    }

    #[test]
    fn test_hull_segment() {
        let mut segment = HullSegment::new(0);
        assert!(!segment.is_breached());

        segment.damage(80.0);
        assert!(segment.is_breached());

        segment.patch(60.0);
        assert!(!segment.is_breached());
    }

    #[test]
    fn test_bulkhead_states() {
        let mut bulkhead = Bulkhead::new(0, 1);
        assert!(bulkhead.is_passable());

        bulkhead.seal();
        assert!(!bulkhead.is_passable());

        bulkhead.open();
        assert!(bulkhead.is_passable());
    }
}

/// Test zero-g movement.
mod zerog_tests {
    use super::*;
    use glam::IVec3;

    #[test]
    fn test_movement_thrust() {
        let mut movement = ZeroGMovement::new();
        assert!(movement.thrust(IVec3::new(5, 0, 0), 10.0));
        assert_eq!(movement.velocity(), IVec3::new(5, 0, 0));
    }

    #[test]
    fn test_movement_fuel_consumption() {
        let mut movement = ZeroGMovement::new();
        let initial = movement.thruster_fuel();

        movement.thrust(IVec3::new(1, 0, 0), 10.0);

        assert!(movement.thruster_fuel() < initial);
    }

    #[test]
    fn test_magnetic_boots() {
        let mut movement = ZeroGMovement::new();
        movement.enable_boots();
        assert!(movement.magnetic_boots());

        movement.disable_boots();
        assert!(!movement.magnetic_boots());
    }
}

/// Test EVA operations.
mod eva_tests {
    use super::*;

    #[test]
    fn test_eva_enter_exit() {
        let mut eva = EVAState::new();
        assert!(!eva.is_outside());

        assert!(eva.enter_vacuum());
        assert!(eva.is_outside());

        assert!(eva.return_inside());
        assert!(!eva.is_outside());
    }

    #[test]
    fn test_eva_oxygen_consumption() {
        let mut eva = EVAState::new();
        let initial_o2 = eva.suit_o2();

        eva.enter_vacuum();
        eva.use_o2(1.0);

        assert!(eva.suit_o2() < initial_o2);
    }

    #[test]
    fn test_eva_tether() {
        let mut eva = EVAState::new();

        // Can't deploy inside
        assert!(!eva.deploy_tether());

        eva.enter_vacuum();
        assert!(eva.deploy_tether());
        assert!(eva.is_tethered());

        eva.retract_tether();
        assert!(!eva.is_tethered());
    }
}

/// Test creatures.
mod creature_tests {
    use super::*;

    #[test]
    fn test_hostile_creature_types() {
        for hostile_type in HostileType::all() {
            let creature = HostileCreature::new(*hostile_type);
            assert!(creature.hp() > 0);
            assert!(creature.damage() > 0);
            assert!(creature.is_alive());
        }
    }

    #[test]
    fn test_hostile_creature_combat() {
        let mut creature = HostileCreature::new(HostileType::VoidCrawler);
        let initial_hp = creature.hp();

        creature.take_damage(20);

        assert!(creature.hp() < initial_hp);
        assert!(creature.is_alive());
    }

    #[test]
    fn test_hostile_creature_death() {
        let mut creature = HostileCreature::new(HostileType::HullMite);
        creature.take_damage(100);

        assert!(!creature.is_alive());
        assert_eq!(creature.attack(), 0);
    }

    #[test]
    fn test_passive_creature_types() {
        for passive_type in PassiveType::all() {
            let creature = PassiveCreature::new(*passive_type);
            assert!(creature.hp() > 0);
            assert!(!creature.drop_item().is_empty());
        }
    }

    #[test]
    fn test_passive_creature_catch() {
        let mut creature = PassiveCreature::new(PassiveType::CircuitMoth);
        let drop = creature.on_catch();

        assert!(drop.is_some());
        assert_eq!(drop.unwrap(), "conductive_dust");
        assert!(!creature.is_alive());
    }

    #[test]
    fn test_passive_creature_flee() {
        let mut creature = PassiveCreature::new(PassiveType::DustBunny);
        assert!(!creature.is_fleeing());

        creature.take_damage(2);
        assert!(creature.is_fleeing());
    }
}

/// Test full scenarios.
mod scenario_tests {
    use super::*;
    use glam::IVec3;

    #[test]
    fn test_emergency_decompression_scenario() {
        let mut atmo = AtmosphereManager::new();
        let bridge = atmo.add_room(200.0, true);
        let engineering = atmo.add_room(250.0, true);

        atmo.connect_rooms(bridge, engineering, 1.0);

        // Normal operation
        for _ in 0..10 {
            atmo.tick(1.0);
        }

        // Emergency in engineering
        atmo.breach(engineering, DecompressionType::Rapid);

        for _ in 0..10 {
            atmo.tick(1.0);
        }

        // Engineering should be losing pressure
        assert!(atmo.get_room(engineering).unwrap().pressure < 101.3);

        // Seal the breach
        atmo.seal_breach(engineering);
        assert!(atmo.is_sealed(engineering));
    }

    #[test]
    fn test_eva_repair_mission() {
        let mut eva = EVAState::new();
        let mut movement = ZeroGMovement::new();

        eva.enter_vacuum();
        eva.deploy_tether();

        // Navigate to repair site
        movement.thrust(IVec3::new(1, 0, 0), 5.0);

        for _ in 0..5 {
            movement.tick(1.0);
            eva.use_o2(1.0);
        }

        // Perform repair (hull segment)
        let mut segment = HullSegment::new(0);
        segment.damage(50.0);
        segment.patch(30.0);

        // Return
        eva.return_inside();

        assert!(!eva.is_outside());
        assert!(!eva.is_o2_depleted());
    }

    #[test]
    fn test_power_failure_cascade() {
        let mut power = PowerManager::new();

        // Register many consumers (total = 120, supply = 100)
        power.register_consumer(PowerConsumer::new(0, "Life Support".to_string(), 1, 30.0));
        power.register_consumer(PowerConsumer::new(1, "Sensors".to_string(), 2, 25.0));
        power.register_consumer(PowerConsumer::new(2, "Lights".to_string(), 3, 20.0));
        power.register_consumer(PowerConsumer::new(3, "Entertainment".to_string(), 3, 45.0));

        // Total demand exceeds supply (100)
        assert!(power.total_demand() > 100.0);

        // Battery should help initially
        power.tick(1.0);

        // After battery depletes, load shedding occurs
        for _ in 0..100 {
            power.tick(1.0);
        }
    }

    #[test]
    fn test_creature_combat_sequence() {
        let mut hostile = HostileCreature::new(HostileType::DebrisDrone);
        let mut player_hp = 100;

        // Combat rounds
        for _ in 0..3 {
            // Player attacks
            hostile.take_damage(15);

            // If hostile alive, it attacks back
            if hostile.is_alive() {
                player_hp -= hostile.attack() as i32;
            }
        }

        // After 3 rounds of 15 damage (45 total), drone (60 HP) should be alive
        assert!(hostile.is_alive());
        // Player should have taken damage
        assert!(player_hp < 100);
    }
}
