use crate::cell_struct::manage_cell::Cell;

pub fn path(
    maze: &Vec<Cell>, 
    path: Vec<(i32,i32)>, 
    target: &(i32,i32), maze_width: i32
) -> Option<Vec<(i32,i32)>> {
    let coordenate = |x: i32, y: i32| -> usize  {
        let calc = (path.last().unwrap().1 + y) * maze_width + (path.last().unwrap().0 + x);
        
        calc as usize
    };

    
    if path.last().unwrap() == target {
        return Some(path);
    }
    
    
    
    
    None



}