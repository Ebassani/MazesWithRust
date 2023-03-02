use rand::Rng;

fn main() {
    let maze_height = 10;
    let maze_width = 10;

    //Cells have a x and y coordenate and a boolean to check if they have been visited before
    let mut cells: Vec<(u32,u32,bool)> = Vec::new();

    for i in 0..maze_height{
        for j in 0..maze_width {
            cells.push((j,i,true));
        }
    }

    //for i in &cells {
    //    print!("{},{}\n",i.0,i.1);
    //}

    //Setting the first cell at 0,0 and making it occupied
    let mut visited = 1;

    let mut current: Vec<(u32,u32)> = Vec::new();
    current.push((0,0));
    cells[0] = (cells[0].0,cells[0].1, false);
    

    while visited < maze_height * maze_width{

        //Closure that return the place in the vector of a coordenate
        let coordenate = |x: u32, y: u32,north:bool, east: bool| -> usize  {
            if north {
                let calc = (&current.last().unwrap().1 - y) * maze_width + (&current.last().unwrap().0 + x);
                return calc as usize
            }
            if east {
                let calc = (&current.last().unwrap().1 + y) * maze_width + (&current.last().unwrap().0 - x);
                
                return calc as usize
            }
    
            let calc = (&current.last().unwrap().1 + y) * maze_width + (&current.last().unwrap().0 + x);
            //print!("!{}!",calc);
            
            calc as usize
        };

        let mut free_cells: Vec<u32> = Vec::new();

        print!("{},{}[{}]\n", current.last().unwrap().0, current.last().unwrap().1, visited);

        //Checks if it is possible for there to be a cell in a certain direction
        //If there is a cell there, check if it is visited
        //In case the cell is free, save it in a vector
        if current.last().unwrap().1 > 0 && cells[coordenate(0,1,true,false)].2 {
            //print!("N[{},{}]",cells[coordenate(0,1,true,false)].0,cells[coordenate(0,1,true,false)].1);
            free_cells.push(1);
        }
        if current.last().unwrap().1 < maze_height -1 && cells[coordenate(0,1,false,false)].2 {
            //print!("S[{},{}]",cells[coordenate(0,1,false,false)].0,cells[coordenate(0,1,false,false)].1);
            free_cells.push(2);
        }
        if current.last().unwrap().0 > 0 && cells[coordenate(1,0,false,true)].2 {
            //print!("E[{},{}]",cells[coordenate(1,0,false,true)].0,cells[coordenate(1,0,false,true)].1);
            free_cells.push(3);
        }
        if current.last().unwrap().0 < maze_width-1 && cells[coordenate(1,0,false,false)].2 {
            //print!("W[{},{}]\n",cells[coordenate(1,0,false,false)].0,cells[coordenate(1,0,false,false)].1);
            free_cells.push(4);
        }

        if free_cells.len() > 0 {
            let mut rng = rand::thread_rng();
            let chosen_cell = free_cells[rng.gen_range(0..free_cells.len())];

            match chosen_cell {
                1 => {
                    let position = coordenate(0,1,true,false);
                    cells[position] = (cells[position].0,cells[position].1,false);
                    current.push((cells[position].0,cells[position].1));
                },
                2 => {
                    let position = coordenate(0,1,false,false);
                    cells[position] = (cells[position].0,cells[position].1,false);
                    current.push((cells[position].0,cells[position].1));
                },
                3 => {
                    let position = coordenate(1,0,false,true);
                    cells[position] = (cells[position].0,cells[position].1,false);
                    current.push((cells[position].0,cells[position].1));
                },
                4 => {
                    let position = coordenate(1,0,false,false);
                    cells[position] = (cells[position].0,cells[position].1,false);
                    current.push((cells[position].0,cells[position].1));
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
