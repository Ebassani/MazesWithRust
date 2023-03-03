pub mod manage_cell {
    pub struct Cell {
        y_axis: u32,
        x_axis: u32,
        visited: bool,
        north: bool,
        south: bool,
        east: bool,
        west: bool
    }

    impl Cell {
        pub fn new(y_axis: u32, x_axis: u32) -> Cell {
            Cell { y_axis, x_axis, visited: (false), north: (false), south: (false), east: (false), west: (false) }
        }

        pub fn get_x(&self) -> u32 { self.x_axis }
        pub fn get_y(&self) -> u32 { self.y_axis }
        pub fn free(&self) -> bool { self.visited }
        pub fn visit(&mut self){ self.visited = true; }
        pub fn link_north(&mut self){ self.north = true; }
        pub fn link_south(&mut self){ self.south = true; }
        pub fn link_east(&mut self){ self.east = true; }
        pub fn link_west(&mut self){ self.west = true; }
    }
}


