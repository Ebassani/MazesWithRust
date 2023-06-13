use crate::cell_struct::manage_cell::Cell;
use std::thread;

pub fn solve(maze: Vec<Cell>, start: (u32,u32), target: (u32,u32), maze_width: u32, maze_height: u32) {
    
    let mut path: Vec<(u32,u32)> = Vec::new();
    path.push(start);

    thread::spawn(move || {

        let coordenate = |x: u32, y: u32,north:bool, west: bool| -> usize  {
            if north {
                let calc = (path.last().unwrap().1 - y) * maze_width + (path.last().unwrap().0 + x);
                return calc as usize
            }
            if west {
                 let calc = (path.last().unwrap().1 + y) * maze_width + (path.last().unwrap().0 - x);
                    
                return calc as usize
            }
        
            let calc = (path.last().unwrap().1 + y) * maze_width + (path.last().unwrap().0 + x);
                
            calc as usize
        };

        let current = path.last().unwrap();

        let mut free_cells: Vec<(u32,u32)> = Vec::new();

        if current.1 > 0 && !maze[coordenate(0,1,true,false)].free() {
            let position = coordenate(0,1,true,false);
            free_cells.push((maze[position].get_x(), maze[position].get_y()));
        }
        if current.1 < maze_height -1 && !maze[coordenate(0,1,false,false)].free() {
            let position = coordenate(0,1,false,false);
            free_cells.push((maze[position].get_x(), maze[position].get_y()));
        }
        if current.0 > 0 && !maze[coordenate(1,0,false,true)].free() {
            let position = coordenate(1,0,false,true);
            free_cells.push((maze[position].get_x(), maze[position].get_y()));
        }
        if current.0 < maze_width-1 && !maze[coordenate(1,0,false,false)].free() {
            let position = coordenate(1,0,false,false);
            free_cells.push((maze[position].get_x(), maze[position].get_y()));
        }
            
    });
    
}