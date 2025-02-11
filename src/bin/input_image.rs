use codingup_qualifs::io::{arg_file_name, read_input};
use image::{Rgb, RgbImage};


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

	for plant in input.plants
	{
		img.put_pixel(plant[0] as u32, max_pos[1] - plant[1] as u32, Rgb::from([0, 255, 0]));
	}

	for seed in input.seeds
	{
		let p = img.get_pixel(seed[0] as u32, seed[1] as u32);
		img.put_pixel(seed[0] as u32, max_pos[1] - seed[1] as u32, Rgb::from([0, p.0[1], 255]));
	}

	let file_name = arg_file_name();
	img.save(format!("input/{file_name}.png")).unwrap();
}