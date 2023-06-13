mod cell_struct;
pub mod generate;
use generate::maze;

fn main() {

    let maze_height = 10;
    let maze_width = 10;

    let maze = maze(maze_height, maze_width);

    /* for cell in maze {
        print!("C({},{})", cell.get_y(), cell.get_x());
        print!("N{},S{},L{},O{}\n", cell.get_n(),cell.get_s(),cell.get_e(),cell.get_w() )
    } */
    
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
    
}
