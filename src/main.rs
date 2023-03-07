mod camera;
mod calcul;
mod texture;

use macroquad::prelude::*;
use macroquad::texture::Image;
use macroquad::texture::Texture2D;

#[derive(Default)]
pub struct Tmath {
	camera_x:f32,
	ray_dirx:f32,
	ray_diry:f32,
	mapx:i32,
	mapy:i32,
	side_distx:f32,
	side_disty:f32,
	delta_distx:f32,
	delta_disty:f32,
	perp_wall_dist:f32,
	step_x:i32,
	step_y:i32,
	hit:i32,
	side:i32,
	line_height:i32,
	draw_s:u32,
	draw_e:u32,
	curent_x:i32,
}

pub struct Tdata {
	pos_x:f32,
	pos_y:f32,
	dir_x:f32,
	dir_y:f32,
	plane_x:f32,
	plane_y:f32,
	windows:Image,
	texture:Texture2D,
}

pub struct Ttex {
	tex_north:Image,
	tex_south:Image,
	tex_west:Image,
	tex_east:Image,
}

fn conf() -> Conf {
	Conf {
		window_width: 1920,
		window_height: 1080,
		window_title: String::from("CUB3D"),
		window_resizable: false,
		..Default::default()
	}
}
#[macroquad::main(conf)]
async fn main() {
	let map:Vec<Vec<i32>> = vec![vec![1, 1, 1, 1, 1], vec![1, 0, 0, 0, 1], vec![1, 0, 0, 0, 1], vec![1, 0, 0, 0, 1], vec![1, 1, 1, 1, 1]];
	let mut data:Tdata = Tdata {
		pos_x: 2.0,
		pos_y: 2.0,
		dir_x: -1.0,
		dir_y: 0.0,
		plane_x: 0.0,
		plane_y: 0.66,
		windows: Image::gen_image_color(screen_width() as u16, screen_height() as u16, BLACK),
		texture: Texture2D::empty(),
	};
	data.texture = Texture2D::from_image(&data.windows);
	let texture:Ttex = Ttex {
		tex_north: load_image("./texture/tex_north.png").await.unwrap(),
		tex_south: load_image("./texture/tex_south.png").await.unwrap(),
		tex_west: load_image("./texture/tex_west.png").await.unwrap(),
		tex_east: load_image("./texture/tex_east.png").await.unwrap(),
	};
	loop {
		if camera::handle_input(&mut data, &map) == 1 { break; };

		let mut old_time = get_time();
		for x in 0..(screen_width() as i32) {
			let mut math:Tmath = Tmath { curent_x:x, ..Default::default() };
			math.curent_x = x;
			calcul::calculate_init(&mut math, &data, x);
			calcul::calculate_step(&mut math, &data);
			calcul::perform_dda(&mut math, &map);
			calcul::calcul_draw(&mut math);
			calcul::draw_the_line(&mut data,&math, &texture, x as f32);
		}
		//print!("dda\t= {}\n", get_time() - old_time);
		old_time = get_time();
		data.texture.update(&data.windows);
		//draw_texture(data.texture, 0.0, 0.0, WHITE);
		//print!("drawing\t= {}\n", get_time() - old_time);
		draw_text(get_fps().to_string().as_str(), 10.0, 10.0, 14.0, GREEN);
		next_frame().await;
	}
}
