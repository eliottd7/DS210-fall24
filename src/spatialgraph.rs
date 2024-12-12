use std::error::Error;
use std::fs;

// A point in WBDGraph
#[derive(Debug)]
pub struct GeoPoint {
    id: u32,
    x: u32,
    y: u32,
    neighbors: Vec<u32>, // connections
    weights: Vec<u32>, // weights of aligned connections
}

// A weighted bi-directional graph
pub struct WBDGraph {
    points: Vec<GeoPoint>, // All the points
}

// Specialized tuples typed for csv serde to unpack the large_twitch_features.csv
type TwitchItem = (
    u64,    // .0 views
    bool,   // .1 mature
    u64,    // .2 life_time
    String, // .3 created_at
    String, // .4 updated_at
    u64,    // .5 numeric_id
    bool,   // .6 dead_account
    String, // .7 language
    bool,   // .8 affiliate
);

impl GeoPoint {
    pub fn new(id: u64, x: u64, y: u64) -> Self {
        GeoPoint {
            id: id,
            x: x,
            y: y,
            neighbors: Vec::<u64>::with_capacity(0),
        }
    }

    pub fn add_neighbor(&mut self, neighbor: u64) {
        self.neighbors.push(neighbor);
    }

    pub fn distance_to(&self, other: &GeoPoint) -> u64 {
        let xx = if self.x > other.x {
        	(self.x - other.x) * (self.x - other.x)
    	} else {
			(other.x - self.x) * (other.x - self.x)
    	};
        let yy = if self.y > other.y {
        	(self.y - other.y) * (self.y - other.y)
        } else {
			(other.y - self.y) * (other.y - self.y)
        };
        ((xx + yy) as f64).sqrt().floor() as u64
    }
}

impl WBDGraph {
    pub fn new() -> Self {
        WBDGraph {
            points: Vec::<GeoPoint>::new(),
        }
    }

    pub fn import_twitch_csv(node_path: String, edge_path: String) -> Result<Self, Box<dyn Error>> {

        let mut newgraph = WBDGraph::new();

        // Read all points into the points array
        println!("Loading {node_path}");
        let mut rdr = csv::ReaderBuilder::new()
            .ascii()
            .terminator(csv::Terminator::CRLF)
            .has_headers(false)
            .from_path(node_path)?;
		for result in rdr.records().skip(1) {
			let record = result?;
			let line = record.get(0).unwrap().split(',').collect::<Vec<&str>>();
			let id : u64 = line[5].parse()?;
			if (id as usize) != newgraph.points.len() {
				panic!("CSV records are misaligned");
			}
			let x : u64 = line[0].parse()?;
			let y : u64 = line[2].parse()?;
			newgraph.points.push(GeoPoint::new(id, x, y));			
		}

        // Pre-allocate, then populate the weights grid
        println!("Loading {edge_path}");
        let mut rdr = csv::ReaderBuilder::new()
            .ascii()
            .terminator(csv::Terminator::CRLF)
            .has_headers(false)
            .from_path(edge_path)?;
        let num_points = newgraph.points.len();
        for result in rdr.records() {
        	let record = result?;
			let line = record.get(0).unwrap().split(',').collect::<Vec<&str>>();
			let src : usize = line[0].parse()?;
			let dst : usize = line[1].parse()?;
            let weight = newgraph.points[src].distance_to(&newgraph.points[dst]);
            newgraph.points[src].neighbors.push(dst.try_into().unwrap());
            newgraph.points[dst].neighbors.push(src.try_into().unwrap());
        }

        // Return
        Ok(newgraph)
    }

    pub fn points(&self) -> &Vec<GeoPoint> {
		&self.points
    }
}
