use grid::*;
mod hmm;

fn main() {

    let mut grid_a = grid![[1, 2, 3]
        [4, 5, 6]
        [7, 8, 9]];
    let grid_b = grid![[10, 11, 12]
        [13, 14, 15]
        [16, 17, 18]];

    for ((i, j), val) in grid_a.indexed_iter_mut() {
        println!("value at {} and {} is {}", i, j, val);
    }


    println!("sum of elements in grid a: {}", grid_a.iter().sum::<i32>());

    for ((i, j), val) in grid_a.indexed_iter_mut() {
        *val *= 3;
        println!("value at {} and {} is {}", i, j, val);
    }
    println!("{:?}", grid_a);

    let result:i32 = grid_a.iter().sum();
    println!("{}", result);
    let val: Vec<f32> = vec![10.0, 3.5, 10.23];
    println!("{:?}", val.iter().map(|x| f32::log10(*x)).sum::<f32>());


    let val = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
    println!("{:?}", val);
    println!("{}", val[1][2]);

    let a = vec![vec![0.7, 0.3], vec![0.4, 0.6]];
    let b = vec![vec![0.1, 0.4, 0.5], vec![0.7, 0.2, 0.1]];
    let alpha = vec![];
    let beta = vec![];
    let o = vec![0, 1, 0, 2];
    let mut model = hmm::Hmm{a,
    b, alpha, beta, o, c: vec![],
    t: 4, n: 2, m: 3, pie: vec![0.0, 1.0]
    };

    model.forward();
}

