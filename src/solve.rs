use crate::cell_struct::manage_cell::Cell;

pub fn find_path(
    mut maze: Vec<Cell>, 
    path: Vec<(i32,i32)>, 
    target: (i32,i32), 
    maze_width: i32
) -> Option<Vec<(i32,i32)>> {
    if path.last().unwrap() == &target {
        return Some(path);
    }
    
    let coordenate = |x: i32, y: i32| -> usize  {
        let calc = (path.last().unwrap().1 + y) * maze_width + (path.last().unwrap().0 + x);
        
        calc as usize
    };

    let current = coordenate(0,0);

    maze[current].visit();
    

    let north = coordenate(0,1);
    let south = coordenate(0,-1);
    let east = coordenate(1 ,0);
    let west = coordenate(-1,0);
    
    if maze[current].get_n() && maze[north].free() {
        let mut next = path.clone();
        next.push((maze[north].get_x(),maze[north].get_y()));
        match find_path(maze.clone(), next, target.clone(), maze_width) {
            Some(new_path) => return Some(new_path),
            None => print!("")
        }
    }
    if maze[current].get_s() && maze[south].free() {
        let mut next = path.clone();
        next.push((maze[south].get_x(),maze[south].get_y()));
        match find_path(maze.clone(), next, target.clone(), maze_width) {
            Some(new_path) => return Some(new_path),
            None => print!("")
        }
    }
    if maze[current].get_e() && maze[east].free() {
        let mut next = path.clone();
        next.push((maze[east].get_x(),maze[east].get_y()));
        match find_path(maze.clone(), next, target.clone(), maze_width) {
            Some(new_path) => return Some(new_path),
            None => print!("")
        }
    }
    if maze[current].get_w() && maze[west].free() {
        let mut next = path.clone();
        next.push((maze[west].get_x(),maze[west].get_y()));
        match find_path(maze.clone(), next, target.clone(), maze_width) {
            Some(new_path) => return Some(new_path),
            None => print!("")
        }
    }
    
    
    None

}