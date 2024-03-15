mod camera;
mod ecs;
mod input;

use ecs::Component::*;
use ecs::*;
use ggez::GameError::CustomError;
use std::collections::HashMap;

use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam};
use ggez::{Context, ContextBuilder, GameResult};
use rapier2d::prelude::*;

use crate::camera::*;

pub(crate) type Entity = Vec<Component>;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("2D Platformer", "CosmoBrain")
        .window_mode(WindowMode::default().dimensions(1920.0 / 2.0, 1080.0 / 2.0))
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = Platformer::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct PhysicsWrapper {
    pub physics_pipeline: PhysicsPipeline,
    pub gravity: Vector<f32>,
    pub integration_parameters: IntegrationParameters,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
    pub physics_hooks: (),
    pub event_handler: (),
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
}
impl PhysicsWrapper {
    pub fn update_physics_pipeline(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &self.physics_hooks,
            &self.event_handler,
        )
    }
}

struct Platformer {
    entities: HashMap<String, Vec<Component>>,
}

impl Platformer {
    pub fn new(_ctx: &mut Context) -> Platformer {
        let floor_y = 1080.0 / 25.0;
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        collider_set.insert(
            ColliderBuilder::cuboid(1920.0 / 50.0, 2.0)
                .translation([0.0, floor_y + 2.0].into())
                .build(),
        );
        let ball_rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![38.4, 0.0])
            .build();
        let ball_collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let ball_body_handle = rigid_body_set.insert(ball_rigid_body);
        collider_set.insert_with_parent(ball_collider, ball_body_handle, &mut rigid_body_set);

        let gravity = vector![0.0, 9.81];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let query_pipeline = QueryPipeline::new();
        let physics_hooks = ();
        let event_handler = ();
        let physics = PhysicsWrapper {
            physics_pipeline,
            gravity,
            integration_parameters,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            query_pipeline,
            physics_hooks,
            event_handler,
            rigid_body_set,
            collider_set,
        };

        let camera = Camera([0.0, 0.0].into(), [1920.0 / 25.0, 1080.0 / 25.0].into());

        let mut entities = HashMap::new();
        entities.insert("physics".into(), vec![Physics(Box::new(physics))]);
        entities.insert(
            "player".into(),
            vec![PlayerTag, BallCollider(ball_body_handle)],
        );
        entities.insert("camera".into(), vec![CameraTag, camera]);
        entities.insert("keymap".into(), vec![Keymap(HashMap::new())]);

        Platformer { entities }
    }
}

macro_rules! get_entity {
    ($entities:expr, $(let $pattern:ident = $getter:ident $name:expr$(;)?)*) => {
        $(let $pattern = $entities.$getter($name).unwrap() else {
            return Err(CustomError(format!(
                "Can't find the {name} component!",
                name = $name
            )));
        };)*
    };
    ($(let $pattern:ident = $entity:expr$(;)?)*) => {
        $(let $pattern = $entity else {
            return Err(CustomError(
                "Can't destructure this entity!".into()
            ));
        };)*
    };

    ($entities:expr, $(let $pattern:pat = $getter:ident $name:expr$(;)?)*) => {
        $(let $pattern = $entities.$getter($name).unwrap()[..] else {
            return Err(CustomError(format!(
                "Can't find the {name} component!",
                name = $name
            )));
        };)*
    };
    ($(let $pattern:pat = $entity:expr$(;)?)*) => {
        $(let $pattern = $entity[..] else {
            return Err(CustomError(
                "Can't destructure this entity!".into()
            ));
        };)*
    };
}

impl EventHandler for Platformer {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 120;
        while ctx.time.check_update_time(DESIRED_FPS) {
            get_entity! { self.entities,
                let player = get_mut "player";
                let physics = get_mut "physics";
                let keys = get "keys";
            };
            player_controller(keys, player, physics).unwrap();
            update_physics(physics).unwrap();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        get_entity! { self.entities,
            let [PlayerTag, BallCollider(player_ball_handle), ..] = get "player";
            let [Physics(ref physics), ..] = get "physics";
            let [CameraTag, Camera(camera_position, camera_size)] = get "camera";
        }

        canvas.set_screen_coordinates(graphics::Rect {
            x: 0.0,
            y: 0.0,
            w: 1920.0,
            h: 1080.0,
        });

        canvas.draw(
            &graphics::Quad,
            DrawParam::default()
                .dest(
                    world_to_screen_point(
                        *physics.rigid_body_set[player_ball_handle].translation()
                            - vector![0.5, 0.5],
                        camera_position,
                        camera_size,
                        [0.0, 0.0],
                        [1920.0, 1080.0],
                    )
                    .into_array(),
                )
                .scale(
                    world_to_screen_offset([0.5 * 2.0, 0.5 * 2.0], camera_size, [1920.0, 1080.0])
                        .into_array(),
                )
                .color(Color::from_rgb(200, 200, 200)),
        );

        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        if let Some(keycode) = input.keycode {
            get_entity! {self.entities,
                let [Keymap(ref mut map)] = get_mut "keymap";
            }
            map.insert(keycode, true);
        }
        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
    ) -> Result<(), ggez::GameError> {
        if let Some(keycode) = input.keycode {
            get_entity! {self.entities,
                let [Keymap(ref mut map)] = get_mut "keymap";
            }
            map.insert(keycode, false);
        }
        Ok(())
    }
}

trait IntoArray<const N: usize> {
    fn into_array(self) -> [f32; N];
}
impl IntoArray<2> for Vector<f32> {
    fn into_array(self) -> [f32; 2] {
        [self.x, self.y]
    }
}
impl IntoArray<2> for Point<f32> {
    fn into_array(self) -> [f32; 2] {
        [self.coords.x, self.coords.y]
    }
}

fn update_physics(physics: &mut Vec<Component>) -> GameResult {
    get_entity!(let [Component::Physics(ref mut physics)] = physics);
    physics.as_mut().update_physics_pipeline();
    Ok(())
}

fn player_controller(
    keymap: &Vec<Component>,
    player: &mut Vec<Component>,
    physics: &mut Vec<Component>,
) -> GameResult {
    get_entity! {
        let [Component::Physics(ref mut physics)] = physics;
        let [PlayerTag, BallCollider(ball_body_handle)] = player;
        let [Keymap(ref keys)] = keymap;
    };
    dbg!("Working!");
    Ok(())
}
