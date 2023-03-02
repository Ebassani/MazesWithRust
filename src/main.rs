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
    let coordenate = |x: u32, y: u32,north:bool, east: bool| -> u64  {
        let mut calc_y =  current.last().unwrap().1;
        if north {
            calc_y -= y;
        }
        else {
            calc_y += y;
        }

        let mut calc_x = current.last().unwrap().0;
        if east {
            calc_x -= x;
        }
        else {
            calc_x += x;
        }

        let calc:u64 = (calc_y * maze_width + calc_x).into();
        calc
    };

    if visited < maze_height * maze_width{
        //Checks if it is possible for there to be a cell North of the current one
        if current.last().unwrap().1 > 0{
            //If there is a cell a above the current one, check if it is visited
            if cells[usize::try_from(coordenate(0,1,true,false)).unwrap()].2 {
                print!("{}{}",cells[usize::try_from(coordenate(1,0,true,false)).unwrap()].0,cells[usize::try_from(coordenate(1,0,true,false)).unwrap()].1);
                print!("free")
            }
            else {
                print!("occupied")
            }
        }
    }
    
    
}
