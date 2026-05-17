use rapier2d::prelude::*;
use nalgebra::Vector2;

pub const PIXELS_PER_METER: f32 = 50.0;
pub const GRAVITY: f32 = -25.0;

pub struct PhysicsWorld {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub gravity: Vector2<f32>,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        PhysicsWorld {
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            gravity: Vector2::new(0.0, GRAVITY),
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
        }
    }

    pub fn step(&mut self) {
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
            &(),
            &(),
        );
    }

    pub fn to_physics_pos(&self, x: f32, y: f32, screen_height: f32) -> Vector2<f32> {
        Vector2::new(x / PIXELS_PER_METER, (screen_height - y) / PIXELS_PER_METER)
    }

    pub fn create_ground(&mut self, screen_width: f32, screen_height: f32) -> RigidBodyHandle {
        let ground_y = self.to_physics_pos(0.0, screen_height - 50.0, screen_height).y;
        let ground_width = screen_width / PIXELS_PER_METER;

        let rigid_body = RigidBodyBuilder::fixed()
            .translation(Vector2::new(screen_width / 2.0 / PIXELS_PER_METER, ground_y))
            .build();
        let handle = self.rigid_body_set.insert(rigid_body);

        let collider = ColliderBuilder::cuboid(ground_width / 2.0, 0.5)
            .restitution(0.3)
            .friction(0.8)
            .build();
        self.collider_set.insert_with_parent(collider, handle, &mut self.rigid_body_set);

        handle
    }

    pub fn create_dynamic_ball(&mut self, x: f32, y: f32, radius: f32, screen_height: f32) -> RigidBodyHandle {
        let pos = self.to_physics_pos(x, y, screen_height);
        let phys_radius = radius / PIXELS_PER_METER;

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(pos)
            .linear_damping(0.5)
            .angular_damping(0.5)
            .build();
        let handle = self.rigid_body_set.insert(rigid_body);

        let collider = ColliderBuilder::ball(phys_radius)
            .restitution(0.6)
            .friction(0.5)
            .density(2.0)
            .build();
        self.collider_set.insert_with_parent(collider, handle, &mut self.rigid_body_set);

        handle
    }

    pub fn create_dynamic_box(&mut self, x: f32, y: f32, width: f32, height: f32, screen_height: f32) -> RigidBodyHandle {
        let pos = self.to_physics_pos(x, y, screen_height);
        let half_width = width / 2.0 / PIXELS_PER_METER;
        let half_height = height / 2.0 / PIXELS_PER_METER;

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(pos)
            .linear_damping(0.5)
            .angular_damping(0.5)
            .build();
        let handle = self.rigid_body_set.insert(rigid_body);

        let collider = ColliderBuilder::cuboid(half_width, half_height)
            .restitution(0.3)
            .friction(0.8)
            .density(1.0)
            .build();
        self.collider_set.insert_with_parent(collider, handle, &mut self.rigid_body_set);

        handle
    }

    pub fn apply_impulse(&mut self, handle: RigidBodyHandle, impulse: Vector2<f32>) {
        if let Some(body) = self.rigid_body_set.get_mut(handle) {
            body.apply_impulse(impulse, true);
        }
    }

    pub fn get_position(&self, handle: RigidBodyHandle) -> Option<Vector2<f32>> {
        self.rigid_body_set.get(handle).map(|body| body.translation().clone())
    }

    pub fn set_linear_velocity(&mut self, handle: RigidBodyHandle, velocity: Vector2<f32>) {
        if let Some(body) = self.rigid_body_set.get_mut(handle) {
            body.set_linvel(velocity, true);
        }
    }

    pub fn remove_body(&mut self, handle: RigidBodyHandle) {
        self.rigid_body_set.remove(
            handle,
            &mut self.island_manager,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            true
        );
    }

    pub fn is_sleeping(&self, handle: RigidBodyHandle) -> bool {
        self.rigid_body_set.get(handle).map(|body| body.is_sleeping()).unwrap_or(true)
    }

    pub fn wake_up(&mut self, handle: RigidBodyHandle) {
        if let Some(body) = self.rigid_body_set.get_mut(handle) {
            body.wake_up(true);
        }
    }
}