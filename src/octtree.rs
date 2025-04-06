use bevy::prelude::*;


#[derive(Asset, Reflect, Resource)]
pub struct Octtree {
    pub data: Vec<u32>
}

impl Default for Octtree {
    fn default() -> Self { Octtree::new() }
}


impl Octtree {
    pub fn new() -> Self {
        // Octtree data ordering:
        // (0, 0, 0), (1, 0, 0), (0, 1, 0), (1, 1, 0)
        Octtree {
            data: [0, 0, 0, 0, 0, 0, 0, 0].into()
        }
    }

    pub fn data_ordering() -> Vec<UVec3> {
        [
            UVec3::new(0,0,0), UVec3::new(1,0,0), UVec3::new(0,1,0), UVec3::new(1,1,0),
            UVec3::new(0,0,1), UVec3::new(1,0,1), UVec3::new(0,1,1), UVec3::new(1,1,1)
        ].into()
    }
}