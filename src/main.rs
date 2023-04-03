use mlrs;

fn main() {
    let data_set = vec![
        vec![1.0, 1.1],
        vec![1.0, 1.0],
        vec![0.0, 0.0],
        vec![0.0, 0.1],
    ];
    let labels = vec![1, 1, 2, 2];
    let in_x = vec![0.0, 0.2];
    let k = 3;
    let result = mlrs::classify0(&in_x, &data_set, &labels, k);
    println!(
        "The input point {:?} is classified as label {}",
        in_x, result
    );
}
