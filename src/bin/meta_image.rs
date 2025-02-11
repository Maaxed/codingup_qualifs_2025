use codingup_qualifs::io::{arg_file_name, read_input, read_output};
use image::{Rgb, RgbImage};
use line_drawing::Bresenham;


fn main()
{
	let input = read_input().unwrap();

	let mut max_pos = [0_u32; 2];

	for l in [&input.plants, &input.seeds]
	{
		for pos in l
		{
			if pos[0] as u32 > max_pos[0]
			{
				max_pos[0] = pos[0] as u32;
			}

			if pos[1] as u32 > max_pos[1]
			{
				max_pos[1] = pos[1] as u32;
			}
		}
	}

	let mut img = RgbImage::new(max_pos[0] + 1, max_pos[1] + 1);

	// Draw meta
	let actions = read_output();

	let mut pos = [0; 2];

	for action in actions
	{
		for (x, y) in Bresenham::new((pos[0], pos[1]), (action.pos[0], action.pos[1]))
		{
			img.put_pixel(x as u32, max_pos[1] - y as u32, Rgb::from([255, 0, 0]));
			pos = action.pos;
		}
	}

	for plant in input.plants
	{
		let p = img.get_pixel(plant[0] as u32, plant[1] as u32);
		img.put_pixel(plant[0] as u32, max_pos[1] - plant[1] as u32, Rgb::from([p.0[0], 255, 0]));
	}

	for seed in input.seeds
	{
		let p = img.get_pixel(seed[0] as u32, seed[1] as u32);
		img.put_pixel(seed[0] as u32, max_pos[1] - seed[1] as u32, Rgb::from([p.0[0], p.0[1], 255]));
	}

	let file_name = arg_file_name();
	let exe_name = std::env::args().nth(2).unwrap();
	img.save(format!("output/{file_name}_{exe_name}.png")).unwrap();
}