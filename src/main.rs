mod cell_struct;
pub mod generate;
use generate::maze;

pub mod solve;
use solve::find_path;

fn main() {

    let maze_height = 10;
    let maze_width = 10;

    let maze = maze(maze_height, maze_width);
    
    for i in (0..maze_height).rev() {
        for j in 0..maze_width {
            if maze[(i*maze_height + j) as usize].get_w() {
                print!("-");
            }
            else {
                print!("|");
            }
            print!("c");
            if maze[(i*maze_height + j) as usize].get_e() {
                print!("-");
            }
            else {
                print!("|");
            }
        }
        print!("\n");
        for j in 0..maze_width {
            print!(" ");
            if maze[(i*maze_height + j) as usize].get_s() {
                print!("|");
            }
            else {
                print!("-");
            }
            print!(" ")
        }
        print!("\n");
    }

    let mut path: Vec<(i32, i32)> = Vec::new();
    path.push((0,0));

    match find_path(maze, path, (5,5), maze_width) {
        Some(found_path) => found_path.iter().enumerate().for_each(|(_index, value)| {
            println!("{},{}", &value.0, &value.1)
        }),
        None => println!("Error")
    }

    
}
