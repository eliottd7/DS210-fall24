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
    u32,    // .0 views
    bool,   // .1 mature
    u32,    // .2 life_time
    String, // .3 created_at
    String, // .4 updated_at
    u32,    // .5 numeric_id
    bool,   // .6 dead_account
    String, // .7 language
    bool,   // .8 affiliate
);

impl GeoPoint {
    pub fn new(id: u32, x: u32, y: u32) -> Self {
        GeoPoint {
            id: id,
            x: x,
            y: y,
            neighbors: Vec::<u32>::with_capacity(0),
            weights: Vec::<u32>::with_capacity(0),
        }
    }

    pub fn distance_to(&self, other: &GeoPoint) -> u32 {
    	let selfx : u64 = self.x as u64;
    	let otherx : u64 = other.x as u64;
    	let selfy : u64 = self.y as u64;
    	let othery : u64 = other.y as u64;
        let xx = if selfx > otherx {
        	(selfx - otherx) * (selfx - otherx)
    	} else {
			(otherx - selfx) * (otherx - selfx)
    	};
        let yy = if selfy > othery {
        	(selfy - othery) * (selfy - othery)
        } else {
			(othery - selfy) * (othery - selfy)
        };
        let overmax = ((xx + yy) as f64).sqrt().floor() as u64;
        if overmax > u32::MAX.into() {
			return u32::MAX
        }
        else {
			overmax as u32
        }
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
			let id : u32 = line[5].parse()?;
			if (id as usize) != newgraph.points.len() {
				panic!("CSV records are misaligned");
			}
			let x : u32 = line[0].parse()?;
			let y : u32 = line[2].parse()?;
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
            newgraph.points[src].weights.push(weight.clone());
            newgraph.points[dst].weights.push(weight);
        }

        // Return
        Ok(newgraph)
    }

    pub fn points(&self) -> &Vec<GeoPoint> {
		&self.points
    }
}
