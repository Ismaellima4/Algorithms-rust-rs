#![allow(unused_variables, dead_code, unused_mut)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    /*
     [1 2 3]    [1 4 7]
     [4 5 6] == [2 5 8]
     [7 8 9]    [3 6 9]
    */

    let mut new_matrix = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            new_matrix[i][j] = matrix[j][i];
        }
    }

    new_matrix
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for i in matrix {
        println!("{i:?}")
    }
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment causes rustfmt to add a new line
        [201, 202, 203], //
        [301, 302, 303], //
    ];

    println!("matrix");
    pretty_print(&matrix);
    
    let transposed = transpose(matrix);
    pretty_print(&transposed);
}
