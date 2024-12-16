mod spatialgraph;
use spatialgraph::WBDGraph;
use std::env;
use std::thread;
use rand::prelude::*;

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
    let graph_max = graph.points().len();
    assert_eq!(graph_max, 168114);

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
    assert_eq!(node_with_min_neighbors, 25309);
    graph.clear_visits();
    assert_eq!(graph.check_visits(), 0);

    println!("Dumping 10k random min neighbors to working directory");
    let mut wtr = csv::WriterBuilder::new()
        .from_path("./mnm.csv")
        .unwrap();
    match wtr.write_record(["origin_node", "min_neighbors_node", "count"]) {
		Ok(_) => (),
		Err(e) => {panic!("{}", e)},
    }
    for _ in 0..10000 {
		let ix : usize = rand::random::<usize>() % graph_max;
        let mut mnn: usize = usize::MAX;
        let mut borrow_graph = graph.clone();
        mnn = borrow_graph.dfs_maxdepth_minneighbors(ix, max_dfs_depth.clone(), 0usize);
        let svc = vec![
            ix.clone().to_string(),
            mnn.clone().to_string(),
			graph.points().clone()[mnn].neighbors().len().to_string(),
        ];
        let _ = wtr.write_record(svc);
        graph.clear_visits();
    }
}
