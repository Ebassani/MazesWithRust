use rand::Rng;

fn main() {
    let maze_height = 10;
    let maze_width = 10;

    //Cells have a x and y coordenate and a boolean to check if they have been visited before
    let mut cells: Vec<(u32,u32,bool)> = Vec::new();

    for i in 0..maze_height{
        for j in 0..maze_width {
            cells.push((i,j,true));
        }
    }

    //Setting the first cell at 0,0 and making it occupied
    let visited = 1;

    let mut current: Vec<(u32,u32)> = Vec::new();
    current.push((0,0));
    cells[0] = (cells[0].0,cells[0].1, false);


    //Closure that return the place in the vector of a coordenate
    let coordenate = |x: u32, y: u32,north:bool, east: bool| -> usize  {
        if north {
            let calc = (current.last().unwrap().1) - y * maze_width + (current.last().unwrap().0 + x);
            let calc = usize::try_from(calc).unwrap();
            return calc
        }
        if east {
            let calc = (current.last().unwrap().1) + y * maze_width + (current.last().unwrap().0 - x);
            let calc = usize::try_from(calc).unwrap();
            return calc
        }

        let calc = (current.last().unwrap().1) + y * maze_width + (current.last().unwrap().0 + x);
        let calc = usize::try_from(calc).unwrap();
        calc
    };

    if visited < maze_height * maze_width{

        let mut free_cells: Vec<u32> = Vec::new();

        //Checks if it is possible for there to be a cell in a certain direction
        //If there is a cell there, check if it is visited
        //In case the cell is free, save it in a vector
        if current.last().unwrap().1 > 0 && cells[coordenate(0,1,true,false)].2 {
            free_cells.push(1);
        }
        if current.last().unwrap().1 < maze_height && cells[coordenate(0,1,false,false)].2 {
            free_cells.push(2);
        }
        if current.last().unwrap().0 > 0 && cells[coordenate(1,0,false,true)].2 {
            free_cells.push(3);
        }
        if current.last().unwrap().0 < maze_width && cells[coordenate(1,1,false,false)].2 {
            free_cells.push(4);
        }

        if free_cells.len() > 0 {
            let mut rng = rand::thread_rng();
            let chosen_cell = free_cells[rng.gen_range(0..free_cells.len())];

            match chosen_cell {
                1 => {
                    let position = coordenate(0,1,true,false);
                    current.push((cells[position].0,cells[position].1));
                    
                },
                2 => {
                    let position = coordenate(0,1,true,false);
                    current.push((cells[position].0,cells[position].1));
                },
                3 => {
                    let position = coordenate(0,1,true,false);
                    current.push((cells[position].0,cells[position].1));
                },
                4 => {
                    let position = coordenate(0,1,true,false);
                    current.push((cells[position].0,cells[position].1));
                }
                _ => print!("error")
            }
        }

        
    }
    
    
}
