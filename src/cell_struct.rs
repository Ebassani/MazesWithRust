pub mod manage_cell {
    ///  # Cell
    /// Cells have their x and y coordenates saved and different cardinal directions, 
    /// if the direction is set to false, that means the cell is not linked to another in that direction
    /// ## Fields
    /// * y_axis : The position of the cell in the y axis of the grid
    /// * x_axis : The position of the cell in the x axis of the grid
    /// * visited : True if the cell has been visited before, false if not
    /// * north : When this Cell links itself with one in the north direction this field is set to True
    /// * south : When this Cell links itself with one in the south direction this field is set to True
    /// * east : When this Cell links itself with one in the east direction this field is set to True
    /// * west : When this Cell links itself with one in the west direction this field is set to True
    pub struct Cell {
        y_axis: i32,
        x_axis: i32,
        visited: bool,
        north: bool,
        south: bool,
        east: bool,
        west: bool
    }

    impl Cell {
        /// # Create a default Cell
        /// new() is a function that returs a Cell with x and y defined, but all the other fields
        /// are set as default, in this case, False
        /// ## Arguments
        /// * y_axis : the position of the cell in the y axis of the grid
        /// * x_axis : the position of the cell in the x axis of the grid
        /// 
        pub fn new(y_axis: i32, x_axis: i32) -> Cell {
            Cell { y_axis, x_axis, visited: (false), north: (false), south: (false), east: (false), west: (false) }
        }

        pub fn get_x(&self) -> i32 { self.x_axis }
        pub fn get_y(&self) -> i32 { self.y_axis }
        pub fn free(&self) -> bool { self.visited }
        pub fn visit(&mut self){ self.visited = true; }
        pub fn link_north(&mut self){ self.north = true; }
        pub fn link_south(&mut self){ self.south = true; }
        pub fn link_east(&mut self){ self.east = true; }
        pub fn link_west(&mut self){ self.west = true; }
        pub fn get_w(&self) -> bool { self.west }
        pub fn get_e(&self) -> bool { self.east }
        pub fn get_s(&self) -> bool { self.south }
        pub fn get_n(&self) -> bool { self.north }
        pub fn clean_cell(&mut self) {self.visited = false}
    }
}


