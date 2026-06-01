#![windows_subsystem = "windows"]

mod assets;
mod fps;
mod object;
mod physics;
mod sprite;

use glam::Vec2;
use rapier2d::prelude::{ColliderBuilder, RigidBodyBuilder};
use sdl3::{
	event::Event,
	pixels::Color,
	rect::Rect,
	sys::render::{SDL_RendererLogicalPresentation, SDL_SetRenderVSync},
};
use std::time::{Duration, Instant};

use crate::{
	assets::Assets,
	fps::FPS,
	object::{Object, PhysicsHandle},
	physics::Physics,
	sprite::Sprite,
};

const TICK_RATE: f64 = 60_f64;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const GAME_WIDTH: u32 = 320;
const GAME_HEIGHT: u32 = 180;

fn main() {
	let sdl_context = sdl3::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem
		.window("sdl3-sprites", WINDOW_WIDTH, WINDOW_HEIGHT)
		.position_centered()
		.resizable()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas();
	unsafe {
		SDL_SetRenderVSync(canvas.raw(), 1);
	}

	canvas
		.set_logical_size(GAME_WIDTH, GAME_HEIGHT, SDL_RendererLogicalPresentation::LETTERBOX)
		.unwrap();

	// physics ===============================================

	let mut physics = Physics::new();

	let collider = ColliderBuilder::cuboid(100_f32, 0.5).build();
	let collider_handle = physics
		.collider_set
		.insert(collider);

	let mut ground = Object::from_collider(collider_handle);

	let rigid_body = RigidBodyBuilder::dynamic()
		.translation(rapier2d::prelude::Vec2::new(3.5, 10_f32))
		.build();
	let collider = ColliderBuilder::cuboid(0.5, 0.5)
		.restitution(0.7)
		.build();
	let cube_body_handle = physics
		.rigid_body_set
		.insert(rigid_body);
	physics
		.collider_set
		.insert_with_parent(collider, cube_body_handle, &mut physics.rigid_body_set);

	let mut cube = Object::from_rigid_body(cube_body_handle);

	// sprites ===============================================

	let texture_creator = canvas.texture_creator();

	let assets = Assets::new(&texture_creator);
	let mut sprite = Sprite::from_texture(assets.get_texture("cube"));

	// =======================================================

	let mut event_pump = sdl_context
		.event_pump()
		.unwrap();

	let mut last_frame = Instant::now();
	let mut accumulator = Duration::new(0, 0);
	let tick_time = Duration::from_secs_f64(1_f64 / TICK_RATE);

	let mut fps = FPS::new();

	'running: loop {
		let now = Instant::now();
		let frame_duration = now.duration_since(last_frame);
		accumulator += frame_duration;
		last_frame = now;

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {
					..
				} => break 'running,
				_ => {}
			}
		}

		while accumulator >= tick_time {
			let mouse = event_pump.relative_mouse_state();

			if mouse.is_mouse_button_pressed(sdl3::mouse::MouseButton::Left) {
				let x = mouse.x();
				let y = mouse.y();

				let handle = match cube.physics_handle {
					PhysicsHandle::RigidBody(handle) => handle,
					_ => panic!(),
				};

				physics.rigid_body_set[handle]
					.apply_impulse(rapier2d::prelude::Vec2::new(x / 50_f32, -y / 50_f32), true);
			}

			physics.step();

			ground.update(&physics);
			cube.update(&physics);

			accumulator -= tick_time;
		}

		let display_fps = fps.fps(frame_duration);

		canvas
			.window_mut()
			.set_title(&format!("physics-test-2 | {:.0} FPS", display_fps))
			.unwrap();

		canvas.set_draw_color(Color::RED);
		canvas.clear();

		let cube_pos = cube.logic_pos(GAME_HEIGHT, 16_f32);
		sprite.rotation = cube.rotation;
		sprite.draw(&mut canvas, Vec2::new(cube_pos.x, cube_pos.y));

		canvas.set_draw_color(Color::WHITE);

		let ground_pos = ground.logic_pos(GAME_HEIGHT, 16_f32);
		let ground_size = ground.logic_size(16_f32);
		let ground_rect = Rect::new(
			ground_pos.x as i32,
			ground_pos.y as i32,
			ground_size.x as u32,
			ground_size.y as u32,
		);

		canvas
			.fill_rect(ground_rect)
			.unwrap();

		canvas.present();
	}
}
