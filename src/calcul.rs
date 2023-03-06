use crate::{Tdata, Tmath, Ttex};
use crate::texture::draw_the_texture;
use macroquad::window::{screen_width, screen_height};
use macroquad::color::{BLUE, GRAY};

pub fn calculate_init(math:&mut Tmath, data:& Tdata, x:i32) {
	math.camera_x = 2.0 * (x as f32) / screen_width() - 1.0;
	math.ray_dirx = data.dir_x + data.plane_x * math.camera_x;
	math.ray_diry = data.dir_y + data.plane_y * math.camera_x;
	math.mapx = data.pos_x as i32;
	math.mapy = data.pos_y as i32;
	if math.ray_dirx == 0.0 {
		math.delta_distx = 1e30;
	}
	else {
		math.delta_distx = (1.0 / math.ray_dirx).abs();
	}
	if math.ray_diry == 0.0 {
		math.delta_disty = 1e30;
	}
	else {
		math.delta_disty = (1.0 / math.ray_diry).abs();
	}
}

pub fn calculate_step(math:&mut Tmath, data:& Tdata) {
	if math.ray_dirx < 0.0 {
		math.step_x = -1;
		math.side_distx = (data.pos_x - math.mapx as f32) * math.delta_distx;
	}
	else {
		math.step_x = 1;
		math.side_distx = (math.mapx as f32 + 1.0 - data.pos_x) * math.delta_distx;
	}
	if math.ray_diry < 0.0 {
		math.step_y = -1;
		math.side_disty = (data.pos_y - math.mapy as f32) * math.delta_disty;
	}
	else {
		math.step_y = 1;
		math.side_disty = (math.mapy as f32 + 1.0 - data.pos_y) * math.delta_disty;
	}
}

pub fn perform_dda(math:&mut Tmath, map: &[Vec<i32>] ) {
	//println!("mapx x = {} mapy = {}", math.mapx, math.mapy);
	while math.hit == 0 {
		if math.side_distx < math.side_disty {
			math.side_distx += math.delta_distx;
			math.mapx += math.step_x;
			math.side = 0;
		}
		else {
			math.side_disty += math.delta_disty;
			math.mapy += math.step_y;
			math.side = 1;
		}
		//println!("mapx x = {} mapy = {}", math.mapx, math.mapy);
		if map[math.mapx as usize][math.mapy as usize] == 1 {
			math.hit = 1;
		}
	}
}

pub fn calcul_draw(math:&mut Tmath) {
	if math.side == 0 {
		math.perp_wall_dist = math.side_distx - math.delta_distx;
	}
	else {
		math.perp_wall_dist = math.side_disty - math.delta_disty;
	}
	math.line_height = (screen_height() / math.perp_wall_dist) as i32;
	math.draw_s = ((-math.line_height / 2) as f32 + screen_height() / 2.0) as u32;
	if math.draw_s > screen_height() as u32 {
		math.draw_s = 0;
	}
	math.draw_e = (math.line_height / 2 + (screen_height() / 2.0) as i32) as u32;
	if math.draw_e >= screen_height()  as u32 {
		math.draw_e = screen_height() as u32 - 1;
	}
}

pub fn draw_the_line( data: &mut Tdata, math:& Tmath, texture:&Ttex, x:f32) {
	//draw ceiling
	for y in 0..math.draw_s {
		data.windows.set_pixel(x as u32, y, GRAY);
	}
	//draw wall
	// for y in math.draw_s..math.draw_e {
	// 	data.windows.set_pixel(x as u32, y, GREEN);
	// }
	draw_the_texture(data, math, texture);

	//draw floor
	if math.draw_e == (screen_height() - 1.0) as u32 { return ; }
	for y in math.draw_e..screen_height() as u32 {
		data.windows.set_pixel(x as u32, y, BLUE);
	}
}