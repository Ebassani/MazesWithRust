use rand::Rng;


#[derive(Default)]
struct Cell {
    y_axis: u32,
    x_axis: u32,
    visited: bool,
    north:bool,
    south: bool,
    east:bool,
    west:bool
}

fn main() {
    let maze_height = 10;
    let maze_width = 10;

    //Cells have a x and y coordenate and a boolean to check if they have been visited before
    let mut cells: Vec<Cell> = Vec::new();

    for i in 0..maze_height{
        for j in 0..maze_width {
            cells.push(Cell { y_axis: i, x_axis: j, ..Default::default() });
        }
    }

    //Setting the first Cell at 0,0 and making it occupied
    let mut visited = 1;

    let mut current: Vec<(u32,u32)> = Vec::new();
    current.push((0,0));
    cells[0].visited = true;
    

    while visited < maze_height * maze_width{

        //Closure that return the place in the vector of a coordenate
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
            //print!("!{}!",calc);
            
            calc as usize
        };

        let mut free_cells: Vec<u32> = Vec::new();

        print!("{},{}[{}]\n", current.last().unwrap().0, current.last().unwrap().1, visited);

        //Checks if it is possible for there to be a Cell in a certain direction
        //If there is a Cell there, check if it is visited
        //In case the Cell is free, save it in a vector
        if current.last().unwrap().1 > 0 && !cells[coordenate(0,1,true,false)].visited {
            //print!("N[{},{}]",cells[coordenate(0,1,true,false)].0,cells[coordenate(0,1,true,false)].1);
            free_cells.push(1);
        }
        if current.last().unwrap().1 < maze_height -1 && !cells[coordenate(0,1,false,false)].visited {
            //print!("S[{},{}]",cells[coordenate(0,1,false,false)].0,cells[coordenate(0,1,false,false)].1);
            free_cells.push(2);
        }
        if current.last().unwrap().0 > 0 && !cells[coordenate(1,0,false,true)].visited {
            //print!("E[{},{}]",cells[coordenate(1,0,false,true)].0,cells[coordenate(1,0,false,true)].1);
            free_cells.push(3);
        }
        if current.last().unwrap().0 < maze_width-1 && !cells[coordenate(1,0,false,false)].visited {
            //print!("W[{},{}]\n",cells[coordenate(1,0,false,false)].0,cells[coordenate(1,0,false,false)].1);
            free_cells.push(4);
        }

        if free_cells.len() > 0 {
            let mut rng = rand::thread_rng();
            let chosen_cell = free_cells[rng.gen_range(0..free_cells.len())];

            match chosen_cell {
                1 => {
                    let position = coordenate(0,1,true,false);
                    cells[position].visited = true;
                    current.push((cells[position].x_axis,cells[position].y_axis));
                },
                2 => {
                    let position = coordenate(0,1,false,false);
                    cells[position].visited = true;
                    current.push((cells[position].x_axis,cells[position].y_axis));
                },
                3 => {
                    let position = coordenate(1,0,false,true);
                    cells[position].visited = true;
                    current.push((cells[position].x_axis,cells[position].y_axis));
                },
                4 => {
                    let position = coordenate(1,0,false,false);
                    cells[position].visited = true;
                    current.push((cells[position].x_axis,cells[position].y_axis));
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
