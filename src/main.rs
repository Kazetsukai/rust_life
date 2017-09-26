extern crate rand;
extern crate png_encode_mini;

use rand::{thread_rng, Rng};
use std::mem::swap;
use std::{thread, time};
use std::iter::repeat;
use png_encode_mini::write_rgba_from_u8;

const NEIGHBOURS: [(i32, i32); 8] = [
		(-1, -1), (0, -1), (1, -1),
		(-1,  0),          (1,  0),
		(-1,  1), (0,  1), (1,  1)
	];

fn main() {

	let width = 1920;
	let height = 1080;
	let size = (width*height) as usize;

	let mut img = repeat(0).take(size * 4).collect();

	let mut grid = Grid {
		width: width,
		height: height,
		gen: 0,
		data: vec![false; size]
	};
	let mut oldGrid = Grid {
		width: width,
		height: height,
		gen: 0,
		data: vec![false; size]
	};

	initRand(&mut grid);

	let delay = time::Duration::from_millis(10);

	for i in 0..2000 {

    	let mut f = std::fs::File::create(format!("life-{}.png", i)).unwrap();

		//for j in 0..30 {
			swap(&mut grid, &mut oldGrid);
			stepLife(&mut grid, &mut oldGrid);
		//}

		render(&grid, &mut img);

        match write_rgba_from_u8(&mut f, &img, width as u32, height as u32) {
            Ok(_) => println!("wrote image"),
            Err(reason) => println!("failed to write output png: {}", reason)
        }

		//display(&grid);
	}
}

fn stepLife(grid: &mut Grid, oldGrid: &mut Grid) {
	let mut i = 0;
	for y in 0..grid.height {
		for x in 0..grid.width {
			let live = oldGrid.data[i];
			let n = neighbours(&oldGrid, x, y);
			
			grid.data[i] = match n {
				2 if live => true,
				3 => true,
				_ => false
			};

			i += 1;
		}
	}
}

fn neighbours(grid: &Grid, x: i32, y: i32) -> i32 {
	let mut count = 0;

	for &(tx, ty) in NEIGHBOURS.iter() {
		if x+tx < 0 || x+tx >= grid.width || y+ty < 0 || y+ty >= grid.height { continue; }

		let index = x + tx + (y + ty) * grid.width;

		if grid.data[index as usize] {
			count += 1;
		}
	}

	count
}

fn initRand(grid: &mut Grid) {
    let mut rng = rand::thread_rng();

	let mut i = 0;
	for y in 0..grid.height {
		for x in 0..grid.width {
			grid.data[i] = rng.gen_weighted_bool(3);
			i += 1;
		}
	}
}

fn display(grid: &Grid) {
    print!("{}[2J", 27 as char);

	let mut i = 0;
	for y in 0..grid.height {
		for x in 0..grid.width {
			if grid.data[i] { print!("#") } else { print!(".") };
			i += 1;
		}
		println!();
	}

	println!("{:?}", grid.gen);
	println!();
}

fn render(grid: &Grid, mut img: &mut Vec<u8>) {
    let buffer = (&mut img).chunks_mut(4);
    for (px, cell) in buffer.zip(&grid.data) {
    	if *cell {
    		px[0] = 255;
    		px[1] = 255;
    		px[2] = 255;
    		px[3] = 255;
    	} else {
    		px[0] = 0;
    		px[1] = 0;
    		px[2] = 0;
    		px[3] = 255;
    	}
    }
}

struct Grid {
	width: i32,
	height: i32,
	gen: i32,
	data: Vec<bool>
}