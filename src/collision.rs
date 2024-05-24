use crate::{GameState, GameplaySet};
use bevy::math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;
use crate::movement::Velocity;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>().add_systems(
            PostUpdate,
            (
                update_hitbox_positions,
                render_hitbox_gizmos, // todo: toggleable/debug
                detect_collisions,
            )
                .chain()
                .run_if(in_state(GameState::Playing))
                .in_set(GameplaySet::Collisions),
        );
    }
}

pub enum HitBox {
    Circle(BoundingCircle),
    Aabb(Aabb2d),
}

#[derive(Event)]
#[allow(unused)]
pub struct CollisionEvent {
    pub entity1: Entity,
    pub entity2: Entity,
    pub velocity1: Vec2,
    pub velocity2: Vec2,
    pub normal: Vec2,
}

#[derive(Component)]
pub struct Collider {
    pub layer: CollisionLayer,
    pub size: Vec2,
    pub hit_box: HitBox,
}

impl Collider {
    pub fn new_aabb(layer: CollisionLayer, half_size: Vec2) -> Self {
        Self {
            layer,
            size: half_size,
            hit_box: HitBox::Aabb(Aabb2d::new(Vec2::ZERO, half_size)),
        }
    }

    pub fn new_circle(layer: CollisionLayer, radius: f32) -> Self {
        Self {
            layer,
            size: Vec2::new(radius, radius),
            hit_box: HitBox::Circle(BoundingCircle::new(Vec2::ZERO, radius)),
        }
    }
}

#[allow(unused)]
pub enum CollisionLayer {
    Player,
    Enemy,
    PlayerProjectile,
    EnemyProjectile,
}

impl CollisionLayer {
    fn should_collide(&self, other: &CollisionLayer) -> bool {
        matches!(
            (self, other),
            (CollisionLayer::PlayerProjectile, CollisionLayer::Enemy)
                | (CollisionLayer::Enemy, CollisionLayer::PlayerProjectile)
                | (CollisionLayer::EnemyProjectile, CollisionLayer::Player)
                | (
                    CollisionLayer::EnemyProjectile,
                    CollisionLayer::EnemyProjectile
                )
        )
    }
}

impl HitBox {
    fn new_circle(center: Vec2, scale: f32, radius: f32) -> Self {
        Self::Circle(BoundingCircle::new(center, radius * scale))
    }

    fn new_aabb(center: Vec2, scale: Vec2, half_size: Vec2) -> Self {
        Self::Aabb(Aabb2d::new(center, half_size * scale))
    }

    fn intersects(&self, other: &HitBox) -> bool {
        match (self, other) {
            (HitBox::Circle(c1), HitBox::Circle(c2)) => c1.intersects(c2),
            (HitBox::Aabb(a1), HitBox::Aabb(a2)) => a1.intersects(a2),
            (HitBox::Circle(c), HitBox::Aabb(a)) | (HitBox::Aabb(a), HitBox::Circle(c)) => {
                c.intersects(a)
            }
        }
    }
}

fn update_hitbox_positions(mut collider_query: Query<(&Transform, &mut Collider)>) {
    for (transform, mut collider) in collider_query.iter_mut() {
        match collider.hit_box {
            HitBox::Aabb(_) => {
                collider.hit_box = HitBox::new_aabb(
                    transform.translation.xy(),
                    transform.scale.xy(),
                    collider.size,
                );
            }
            HitBox::Circle(_) => {
                collider.hit_box = HitBox::new_circle(
                    transform.translation.xy(),
                    transform.scale.x,
                    collider.size.x,
                );
            }
        }
    }
}

fn detect_collisions(
    collider_query: Query<(Entity, &Collider, &Transform, &Velocity)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let entities: Vec<_> = collider_query.iter().collect();

    for (i, (entity1, collider1, transform1, velocity1)) in entities.iter().enumerate() {
        for (entity2, collider2, transform2, velocity2) in entities.iter().skip(i + 1) {
            if collider1.layer.should_collide(&collider2.layer)
                && collider1.hit_box.intersects(&collider2.hit_box)
            {
                let normal = (transform2.translation.xy() - transform1.translation.xy()).normalize();
                collision_events.send(CollisionEvent {
                    entity1: *entity1,
                    entity2: *entity2,
                    velocity1: velocity1.0,
                    velocity2: velocity2.0,
                    normal
                });
            }
        }
    }
}

fn render_hitbox_gizmos(mut gizmos: Gizmos, collider_query: Query<&Collider>) {
    for collider in collider_query.iter() {
        match &collider.hit_box {
            HitBox::Circle(c) => {
                gizmos.circle_2d(c.center(), c.radius(), Color::GREEN);
            }
            HitBox::Aabb(a) => {
                let center = a.center();
                let half_size = a.half_size();
                gizmos.rect_2d(center, 0.0, half_size * 2.0, Color::RED);
            }
        }
    }
}
