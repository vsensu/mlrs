use std::collections::HashMap;

pub fn classify0(in_x: &[f32], data_set: &[Vec<f32>], labels: &[i32], k: usize) -> i32 {
    let mut distances: Vec<(usize, f32)> = data_set
        .iter()
        .enumerate()
        .map(|(i, p)| (i, euclidean_distance(in_x, p)))
        .collect();
    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut class_count: HashMap<i32, usize> = HashMap::new();
    for (i, _) in distances.iter().take(k) {
        *class_count.entry(labels[*i]).or_insert(0) += 1;
    }
    let mut sorted_class_count: Vec<_> = class_count.iter().collect();
    sorted_class_count.sort_by(|a, b| b.1.cmp(&a.1));
    *sorted_class_count[0].0
}

fn euclidean_distance(p1: &[f32], p2: &[f32]) -> f32 {
    p1.iter()
        .zip(p2.iter())
        .map(|(&a, &b)| (a - b).powi(2))
        .sum::<f32>()
        .sqrt()
}
