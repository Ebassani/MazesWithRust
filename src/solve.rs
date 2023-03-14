use crate::cell_struct::manage_cell::Cell;
use std::thread;

pub fn solve(maze: Vec<Cell>, start: (u32,u32), target: (u32,u32), maze_width: u32, maze_height: u32) -> Vec<(u32,u32)> {
    let mut current: Vec<(u32,u32)> = Vec::new();
    current.push(start);

    thread::scope(|scope| {

        fn follow_path(path: Vec<(u32,u32)>) {
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


            scope.spawn(|| {
                
                if path.last() == target {
                    current = path;
                }
                else {
                    
                }
            });
        }
        
        
    });
}