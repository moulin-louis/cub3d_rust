use crate::Tdata;
use std::process::exit;
use macroquad::input::KeyCode::Escape;
use macroquad::input::KeyCode::Right;
use macroquad::input::KeyCode::Left;
use macroquad::input::KeyCode::W;
use macroquad::input::KeyCode::S;
use macroquad::input::KeyCode::A;
use macroquad::input::KeyCode::D;
use macroquad::prelude::*;

static ROTSPEED: f32 = 0.05;
static MOVSPEED:f32 = 0.05;

pub fn handle_input( data:&mut Tdata, map: &Vec<Vec<i32>> ) -> i32 {
	if is_key_down(Escape) {
		exit(1);
	}
	if is_key_down(Left) {
		let mut old_var:f32;
		old_var = data.dir_x;
		data.dir_x = data.dir_x * (-ROTSPEED).cos() - data.dir_y * (-ROTSPEED).sin();
		data.dir_y = old_var * (-ROTSPEED).sin() + data.dir_y * (-ROTSPEED).cos();
		old_var = data.plane_x;
		data.plane_x = data.plane_x * (-ROTSPEED).cos() - data.plane_y * (-ROTSPEED).sin();
		data.plane_y = old_var * (-ROTSPEED).sin() + data.plane_y * (-ROTSPEED).cos();
	}
	if is_key_down(Right) {
		let mut old_var:f32;
		old_var = data.dir_x;
		data.dir_x = data.dir_x * ROTSPEED.cos() - data.dir_y * ROTSPEED.sin();
		data.dir_y = old_var * ROTSPEED.sin() + data.dir_y * ROTSPEED.cos();
		old_var = data.plane_x;
		data.plane_x = data.plane_x * ROTSPEED.cos() - data.plane_y * ROTSPEED.sin();
		data.plane_y = old_var * ROTSPEED.sin() + data.plane_y * ROTSPEED.cos();
	}
	if is_key_down(W) {
		let diff_x = data.pos_x + data.dir_x * MOVSPEED;
		let diff_y = data.pos_y + data.dir_y * MOVSPEED;
		if map[diff_x as usize][data.pos_y as usize] == 0 {
			data.pos_x += data.dir_x * MOVSPEED;
		}
		if map[data.pos_x as usize][diff_y as usize] == 0 {
			data.pos_y += data.dir_y * MOVSPEED;
		}
	}
	if is_key_down(S) {
		let diff_x = data.pos_x - data.dir_x * MOVSPEED;
		let diff_y = data.pos_y - data.dir_y * MOVSPEED;
		if map[diff_x as usize][data.pos_y as usize] == 0 {
			data.pos_x -= data.dir_x * MOVSPEED;
		}
		if map[data.pos_x as usize][diff_y as usize] == 0 {
			data.pos_y -= data.dir_y * MOVSPEED;
		}
	}
	if is_key_down(A) {
		let diff_x = data.pos_x + data.dir_y * MOVSPEED;
		let diff_y = data.pos_y - data.dir_x * MOVSPEED;
		if map[diff_x as usize][data.pos_y as usize] == 0 {
			data.pos_x += data.dir_y * MOVSPEED;
		}
		if map[data.pos_x as usize][diff_y as usize] == 0 {
			data.pos_y -= data.dir_x * MOVSPEED;
		}
	}
	if is_key_down(D) {
		let diff_x = data.pos_x - data.dir_y * MOVSPEED;
		let diff_y = data.pos_y + data.dir_x * MOVSPEED;
		if map[diff_x as usize][data.pos_y as usize] == 0 {
			data.pos_x -= data.dir_y * MOVSPEED;
		}
		if map[data.pos_x as usize][diff_y as usize] == 0 {
			data.pos_y += data.dir_x * MOVSPEED;
		}
	}
	0
}