use macroquad::window::screen_height;
use crate::Tmath;
use crate::Ttex;
use crate::Tdata;
use macroquad::texture::Image;
use macroquad::shapes::draw_line;

fn calcul_text_coor(texture: &Image, math: &Tmath, wall_hit:f32 ) -> u32  {
	let mut result:u32;

	result = (wall_hit * texture.width() as f32) as u32;
	if math.side == 0 && math.ray_dirx > 0.0 {
		result = texture.width as u32 - result - 1;
	}
	if math.side == 1 && math.ray_diry < 0.0 {
		result = texture.width as u32 - result - 1;
	}
	return result;
}

fn draw_this_tex( data: &Tdata, math: &Tmath, texture: &Image) {
	let mut wall_hit: f32;
	if math.side == 1 {
		wall_hit = data.pos_x + math.perp_wall_dist * math.ray_dirx;
	} else {
		wall_hit = data.pos_y + math.perp_wall_dist * math.ray_diry;
	}
	wall_hit -= wall_hit.floor();
	let x: u32 = calcul_text_coor(&texture, &math, wall_hit);
	let step: f32 = (1.0 * texture.height as f32) / math.line_height as f32;
	let mut tex_pos: f32 = (math.draw_s as f32 - screen_height() / 2.0 + math.line_height as f32 / 2.0) * step;
	let mut y:u32 = math.draw_s - 1;
	let mut tex_y:u32;
	while y <= math.draw_e {
		tex_y = tex_pos as u32;
		//println!("tex_y = {}", tex_y);
		//println!("y = {}", y);
		tex_pos += step;
		if tex_y == texture.width as u32 {
			tex_y -= 1;
		}
		draw_line(math.curent_x as f32, y as f32, math.curent_x as f32, y as f32, 1.0, texture.get_pixel(x, tex_y));
		y += 1;
	}
}

pub fn draw_the_texture( data: &Tdata, math: &Tmath, texture:& Ttex) {
	if math.side == 1 {
		if math.ray_diry > 0.0 {
			draw_this_tex(data, math, &texture.tex_east);
			return ;
		}
		draw_this_tex(data, math, &texture.tex_west);
		return;
	}
	if math.ray_dirx > 0.0 {
		draw_this_tex(data ,math, &texture.tex_south);
		return ;
	}
	draw_this_tex(data, math, &texture.tex_north);
	return ;
}