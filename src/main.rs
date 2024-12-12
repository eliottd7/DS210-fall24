mod spatialgraph;
use spatialgraph::{WBDGraph, GeoPoint};
use std::env;

fn main() {
	if env::args().len() != 3 {
		panic!("Expected 2 args: path/to/twitch_large_features.csv path/to/twitch_large_edges.csv");
	}
	let argv : Vec<String> = env::args().collect();
	let graph = match WBDGraph::import_twitch_csv(argv[1].clone(), argv[2].clone()) {
		Ok(g) => g,
		Err(e) => {panic!("{e}")},
	};

	for i in 0..5 {
		println!("{:?}", &graph.points()[i]);
	}
}
