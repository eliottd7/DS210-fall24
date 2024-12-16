mod geopoint;
use geopoint::GeoPoint;
use std::error::Error;
use std::collections::HashSet;

// A weighted bi-directional graph
#[derive(Clone)]
pub struct WBDGraph {
    points: Vec<GeoPoint>, // All the points
	visited: HashSet<usize>,
}

impl WBDGraph {
    pub fn new() -> Self {
        WBDGraph {
            points: Vec::<GeoPoint>::new(),
            visited: HashSet::<usize>::new(),
        }
    }

    pub fn import_twitch_csv(node_path: String, edge_path: String) -> Result<Self, Box<dyn Error>> {
        let mut newgraph = WBDGraph::new();

        // Read all points into the points array
        println!("Loading nodes from {node_path}");
        let mut rdr = csv::ReaderBuilder::new()
            .ascii()
            .terminator(csv::Terminator::CRLF)
            .has_headers(false)
            .from_path(node_path)?;
        for result in rdr.records().skip(1) {
            let record = result?;
            let line = record.get(0).unwrap().split(',').collect::<Vec<&str>>();
            let id: usize = line[5].parse()?;
            if id != newgraph.points.len() {
                panic!("CSV records are misaligned");
            }
            let x: usize = line[0].parse()?;
            let y: usize = line[2].parse()?;
            newgraph.points.push(GeoPoint::new(id, x, y));
        }

        // Pre-allocate, then populate the weights grid
        println!("Loading edges from {edge_path}");
        let mut rdr = csv::ReaderBuilder::new()
            .ascii()
            .terminator(csv::Terminator::CRLF)
            .has_headers(false)
            .from_path(edge_path)?;
        for result in rdr.records() {
            let record = result?;
            let line = record.get(0).unwrap().split(',').collect::<Vec<&str>>();
            let src: usize = line[0].parse()?;
            let dst: usize = line[1].parse()?;
            let weight = newgraph.points[src].distance_to(&newgraph.points[dst]);
            newgraph.points[src].neighbors().push(dst.try_into().unwrap());
            newgraph.points[dst].neighbors().push(src.try_into().unwrap());
            newgraph.points[src].weights().push(weight.clone());
            newgraph.points[dst].weights().push(weight);
        }

        // Return
        Ok(newgraph)
    }

    pub fn points(&self) -> &Vec<GeoPoint> {
        &self.points
    }

	// return the index of the node with the least neighbors
    pub fn dfs_maxdepth_minneighbors(
        &mut self,
        node: usize,
        maxdepth: usize,
        currdepth: usize,
    ) -> usize {
    
        let _ = self.visited.insert(node);
		let mut point = self.points[node].clone();

        // if the current node has no neighbors, it is the min!
		let neighbors = point.neighbors().clone();
        if neighbors.is_empty() {
            return node;
        }

		// dfs check on each neighbor
		let mut current_min = neighbors.len();
		let mut current_min_node = node;
		for neigh_ix in 0..neighbors.len() {
			let neighbor = neighbors[neigh_ix];

			// do not exceed the maximum depth
			let newdepth = currdepth + point.weights()[neigh_ix];
			if newdepth > maxdepth {
				continue;
			}

			// dfs call
			let neighbor_with_min = self.dfs_maxdepth_minneighbors(
				neighbor.clone(),
				maxdepth,
				newdepth,
			);

			// compare to the current
			if self.points[neighbor_with_min].neighbors().len() < current_min {
				current_min = self.points[neighbor_with_min].neighbors().len();
				current_min_node = neighbor_with_min;
			}
		}

		return current_min_node
    }

    // Returns a list of nodes visited
    pub fn check_visits(&self) -> usize {
		self.visited.len()
    }

    pub fn clear_visits(&mut self) {
        self.visited.clear();
    }
}
