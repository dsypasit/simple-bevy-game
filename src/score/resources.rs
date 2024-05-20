use bevy::ecs::system::Resource;
#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Default)]
pub struct HighestScore {
    pub value: u32,
}
