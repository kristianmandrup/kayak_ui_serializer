use bevy::{prelude::{Vec3, Transform, Quat}};

use crate::{serialized::{STransform, SVec3, SQuat}, ui_bevy_style::to_f32};

pub fn to_vec3(vec3: SVec3) -> Option<Vec3> {
    let vec = vec![vec3.x, vec3.y, vec3.z];
    let col = vec.iter().map(|str| { 
        to_f32(str)
    });
    let nums: Vec<Option<f32>> = col.collect();
     let tuples = (nums[0], nums[1], nums[2]);
     match tuples {
        (Some(x), Some(y), Some(z)) => Some(Vec3 { x, y, z }),
        _ => None
     }
}

pub fn to_quat(quat: SQuat) -> Option<Quat> {
    let vec = vec![quat.x, quat.y, quat.z, quat.w];
    let col = vec.iter().map(|str| { 
        to_f32(str)
    });
    let nums: Vec<Option<f32>> = col.collect();
     let tuples = (nums[0], nums[1], nums[2], nums[3]);
     match tuples {
        (Some(x), Some(y), Some(z), Some(w)) => Some(Quat { x, y, z, w }),
        _ => None
     }
}


pub struct TransformBuilder {
    node: STransform,
}
impl TransformBuilder {
    pub fn new(node: STransform) -> Self {
        Self {
            node
        }
    }
     
    pub fn translation(&self) -> Option<Vec3> {
        let prop = &self.node.translation.clone();        
        if let Some(val) = prop.clone() {
            to_vec3(val)
        } else {
            None
        }
    }

    pub fn scale(&self) -> Option<Vec3> {
        let prop = &self.node.scale.clone();        
        if let Some(val) = prop.clone() {
            to_vec3(val)
        } else {
            None
        }        
    }

    pub fn rotation(&self) -> Option<Quat> {
        let prop = &self.node.rotation.clone();        
        if let Some(val) = prop.clone() {
            to_quat(val)
        } else {
            None
        }        
    }

    pub fn parse(&self) -> Result<Transform, &'static str> {        
        let translation = self.translation();
        let scale = self.scale();
        let rotation = self.rotation();
        let mut transform = Transform::default();
        if let Some(val) = translation {
            transform.translation = val;    
        }        
        if let Some(val) = scale {
            transform.scale = val;    
        }        
        if let Some(val) = rotation {
            transform.rotation = val;    
        }        
        Ok(transform)
    }
}
