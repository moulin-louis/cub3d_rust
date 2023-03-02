mod camera;
mod calcul;
mod color;

use macroquad::prelude::*;
use macroquad::texture::Image;

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

#[derive(Default)]
pub struct Tdata {
	pos_x:f32,
	pos_y:f32,
	dir_x:f32,
	dir_y:f32,
	plane_x:f32,
	plane_y:f32,
}

pub struct Ttex {
	tex_north:Image,
	tex_south:Image,
	tex_west:Image,
	tex_east:Image,
}

fn conf() -> Conf {
	Conf {
		window_width:1920,
		window_height: 1080,
		window_title: String::from("CUB3D"),
		window_resizable:true,
		sample_count:32768,
		..Default::default()
	}
}
#[macroquad::main(conf)]
async fn main() {
	let map:Vec<Vec<i32>> = vec![vec![1, 1, 1, 1, 1], vec![1, 0, 0, 0, 1], vec![1, 0, 0, 0, 1], vec![1, 0, 0, 0, 1], vec![1, 1, 1, 1, 1]];
	for x in 0..map.len() {
		for y in 0..map[x].len() {
			print!("{}", map[x][y]);
		}
		println!("");
	}

	let mut data:Tdata = Tdata::default();
	data.pos_x = 1.0;
	data.pos_y = 1.0;
	data.dir_x = 1.0;
	data.dir_y = 0.0;
	data.plane_x = 0.0;
	data.plane_y = 0.8;
	let texture:Ttex = Ttex {
		tex_north: load_image("./tex_north.png").await.unwrap(),
		tex_south: load_image("./tex_south.png").await.unwrap(),
		tex_west: load_image("./tex_west.png").await.unwrap(),
		tex_east: load_image("./tex_east.png").await.unwrap(),
	};
	loop {
		if camera::handle_input(&mut data, &map) == 1 {
			break ;
		}
		for x in 0..(screen_width() as i32) {
			let mut math:Tmath = Tmath::default();
			calcul::calculate_init(&mut math, &data, x);
			calcul::calculate_step(&mut math, &data);
			calcul::perform_dda(&mut math, &map);
			calcul::calcul_draw(&mut math);
			math.curent_x = x;
			calcul::draw_the_line(&data,&math, &texture, x as f32);
		}
		let fps = get_fps();
		draw_text(fps.to_string().as_str(), 10.0, 10.0, 14.0, GREEN);
		next_frame().await;
	}
}