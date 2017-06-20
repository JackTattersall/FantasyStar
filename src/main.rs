#[derive(PartialEq, Debug)]
enum TerrainGround {
    Soil,
    Stone,
}

#[derive(PartialEq, Debug)]
enum TerrainBlock {
    Tree,
    Soil,
    Stone,
}

#[derive(PartialEq, Debug)]
enum Being {
    Orc,
    Human,
}

struct Square {
    ground: TerrainGround,
    block: Option<TerrainBlock>,
    being: Option<Being>,
}

struct Grid {
    size: (usize, usize),
    squares: Vec<Square>,
}



fn main() {
    println!("Hello, world!");
}
