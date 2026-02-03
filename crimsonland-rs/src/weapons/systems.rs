//! Weapon systems

use bevy::prelude::*;
use rand::Rng;

use super::components::*;
use super::registry::WeaponRegistry;
use crate::creatures::{Creature, CreatureHealth, CreatureSpeed, MarkedForDespawn};
use crate::perks::components::PerkBonuses;
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
/// Integrates perk bonuses: fire_rate_multiplier, damage_multiplier, crit_chance, accuracy_bonus, range_multiplier
#[allow(clippy::type_complexity)]
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
            &PerkBonuses,
        ),
        With<Player>,
    >,
) {
    for (entity, transform, aim, firing, mut weapon, perk_bonuses) in query.iter_mut() {
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
            // Apply spread with accuracy bonus (accuracy reduces spread)
            let spread_reduction = 1.0 - perk_bonuses.accuracy_bonus.min(0.9); // Cap at 90% reduction
            let effective_spread = weapon_data.spread * spread_reduction;
            let spread_angle = rng.gen_range(-effective_spread..effective_spread);
            let base_angle = aim.angle;
            let final_angle = base_angle + spread_angle;
            let direction = Vec2::new(final_angle.cos(), final_angle.sin());

            // Calculate damage with perk bonuses
            let mut damage = weapon_data.damage * perk_bonuses.damage_multiplier;

            // Check for critical hit
            if perk_bonuses.crit_chance > 0.0 && rng.gen::<f32>() < perk_bonuses.crit_chance {
                damage *= perk_bonuses.crit_multiplier;
            }

            // Apply range multiplier to projectile lifetime
            let projectile_lifetime = weapon_data.projectile_lifetime * perk_bonuses.range_multiplier;

            // Determine projectile color based on weapon type
            let color = get_projectile_color(weapon.weapon_id);
            let size = get_projectile_size(weapon.weapon_id);

            // Spawn projectile
            let mut projectile_commands = commands.spawn(ProjectileBundle::new(
                weapon.weapon_id,
                damage,
                entity,
                position,
                direction,
                weapon_data.projectile_speed,
                projectile_lifetime,
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
                    damage,
                });
            }

            // Add special weapon components
            match weapon.weapon_id {
                WeaponId::ChainReactor => {
                    projectile_commands.insert(ChainLightning::new(5, 150.0, 0.8));
                }
                WeaponId::SplitterGun => {
                    projectile_commands.insert(Splitter::new(2, 3, 0.6));
                }
                WeaponId::FreezeRay => {
                    projectile_commands.insert(Freezing {
                        slow_amount: 0.3,
                        duration: 3.0,
                    });
                }
                _ => {}
            }
        }

        // Consume ammo and set cooldown (fire rate multiplier reduces cooldown)
        weapon.consume_ammo();
        weapon.fire_cooldown = weapon_data.fire_cooldown() / perk_bonuses.fire_rate_multiplier;
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

/// Updates homing projectiles to track targets
/// Homing missiles acquire and track the nearest creature
#[allow(clippy::type_complexity)]
pub fn homing_projectile_update(
    time: Res<Time>,
    creature_query: Query<(Entity, &Transform), (With<Creature>, Without<MarkedForDespawn>)>,
    mut homing_query: Query<
        (&Transform, &mut Homing, &mut Velocity),
        (With<Projectile>, Without<Creature>),
    >,
) {
    for (projectile_transform, mut homing, mut velocity) in homing_query.iter_mut() {
        let projectile_pos = projectile_transform.translation.truncate();

        // Find or update target
        let target_pos = if let Some(target_entity) = homing.target {
            // Check if current target is still valid
            if let Ok((_, target_transform)) = creature_query.get(target_entity) {
                Some(target_transform.translation.truncate())
            } else {
                // Target died, clear it
                homing.target = None;
                None
            }
        } else {
            None
        };

        // If no target, find nearest creature
        let target_pos = target_pos.or_else(|| {
            let mut nearest: Option<(Entity, f32, Vec2)> = None;

            for (entity, creature_transform) in creature_query.iter() {
                let creature_pos = creature_transform.translation.truncate();
                let distance = projectile_pos.distance(creature_pos);

                if nearest.is_none() || distance < nearest.unwrap().1 {
                    nearest = Some((entity, distance, creature_pos));
                }
            }

            if let Some((entity, _, pos)) = nearest {
                homing.target = Some(entity);
                Some(pos)
            } else {
                None
            }
        });

        // Turn toward target
        if let Some(target_pos) = target_pos {
            let to_target = target_pos - projectile_pos;
            let desired_direction = to_target.normalize_or_zero();

            let current_speed = velocity.0.length();
            let current_direction = velocity.0.normalize_or_zero();

            // Smoothly rotate toward target based on turn rate
            let turn_amount = homing.turn_rate * time.delta_seconds();
            let new_direction = current_direction
                .lerp(desired_direction, turn_amount.min(1.0))
                .normalize_or_zero();

            velocity.0 = new_direction * current_speed;
        }
    }
}

/// Handles projectile collision with creatures
/// Also handles special weapon effects: chain lightning, splitter, freezing
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn projectile_collision(
    mut commands: Commands,
    mut projectile_query: Query<
        (
            Entity,
            &Transform,
            &mut Projectile,
            Option<&Explosive>,
            Option<&mut ChainLightning>,
            Option<&Splitter>,
            Option<&Freezing>,
        ),
        Without<ProjectileDespawn>,
    >,
    mut creature_query: Query<
        (Entity, &Transform, &mut CreatureHealth, &mut CreatureSpeed),
        (With<Creature>, Without<MarkedForDespawn>),
    >,
    mut hit_events: EventWriter<ProjectileHitEvent>,
) {
    const COLLISION_RADIUS: f32 = 20.0;

    // Collect data for effects to apply after the main loop
    let mut explosions: Vec<(Vec2, f32, f32, Entity)> = Vec::new();
    let mut chain_spawns: Vec<(Vec2, f32, u32, f32, f32, Vec<Entity>, Entity)> = Vec::new();
    let mut split_spawns: Vec<(Vec2, Vec2, f32, u32, u32, f32, Entity)> = Vec::new();

    for (
        projectile_entity,
        projectile_transform,
        mut projectile,
        explosive,
        mut chain_lightning,
        splitter,
        freezing,
    ) in projectile_query.iter_mut()
    {
        let projectile_pos = projectile_transform.translation.truncate();

        for (creature_entity, creature_transform, mut creature_health, mut creature_speed) in
            creature_query.iter_mut()
        {
            // Skip if chain lightning already hit this target
            if let Some(ref chain) = chain_lightning {
                if chain.already_hit.contains(&creature_entity) {
                    continue;
                }
            }

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

                // Apply freezing effect
                if let Some(freeze) = &freezing {
                    creature_speed.0 *= freeze.slow_amount;
                }

                // Queue explosive damage for later
                if let Some(explosive) = explosive {
                    explosions.push((
                        projectile_pos,
                        explosive.radius,
                        explosive.damage,
                        creature_entity,
                    ));
                }

                // Queue chain lightning spawn
                if let Some(ref mut chain) = chain_lightning {
                    if chain.jumps_remaining > 0 {
                        let mut already_hit = chain.already_hit.clone();
                        already_hit.push(creature_entity);
                        chain_spawns.push((
                            creature_pos,
                            projectile.damage * chain.damage_falloff,
                            chain.jumps_remaining - 1,
                            chain.jump_range,
                            chain.damage_falloff,
                            already_hit,
                            projectile.owner,
                        ));
                        chain.already_hit.push(creature_entity);
                    }
                }

                // Queue splitter spawn
                if let Some(split) = splitter {
                    if split.splits_remaining > 0 {
                        let velocity_dir = (creature_pos - projectile_pos).normalize_or_zero();
                        split_spawns.push((
                            creature_pos,
                            velocity_dir,
                            projectile.damage * split.damage_multiplier,
                            split.splits_remaining - 1,
                            split.split_count,
                            split.damage_multiplier,
                            projectile.owner,
                        ));
                    }
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
        for (entity, transform, mut health, _) in creature_query.iter_mut() {
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

    // Spawn chain lightning projectiles
    for (pos, damage, jumps, range, falloff, already_hit, owner) in chain_spawns {
        // Find nearest creature not already hit
        let mut nearest: Option<(Entity, Vec2)> = None;
        let mut nearest_dist = f32::MAX;

        for (entity, transform, _, _) in creature_query.iter() {
            if already_hit.contains(&entity) {
                continue;
            }
            let creature_pos = transform.translation.truncate();
            let dist = pos.distance(creature_pos);
            if dist < range && dist < nearest_dist {
                nearest = Some((entity, creature_pos));
                nearest_dist = dist;
            }
        }

        if let Some((_, target_pos)) = nearest {
            let direction = (target_pos - pos).normalize_or_zero();
            let mut new_chain = ChainLightning::new(jumps, range, falloff);
            new_chain.already_hit = already_hit;

            commands.spawn((
                ProjectileBundle::new(
                    WeaponId::ChainReactor,
                    damage,
                    owner,
                    Vec3::new(pos.x, pos.y, 0.0),
                    direction,
                    800.0, // Fast chain lightning
                    0.5,   // Short lifetime
                    Color::srgb(0.5, 0.7, 1.0), // Blue lightning color
                    4.0,
                ),
                new_chain,
            ));
        }
    }

    // Spawn splitter projectiles
    for (pos, base_dir, damage, splits, count, mult, owner) in split_spawns {
        let angle_spread = std::f32::consts::PI / 3.0; // 60 degree spread
        let angle_step = angle_spread / (count as f32 - 1.0).max(1.0);
        let start_angle = base_dir.y.atan2(base_dir.x) - angle_spread / 2.0;

        for i in 0..count {
            let angle = start_angle + angle_step * i as f32;
            let direction = Vec2::new(angle.cos(), angle.sin());

            let mut projectile_commands = commands.spawn(ProjectileBundle::new(
                WeaponId::SplitterGun,
                damage,
                owner,
                Vec3::new(pos.x, pos.y, 0.0),
                direction,
                500.0,
                1.5,
                Color::srgb(0.8, 0.4, 1.0), // Purple splitter color
                4.0,
            ));

            if splits > 0 {
                projectile_commands.insert(Splitter::new(splits, count, mult));
            }
        }
    }
}

/// Updates projectile lifetimes and marks expired ones for despawn
#[allow(clippy::type_complexity)]
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
