use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationSet {
    pub animations: Vec<Vec<usize>>, // vector for all directions, containing each frame
}
