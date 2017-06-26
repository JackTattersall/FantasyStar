#[derive(PartialEq, Debug, Clone)]
#[allow(dead_code)]
enum TerrainGround {
    Soil,
    Stone,
}

#[derive(PartialEq, Debug, Clone)]
#[allow(dead_code)]
enum TerrainBlock {
    Tree,
    Soil,
    Stone,
}

#[derive(PartialEq, Debug, Clone)]
#[allow(dead_code)]
enum Being {
    Orc,
    Human,
}

#[derive(PartialEq, Debug, Clone)]
#[allow(dead_code)]
enum Direction {
    West,
    East,
    North,
    South,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
enum MovementError {
    NoBeingInSquare,
    SquareOutsideGridBoundary,
    BeingAlreadyInSquare,
    TerrainGroundUnsuitable,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
struct Square {
    ground: TerrainGround,
    block: Option<TerrainBlock>,
    beings: Option<Being>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
struct Grid {
    size: (usize, usize),
    squares: Vec<Square>,
}
#[allow(dead_code)]
impl Grid {

    fn move_being_in_coord(&self, coord: (usize, usize), direction: Direction) -> Result<(usize, usize), MovementError> {
        let square = self.squares.get(coord.0 * self.size.0 + coord.1).expect("Index out of bounds trying to get being");
        match square.beings {
            Some(_) => {
                let mut coord_to_move_into: (usize, usize) = coord;
                match direction {
                    ::Direction::West => coord_to_move_into.1 -= 1,
                    ::Direction::East => coord_to_move_into.1 += 1,
                    ::Direction::South => coord_to_move_into.0 -= 1,
                    ::Direction::North => coord_to_move_into.0 += 1,
                };

                if (coord_to_move_into.0 > (self.size.0 - 1)) |
                     (coord_to_move_into.1 > (self.size.1 - 1)) {
                        return Err(MovementError::SquareOutsideGridBoundary)
                };

                let square_moving_into: &Square = self.squares.get(coord_to_move_into.0 * self.size.0 + coord_to_move_into.1)
                    .expect("Index out of bounds trying to get square_moving_into");

                if square_moving_into.beings.is_some() { return Err(MovementError::BeingAlreadyInSquare)};

                match square_moving_into.ground {
                    ::TerrainGround::Soil => return Ok(coord_to_move_into),
                    ::TerrainGround::Stone => return Err(MovementError::TerrainGroundUnsuitable)
                };
            }
            None => Err(MovementError::NoBeingInSquare)
        }
    }

    fn generate_empty(size_x: usize, size_y: usize) -> Grid {
        let number_of_squares = size_x * size_y;
        let mut squares: Vec<Square> = Vec::with_capacity(number_of_squares);

        for _ in 0..number_of_squares {
            squares.push(Square {
                ground: TerrainGround::Soil,
                block: None,
                beings: None,
            });
        }

        Grid {
            size: (size_x, size_y),
            squares: squares
        }
    }
}


// ------------------TESTS-------------------------

// Grid tests

#[cfg(test)]
mod test {
    #[test]
    fn test_empty_grid() {
        let grid = ::Grid::generate_empty(5, 13);
        assert_eq!(grid.size, (5, 13));
        let mut number_of_squares = 0;

        for square in &grid.squares {
            assert_eq!(square.ground, ::TerrainGround::Soil);
            assert_eq!(square.block, None);
            assert_eq!(square.beings, None);
            number_of_squares += 1;
        }

        assert_eq!(grid.squares.len(), 5 * 13);
        assert_eq!(number_of_squares, 5 * 13);
    }

    #[test]
    fn try_moving_from_a_square_without_a_being() {
        let grid = ::Grid::generate_empty(3, 3);
        assert_eq!(
            grid.move_being_in_coord((0, 0), ::Direction::West),
            Err(::MovementError::NoBeingInSquare)
        )
    }

    #[test]
    fn try_moving_a_being_outside_grid() {
        let mut grid = ::Grid::generate_empty(3, 3);
        let test_being = ::Being::Orc;
        let test_terrain_ground = ::TerrainGround::Soil;
        let test_square = ::Square{ ground: test_terrain_ground, block: None, beings: Some(test_being)};
        grid.squares.insert(8, test_square);

        // Test moving east
        assert_eq!(
            grid.move_being_in_coord((2, 2), ::Direction::East),
            Err(::MovementError::SquareOutsideGridBoundary)
        );

        // Test moving north
        assert_eq!(
            grid.move_being_in_coord((2, 2), ::Direction::North),
            Err(::MovementError::SquareOutsideGridBoundary)
        );

        // south and west not tested as usize minimum value is 0
    }

    #[test]
    fn being_already_exists_in_square_trying_to_move_into() {
        let mut grid = ::Grid::generate_empty(3, 3);
        let test_being = ::Being::Orc;
        let test_terrain_ground = ::TerrainGround::Soil;
        let test_square = ::Square{ ground: test_terrain_ground, block: None, beings: Some(test_being)};

        grid.squares.insert(7, test_square.clone());
        grid.squares.insert(8, test_square.clone());

        assert_eq!(
            grid.move_being_in_coord((2, 1), ::Direction::East),
            Err(::MovementError::BeingAlreadyInSquare)
        );
    }

    #[test]
    fn try_to_move_being_to_stone_ground() {
        let mut grid = ::Grid::generate_empty(3, 3);
        let test_being = ::Being::Orc;
        let test_terrain_ground = ::TerrainGround::Soil;
        let test_square = ::Square{ ground: test_terrain_ground, block: None, beings: Some(test_being)};

        grid.squares.insert(7, test_square);
        grid.squares.insert(8, ::Square{ ground: ::TerrainGround::Stone, block: None, beings: None});

        assert_eq!(
            grid.move_being_in_coord((2, 1), ::Direction::East),
            Err(::MovementError::TerrainGroundUnsuitable)
        );
    }
}


fn main() {
    println!("Hello, world!");
}
