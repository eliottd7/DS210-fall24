// A point in WBDGraph
#[derive(Debug, Clone)]
pub struct GeoPoint {
    id: usize,
    x: usize,
    y: usize,
    neighbors: Vec<usize>, // connections
    weights: Vec<usize>,   // weights of aligned connections
}

impl GeoPoint {
    pub fn new(id: usize, x: usize, y: usize) -> Self {
        GeoPoint {
            id: id,
            x: x,
            y: y,
            neighbors: Vec::<usize>::with_capacity(0),
            weights: Vec::<usize>::with_capacity(0),
        }
    }

    pub fn distance_to(&self, other: &GeoPoint) -> usize {
        let selfx: usize = self.x;
        let otherx: usize = other.x;
        let selfy: usize = self.y;
        let othery: usize = other.y;
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
        return ((xx + yy) as f64).sqrt().floor() as usize
    }

	pub fn neighbors(&mut self) -> &mut Vec<usize> {
        &mut self.neighbors
    }

    pub fn id(&mut self) -> &mut usize {
		&mut self.id
    }

    pub fn x(&mut self) -> &mut usize {
		&mut self.x
    }

    pub fn y(&mut self) -> &mut usize {
		&mut self.y
    }

    pub fn weights(&mut self) -> &mut Vec<usize> {
		&mut self.weights
    }
}
