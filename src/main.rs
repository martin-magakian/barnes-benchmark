extern crate barnes;

use std::env;
use barnes::{Point, Tree, Square};


extern crate time;
use time::SteadyTime;

extern crate rand;
use rand::distributions::{IndependentSample, Range};

fn random_point(num_point: i64, max: i64) -> Vec<Point> {
	let mut rng = rand::thread_rng();
	let between = Range::new(0i64, max);
	
	(0..num_point).map(|_| {
						Point::new(between.ind_sample(&mut rng), between.ind_sample(&mut rng), "hey")
					}).collect()
}

fn run_benchmark(num_point: i64) {
	
	let grid_size = num_point*100;
	println!("[mono-thread] generate {} random point", num_point);
	let rand_points = random_point(num_point, grid_size);
	
	let mut square = Square::new(0, 0, grid_size);
	square.weight = rand_points.len() as i64;
	
	println!("[multi-threaded] - start Barnes Hut");
	let start = SteadyTime::now();
	Tree.compute_root(square, rand_points);
	let end = SteadyTime::now();
	println!("finish Barnes Hut. total time: {}", end-start);
}

fn main() {
	let mut bucket_size = 5_000_000;
	
	if let Some(arg1) = env::args().nth(1) {
		bucket_size = match arg1.parse::<i64>() {
		  Ok(n) => n,
		  Err(_) => 5_000_000
		};
	}

	
	run_benchmark(bucket_size);
}