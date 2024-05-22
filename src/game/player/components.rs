use bevy::ecs::component::Component;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec2,
}

#[derive(Event)]
pub struct CreateBulletEvent {
    pub position: Vec3,
}
