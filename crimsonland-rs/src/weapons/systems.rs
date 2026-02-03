//! Weapon systems

use bevy::prelude::*;
use rand::Rng;

use super::components::*;
use super::registry::WeaponRegistry;
use crate::creatures::components::{Creature, CreatureHealth, MarkedForDespawn};
use crate::player::components::{AimDirection, Firing, Player};

/// Event to fire a weapon
#[derive(Event)]
pub struct FireWeaponEvent {
    pub shooter: Entity,
    pub position: Vec3,
    pub direction: Vec2,
    pub weapon_id: WeaponId,
}

/// Event when a projectile hits something
#[derive(Event)]
pub struct ProjectileHitEvent {
    pub projectile: Entity,
    pub target: Entity,
    pub damage: f32,
    pub position: Vec3,
}

/// System that handles weapon firing from player input
pub fn fire_weapon_system(
    mut commands: Commands,
    weapon_registry: Res<WeaponRegistry>,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &Transform,
            &AimDirection,
            &Firing,
            &mut EquippedWeapon,
        ),
        With<Player>,
    >,
) {
    for (entity, transform, aim, firing, mut weapon) in query.iter_mut() {
        // Update cooldown
        weapon.fire_cooldown = (weapon.fire_cooldown - time.delta_seconds()).max(0.0);

        if !firing.is_firing || !weapon.can_fire() {
            continue;
        }

        let Some(weapon_data) = weapon_registry.get(weapon.weapon_id) else {
            continue;
        };

        // Fire projectiles
        let mut rng = rand::thread_rng();
        let position = transform.translation;

        for _ in 0..weapon_data.projectiles_per_shot {
            // Apply spread
            let spread_angle = rng.gen_range(-weapon_data.spread..weapon_data.spread);
            let base_angle = aim.angle;
            let final_angle = base_angle + spread_angle;
            let direction = Vec2::new(final_angle.cos(), final_angle.sin());

            // Determine projectile color based on weapon type
            let color = get_projectile_color(weapon.weapon_id);
            let size = get_projectile_size(weapon.weapon_id);

            // Spawn projectile
            let mut projectile_commands = commands.spawn(ProjectileBundle::new(
                weapon.weapon_id,
                weapon_data.damage,
                entity,
                position,
                direction,
                weapon_data.projectile_speed,
                weapon_data.projectile_lifetime,
                color,
                size,
            ));

            // Add homing component if needed
            if weapon_data.homing {
                projectile_commands.insert(Homing {
                    turn_rate: 3.0,
                    target: None,
                });
            }

            // Add explosive component if needed
            if weapon_data.is_explosive() {
                projectile_commands.insert(Explosive {
                    radius: weapon_data.explosive_radius,
                    damage: weapon_data.damage,
                });
            }
        }

        // Consume ammo and set cooldown
        weapon.consume_ammo();
        weapon.fire_cooldown = weapon_data.fire_cooldown();
    }
}

fn get_projectile_color(weapon_id: WeaponId) -> Color {
    match weapon_id {
        WeaponId::Pistol | WeaponId::Magnum => Color::srgb(1.0, 0.9, 0.3),
        WeaponId::Uzi | WeaponId::Smg | WeaponId::DualSmg => Color::srgb(1.0, 0.8, 0.2),
        WeaponId::AssaultRifle | WeaponId::MachineGun | WeaponId::Minigun => {
            Color::srgb(1.0, 0.7, 0.1)
        }
        WeaponId::Shotgun | WeaponId::DoubleBarrel | WeaponId::Jackhammer => {
            Color::srgb(0.9, 0.6, 0.2)
        }
        WeaponId::Flamethrower | WeaponId::Blowtorch => Color::srgb(1.0, 0.4, 0.1),
        WeaponId::PlasmaRifle | WeaponId::PulseGun => Color::srgb(0.3, 0.8, 1.0),
        WeaponId::IonRifle | WeaponId::GaussGun | WeaponId::GaussShotgun => {
            Color::srgb(0.5, 0.5, 1.0)
        }
        WeaponId::RocketLauncher | WeaponId::GrenadeLauncher => Color::srgb(0.6, 0.3, 0.1),
        WeaponId::HomingMissile => Color::srgb(0.8, 0.2, 0.2),
        WeaponId::FreezeRay => Color::srgb(0.6, 0.9, 1.0),
        WeaponId::ShrinkRay => Color::srgb(0.8, 0.3, 0.8),
        _ => Color::srgb(1.0, 1.0, 0.5),
    }
}

fn get_projectile_size(weapon_id: WeaponId) -> f32 {
    match weapon_id {
        WeaponId::Pistol | WeaponId::Magnum => 6.0,
        WeaponId::Shotgun | WeaponId::DoubleBarrel | WeaponId::Jackhammer => 4.0,
        WeaponId::RocketLauncher | WeaponId::HomingMissile | WeaponId::GrenadeLauncher => 12.0,
        WeaponId::GaussGun | WeaponId::IonRifle => 10.0,
        WeaponId::Minigun => 5.0,
        WeaponId::Flamethrower | WeaponId::Blowtorch => 8.0,
        _ => 6.0,
    }
}

/// Moves projectiles based on their velocity
pub fn projectile_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Projectile>>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();
    }
}

/// Handles projectile collision with creatures
pub fn projectile_collision(
    mut commands: Commands,
    mut projectile_query: Query<
        (Entity, &Transform, &mut Projectile, Option<&Explosive>),
        Without<ProjectileDespawn>,
    >,
    mut creature_query: Query<
        (Entity, &Transform, &mut CreatureHealth),
        (With<Creature>, Without<MarkedForDespawn>),
    >,
    mut hit_events: EventWriter<ProjectileHitEvent>,
) {
    const COLLISION_RADIUS: f32 = 20.0;

    // Collect explosion data to apply after the main loop
    let mut explosions: Vec<(Vec2, f32, f32, Entity)> = Vec::new();

    for (projectile_entity, projectile_transform, mut projectile, explosive) in
        projectile_query.iter_mut()
    {
        let projectile_pos = projectile_transform.translation.truncate();

        for (creature_entity, creature_transform, mut creature_health) in creature_query.iter_mut()
        {
            let creature_pos = creature_transform.translation.truncate();
            let distance = projectile_pos.distance(creature_pos);

            if distance < COLLISION_RADIUS {
                // Apply damage
                creature_health.damage(projectile.damage);

                hit_events.send(ProjectileHitEvent {
                    projectile: projectile_entity,
                    target: creature_entity,
                    damage: projectile.damage,
                    position: projectile_transform.translation,
                });

                // Queue explosive damage for later
                if let Some(explosive) = explosive {
                    explosions.push((
                        projectile_pos,
                        explosive.radius,
                        explosive.damage,
                        creature_entity,
                    ));
                }

                // Check pierce
                if projectile.pierce_count > 0 {
                    projectile.pierce_count -= 1;
                } else {
                    commands.entity(projectile_entity).insert(ProjectileDespawn);
                    break;
                }
            }
        }
    }

    // Apply explosion damage
    for (center, radius, damage, already_hit) in explosions {
        for (entity, transform, mut health) in creature_query.iter_mut() {
            if entity == already_hit {
                continue;
            }

            let pos = transform.translation.truncate();
            let distance = center.distance(pos);

            if distance < radius {
                let falloff = 1.0 - (distance / radius);
                let explosion_damage = damage * falloff;
                health.damage(explosion_damage);
            }
        }
    }
}

/// Updates projectile lifetimes and marks expired ones for despawn
pub fn projectile_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime), (With<Projectile>, Without<ProjectileDespawn>)>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.tick(time.delta_seconds());
        if lifetime.is_expired() {
            commands.entity(entity).insert(ProjectileDespawn);
        }
    }
}

/// Removes projectiles marked for despawn
pub fn cleanup_projectiles(
    mut commands: Commands,
    query: Query<Entity, With<ProjectileDespawn>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Despawns all projectiles when leaving Playing state
pub fn despawn_all_projectiles(mut commands: Commands, query: Query<Entity, With<Projectile>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fire_weapon_event_can_be_created() {
        let event = FireWeaponEvent {
            shooter: Entity::PLACEHOLDER,
            position: Vec3::ZERO,
            direction: Vec2::X,
            weapon_id: WeaponId::Pistol,
        };
        assert_eq!(event.weapon_id, WeaponId::Pistol);
    }

    #[test]
    fn projectile_hit_event_can_be_created() {
        let event = ProjectileHitEvent {
            projectile: Entity::PLACEHOLDER,
            target: Entity::PLACEHOLDER,
            damage: 25.0,
            position: Vec3::new(10.0, 20.0, 0.0),
        };
        assert_eq!(event.damage, 25.0);
    }

    #[test]
    fn projectile_colors_are_distinct() {
        let pistol_color = get_projectile_color(WeaponId::Pistol);
        let plasma_color = get_projectile_color(WeaponId::PlasmaRifle);
        let freeze_color = get_projectile_color(WeaponId::FreezeRay);

        // These should be visually distinct
        assert_ne!(pistol_color, plasma_color);
        assert_ne!(plasma_color, freeze_color);
    }
}
