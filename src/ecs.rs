use std::collections::HashMap;

use ggez::winit::event::VirtualKeyCode;
use rapier2d::prelude::*;

use crate::PhysicsWrapper;

pub(crate) enum Component {
    BallCollider(RigidBodyHandle),
    PlayerTag,
    CameraTag,
    Physics(Box<PhysicsWrapper>),
    Camera(Point<f32>, Vector<f32>),
    Keymap(HashMap<VirtualKeyCode, bool>),
}
