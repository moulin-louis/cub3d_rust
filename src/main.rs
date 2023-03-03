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

pub struct Tdata {
	pos_x:f32,
	pos_y:f32,
	dir_x:f32,
	dir_y:f32,
	plane_x:f32,
	plane_y:f32,
	windows:Image,
}

pub struct Ttex {
	tex_north:Image,
	tex_south:Image,
	tex_west:Image,
	tex_east:Image,
}

fn conf() -> Conf {
	Conf {
		window_width:1280,
		window_height: 720,
		window_title: String::from("CUB3D"),
		window_resizable:true,
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
		println!();
	}

	let mut data:Tdata = Tdata {
		pos_x: 2.0,
		pos_y: 2.0,
		dir_x: -1.0,
		dir_y: 0.0,
		plane_x: 0.0,
		plane_y: 0.66,
		windows: Image::gen_image_color(screen_width() as u16, screen_height() as u16, BLACK),
	};
	let texture:Ttex = Ttex {
		tex_north: load_image("./texture/tex_north.png").await.unwrap(),
		tex_south: load_image("./texture/tex_south.png").await.unwrap(),
		tex_west: load_image("./texture/tex_west.png").await.unwrap(),
		tex_east: load_image("./texture/tex_east.png").await.unwrap(),
	};
	loop {
		for x in 0..data.windows.width {
			for y in 0..data.windows.height {
				data.windows.set_pixel(x as u32,y as u32, WHITE);
			}
		}
		camera::handle_input(&mut data, &map);
		for x in 0..(screen_width() as i32) {
			let mut math:Tmath = Tmath { curent_x:x, ..Default::default() };
			math.curent_x = x;
			calcul::calculate_init(&mut math, &data, x);
			calcul::calculate_step(&mut math, &data);
			calcul::perform_dda(&mut math, &map);
			calcul::calcul_draw(&mut math);
			calcul::draw_the_line(&mut data,&math, &texture, x as f32);
		}
		let image = Texture2D::from_image(&data.windows);
		draw_texture(image, 0.0, 0.0, WHITE);
		let fps = get_fps();
		draw_text(fps.to_string().as_str(), 10.0, 10.0, 14.0, GREEN);
		next_frame().await;
	}
}