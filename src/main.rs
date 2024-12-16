mod spatialgraph;
use spatialgraph::WBDGraph;
use std::env;
use std::thread;

fn main() {
    let mut nodes_csv_path = String::from("./twitch_gamers/large_twitch_features.csv");
    let mut edges_csv_path = String::from("./twitch_gamers/large_twitch_edges.csv");
    let mut max_dfs_depth = 3000usize;
    if env::args().len() != 4 {
        println!("Expected 3 args: path/to/twitch_large_features.csv path/to/twitch_large_edges.csv max_dfs_depth.\n(Using defaults ./twitch_gamers/twitch_large_features.csv ./twitch_gamers/twitch_large_edges.csv 3000)");
    } else {
        let argv: Vec<String> = env::args().collect();
        nodes_csv_path = String::from(argv[1].clone());
        edges_csv_path = String::from(argv[2].clone());
        max_dfs_depth = String::from(argv[3].clone())
            .parse::<usize>()
            .unwrap_or(3000);
    }

    let mut graph =
        match WBDGraph::import_twitch_csv(nodes_csv_path.clone(), edges_csv_path.clone()) {
            Ok(g) => g,
            Err(e) => {
                panic!("{e}")
            }
        };

    assert_eq!(graph.points().len(), 168114);

    let node_with_min_neighbors = graph.dfs_maxdepth_minneighbors(49usize, 3000, 0usize);
    let visits = graph.check_visits();
    println!(
        "From node 49, the node with the least neighbors within 3000 is node {} with {} neighbors",
        node_with_min_neighbors.clone(),
        graph.points().clone()[node_with_min_neighbors.clone()]
            .neighbors()
            .len()
    );
    println!("(Visited {} nodes to confirm this)", visits);
    graph.clear_visits();

    println!("Dumping min neighbors to working directory");
    let mut wtr = csv::WriterBuilder::new()
        .from_path("./minimum_neighbors_map.csv")
        .unwrap();
    let _ = wtr.write_record(["origin", "min_neighbors_node", "count"]);

	let progress_max = graph.points().len();
    for ix in 0..graph.points().len() {
    	if ix % 100 == 0 {
			println!("Progress {ix} / {progress_max}");
    	}
        let mut _mnn: usize = usize::MAX;
        let mut borrow_graph = graph.clone();
        thread::spawn(move || {
            _mnn = borrow_graph.dfs_maxdepth_minneighbors(ix, max_dfs_depth.clone(), 0usize);
        });
        let _ = wtr.write_record([
            ix.clone().to_string(),
            _mnn.clone().to_string(),
            _mnn.to_string(),
        ]);
    }
}
