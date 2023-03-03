use rand::Rng;
mod cell_struct;
use cell_struct::manage_cell::Cell;

fn main() {
    let maze_height = 10;
    let maze_width = 10;

    //Cells have a x and y coordenate and a boolean to check if they have been visited before
    let mut cells: Vec<Cell> = Vec::new();

    for i in 0..maze_height{
        for j in 0..maze_width {
            cells.push(cell_struct::manage_cell::Cell::new(i, j));
        }
    }

    //Setting the first Cell at 0,0 and making it occupied
    let mut visited = 1;

    let mut current: Vec<(u32,u32)> = Vec::new();
    current.push((0,0));
    cells[0].visit();
    

    while visited < maze_height * maze_width{

        //Closure that returns the place in the vector of a coordenate
        let coordenate = |x: u32, y: u32,north:bool, east: bool| -> usize  {
            if north {
                let calc = (current.last().unwrap().1 - y) * maze_width + (current.last().unwrap().0 + x);
                return calc as usize
            }
            if east {
                let calc = (current.last().unwrap().1 + y) * maze_width + (current.last().unwrap().0 - x);
                
                return calc as usize
            }
    
            let calc = (current.last().unwrap().1 + y) * maze_width + (current.last().unwrap().0 + x);
            
            calc as usize
        };

        let mut free_cells: Vec<u32> = Vec::new();

        print!("{},{}[{}]\n", current.last().unwrap().0, current.last().unwrap().1, visited);

        //Checks if it is possible for there to be a Cell in a certain direction
        //If there is a Cell there, check if it is visited
        //In case the Cell is free, save it in a vector
        if current.last().unwrap().1 > 0 && !cells[coordenate(0,1,true,false)].free() {
            free_cells.push(1);
        }
        if current.last().unwrap().1 < maze_height -1 && !cells[coordenate(0,1,false,false)].free() {
            free_cells.push(2);
        }
        if current.last().unwrap().0 > 0 && !cells[coordenate(1,0,false,true)].free() {
            free_cells.push(3);
        }
        if current.last().unwrap().0 < maze_width-1 && !cells[coordenate(1,0,false,false)].free() {
            free_cells.push(4);
        }

        // If there are free cells around, choose one randomly
        if free_cells.len() > 0 {
            let mut rng = rand::thread_rng();
            let chosen_cell = free_cells[rng.gen_range(0..free_cells.len())];

            // After selecting a cell, link them by setting True to the direction of the other one
            // Set the next cell as visited and push it as current
            match chosen_cell {
                1 => {
                    let position = coordenate(0,1,true,false);
                    cells[coordenate(0,0,false,false)].link_north();
                    cells[position].visit();
                    cells[position].link_south();
                    current.push((cells[position].get_x(),cells[position].get_y()));
                },
                2 => {
                    let position = coordenate(0,1,false,false);
                    cells[coordenate(0,0,false,false)].link_south();
                    cells[position].visit();
                    cells[position].link_north();
                    current.push((cells[position].get_x(),cells[position].get_y()));
                },
                3 => {
                    let position = coordenate(1,0,false,true);
                    cells[coordenate(0,0,false,false)].link_east();
                    cells[position].visit();
                    cells[position].link_west();
                    current.push((cells[position].get_x(),cells[position].get_y()));
                },
                4 => {
                    let position = coordenate(1,0,false,false);
                    cells[coordenate(0,0,false,false)].link_west();
                    cells[position].visit();
                    cells[position].link_east();
                    current.push((cells[position].get_x(),cells[position].get_y()));
                }
                _ => print!("error")
            }

            visited += 1;
            
        }
        else {
            current.pop();
        }

        
    }
    
}
