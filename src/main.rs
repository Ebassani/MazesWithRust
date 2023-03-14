mod cell_struct;
pub mod generate;
use generate::maze;

fn main() {

    let maze_height = 10;
    let maze_width = 10;

    let maze = maze(maze_height, maze_width);
    

    for i in 0..maze_height{
        for j in 0..maze_width {
            if maze[(i*10 + j) as usize].get_w() {
                print!("-");
            }
            else {
                print!("|");
            }
            print!("c");
            if maze[(i*10 + j) as usize].get_e() {
                print!("-");
            }
            else {
                print!("|");
            }
        }
        print!("\n");
        for j in 0..maze_width {
            print!(" ");
            if maze[(i*10 + j) as usize].get_s() {
                print!("|");
            }
            else {
                print!("-");
            }
            print!(" ")
        }
        print!("\n");
    }
    
}
