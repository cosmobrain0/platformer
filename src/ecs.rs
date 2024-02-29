use rapier2d::prelude::*;

use crate::PhysicsWrapper;

pub(crate) enum Component {
    BallCollider(RigidBodyHandle),
    PlayerTag,
    Physics(Box<PhysicsWrapper>),
}
