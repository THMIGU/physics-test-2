use rapier2d::{dynamics::RigidBodyHandle, geometry::ColliderHandle, math::Vec2};

use crate::physics::Physics;

#[derive(Clone, Copy)]
pub enum PhysicsHandle {
	RigidBody(RigidBodyHandle),
	Collider(ColliderHandle),
}

pub struct Object {
	pub physics_handle: PhysicsHandle,
	pub size: Vec2,
	pub pos: Vec2,
	pub rotation: f32,
}

impl Object {
	pub fn from_rigid_body(handle: RigidBodyHandle) -> Self {
		Self {
			physics_handle: PhysicsHandle::RigidBody(handle),
			size: Vec2::new(0_f32, 0_f32),
			pos: Vec2::new(0_f32, 0_f32),
			rotation: 0_f32,
		}
	}

	pub fn from_collider(handle: ColliderHandle) -> Self {
		Self {
			physics_handle: PhysicsHandle::Collider(handle),
			size: Vec2::new(0_f32, 0_f32),
			pos: Vec2::new(0_f32, 0_f32),
			rotation: 0_f32,
		}
	}

	fn pos(&self, physics: &Physics) -> Vec2 {
		match self.physics_handle {
			PhysicsHandle::RigidBody(handle) => {
				let body = &physics.rigid_body_set[handle];
				body.translation()
			}
			PhysicsHandle::Collider(handle) => {
				let collider = &physics.collider_set[handle];
				collider.translation()
			}
		}
	}

	fn size(&self, physics: &Physics) -> Vec2 {
		match self.physics_handle {
			PhysicsHandle::RigidBody(handle) => {
				let body = &physics.rigid_body_set[handle];

				let collider_handle = body.colliders()[0];
				let collider = &physics.collider_set[collider_handle];

				let aabb = collider
					.shape()
					.compute_local_aabb();

				Vec2::new(aabb.maxs.x - aabb.mins.x, aabb.maxs.y - aabb.mins.y)
			}
			PhysicsHandle::Collider(handle) => {
				let collider = &physics.collider_set[handle];
				let aabb = collider
					.shape()
					.compute_local_aabb();

				Vec2::new(aabb.maxs.x - aabb.mins.x, aabb.maxs.y - aabb.mins.y)
			}
		}
	}

	fn rotation(&self, physics: &Physics) -> f32 {
		match self.physics_handle {
			PhysicsHandle::RigidBody(handle) => {
				let body = &physics.rigid_body_set[handle];
				body.position()
					.rotation
					.angle()
					.to_degrees()
			}
			PhysicsHandle::Collider(handle) => {
				let collider = &physics.collider_set[handle];
				collider
					.position()
					.rotation
					.angle()
					.to_degrees()
			}
		}
	}

	pub fn update(&mut self, physics: &Physics) {
		self.pos = self.pos(physics);
		self.size = self.size(physics);
		self.rotation = self.rotation(physics);
	}

	pub fn logic_pos(&self, height: u32, scl: f32) -> Vec2 {
		Vec2::new(
			(self.pos.x * scl) as i32 as f32,
			(height as i32 - (self.pos.y * scl) as i32 - (self.size.y * scl / 2_f32) as i32) as f32,
		)
	}

	pub fn logic_size(&self, scl: f32) -> Vec2 {
		Vec2::new((self.size.x * scl) as u32 as f32, (self.size.y * scl) as u32 as f32)
	}
}
