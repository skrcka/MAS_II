use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use rayon_hash::HashMap;
use std::fs::{read_to_string, write};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn main() {
    let mut sparse_matrix: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
    let lines = read_lines("com-youtube.ungraph.txt");

    for line in lines {
        let mut iter = line.split_whitespace();
        let from = iter.next().unwrap().parse::<usize>().unwrap();
        let to = iter.next().unwrap().parse::<usize>().unwrap();
        sparse_matrix
            .entry(from)
            .or_insert(HashMap::new())
            .entry(to)
            .or_insert(1);
    }

    // Get average degree
    let start = std::time::Instant::now();
    let mut sum = 0;
    for (_, v) in sparse_matrix.iter() {
        sum += v.len();
    }
    let avg_degree = sum as f64 / sparse_matrix.len() as f64;
    let end = std::time::Instant::now();
    println!(
        "Average degree: {} in {}",
        avg_degree,
        (end - start).as_millis()
    );

    // Get average degree par
    let sparse_matrix_copy = sparse_matrix.clone();
    let start = std::time::Instant::now();
    let sum: usize = sparse_matrix_copy
        .into_par_iter()
        .map(|(_, v)| v.len())
        .sum();
    let avg_degree = sum as f64 / sparse_matrix.len() as f64;
    let end = std::time::Instant::now();
    println!(
        "Average degree par: {} in {}",
        avg_degree,
        (end - start).as_millis()
    );

    // Get max degree
    let start = std::time::Instant::now();
    let mut max_degree = 0;
    for (_, v) in sparse_matrix.iter() {
        if v.len() > max_degree {
            max_degree = v.len();
        }
    }
    let end = std::time::Instant::now();
    println!(
        "Max degree: {} in {}",
        max_degree,
        (end - start).as_millis()
    );

    // Get max degree par
    let sparse_matrix_copy = sparse_matrix.clone();
    let start = std::time::Instant::now();
    let max_degree: usize = sparse_matrix_copy
        .into_par_iter()
        .map(|(_, v)| v.len())
        .max()
        .unwrap();
    let end = std::time::Instant::now();
    println!(
        "Max degree par: {} in {}",
        max_degree,
        (end - start).as_millis()
    );

    // Get degree distribution
    let start = std::time::Instant::now();
    let mut degree_distribution: HashMap<usize, usize> = HashMap::new();
    for (_, v) in sparse_matrix.iter() {
        degree_distribution
            .entry(v.len())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    let end = std::time::Instant::now();
    println!("Degree distribution in {}", (end - start).as_millis());

    // Get degree distribution par
    let sparse_matrix_copy = sparse_matrix.clone();
    let start = std::time::Instant::now();
    let mut degree_distribution: HashMap<usize, usize> = HashMap::new();
    let degree_distribution_arc =
        std::sync::Arc::new(std::sync::Mutex::new(&mut degree_distribution));
    sparse_matrix_copy.into_par_iter().for_each(|(_, v)| {
        degree_distribution_arc
            .lock()
            .unwrap()
            .entry(v.len())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    });
    let end = std::time::Instant::now();
    println!("Degree distribution par in {}", (end - start).as_millis());

    // write degree distribution to file
    let mut degree_distribution_vec: Vec<(usize, usize)> = degree_distribution
        .into_iter()
        .map(|(k, v)| (k, v))
        .collect();
    degree_distribution_vec.sort_by(|a, b| a.0.cmp(&b.0));
    write(
        "distributions.txt",
        degree_distribution_vec
            .iter()
            .map(|(k, v)| format!("{} {}", k, v))
            .collect::<Vec<String>>()
            .join("\n")
            .as_bytes(),
    )
    .unwrap();

    // Get clustering effect
    let start = std::time::Instant::now();
    let mut sum = 0;
    for (_, neighbors) in sparse_matrix.iter() {
        let mut count = 0;
        for neighbor in neighbors.keys() {
            if !sparse_matrix.contains_key(neighbor) {
                continue;
            }
            for neighbor_neighbor in sparse_matrix.get(neighbor).unwrap().keys() {
                if neighbors.contains_key(neighbor_neighbor) {
                    count += 1;
                }
            }
        }
        sum += count;
    }
    let clustering_effect = sum as f64 / sparse_matrix.len() as f64;
    let end = std::time::Instant::now();
    println!(
        "Clustering effect: {} in {}",
        clustering_effect,
        (end - start).as_millis()
    );

    // Get clustering effect par
    let sparse_matrix_copy = sparse_matrix.clone();
    let sparse_matrix_arc = std::sync::Arc::new(sparse_matrix_copy);
    let start = std::time::Instant::now();
    let sum: usize = sparse_matrix_arc
        .iter()
        .par_bridge()
        .map(|(_, neighbors)| {
            let matrix_ref = sparse_matrix_arc.as_ref();
            neighbors
                .keys()
                .filter_map(|&neighbor| matrix_ref.get(&neighbor))
                .flatten()
                .filter(|&(key, _)| neighbors.contains_key(key))
                .count()
        })
        .sum();

    let clustering_effect = sum as f64 / sparse_matrix_arc.len() as f64;

    let end = std::time::Instant::now();
    println!(
        "Clustering effect par: {} in {}",
        clustering_effect,
        (end - start).as_millis()
    );

    // Get clustering distribution
    let sparse_matrix_copy = sparse_matrix.clone();
    let start = std::time::Instant::now();
    let mut clustering_distribution: HashMap<usize, usize> = HashMap::new();
    for (_, neighbors) in &sparse_matrix_copy {
        let mut count = 0;
        for &neighbor in neighbors.keys() {
            if let Some(neighbor_neighbors) = sparse_matrix.get(&neighbor) {
                for &neighbor_neighbor in neighbor_neighbors.keys() {
                    if neighbors.contains_key(&neighbor_neighbor) {
                        count += 1;
                    }
                }
            }
        }
        clustering_distribution
            .entry(count)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    let end = std::time::Instant::now();
    println!("Clustering distribution in {}", (end - start).as_millis());

    // Get clustering distribution par
    let start = std::time::Instant::now();
    let mut clustering_distribution: HashMap<usize, usize> = HashMap::new();
    let sparse_matrix_copy = sparse_matrix.clone();
    let results: Vec<HashMap<usize, usize>> = sparse_matrix_copy
        .into_par_iter()
        .map(|(_, neighbors)| {
            let mut local_distribution: HashMap<usize, usize> = HashMap::new();
            let mut count = 0;
            for &neighbor in neighbors.keys() {
                if let Some(neighbor_neighbors) = sparse_matrix.get(&neighbor) {
                    for &neighbor_neighbor in neighbor_neighbors.keys() {
                        if neighbors.contains_key(&neighbor_neighbor) {
                            count += 1;
                        }
                    }
                }
            }
            local_distribution
                .entry(count)
                .and_modify(|e| *e += 1)
                .or_insert(1);

            local_distribution
        })
        .collect();

    for local_dist in results.iter() {
        for (&k, &v) in local_dist.iter() {
            clustering_distribution
                .entry(k)
                .and_modify(|e| *e += v)
                .or_insert(v);
        }
    }

    let end = std::time::Instant::now();
    println!(
        "Clustering distribution par in {}",
        (end - start).as_millis()
    );

    // Get average common neighbors
    let start = std::time::Instant::now();
    let mut sum = 0;
    for (_, neighbors) in sparse_matrix.iter() {
        let mut count = 0;
        for neighbor in neighbors.keys() {
            if !sparse_matrix.contains_key(neighbor) {
                continue;
            }
            for neighbor_neighbor in sparse_matrix.get(neighbor).unwrap().keys() {
                if neighbors.contains_key(neighbor_neighbor) {
                    count += 1;
                }
            }
        }
        sum += count;
    }
    let avg_common_neighbors = sum as f64 / sparse_matrix.len() as f64;
    let end = std::time::Instant::now();
    println!(
        "Average common neighbors: {} in {}",
        avg_common_neighbors,
        (end - start).as_millis()
    );

    // Get average common neighbors par
    let sparse_matrix_copy = sparse_matrix.clone();
    let start = std::time::Instant::now();
    let sum: usize = sparse_matrix_copy
        .into_par_iter()
        .map(|(_, neighbors)| {
            let mut count = 0;
            for neighbor in neighbors.keys() {
                if !sparse_matrix.contains_key(neighbor) {
                    continue;
                }
                for neighbor_neighbor in sparse_matrix.get(neighbor).unwrap().keys() {
                    if neighbors.contains_key(neighbor_neighbor) {
                        count += 1;
                    }
                }
            }
            count
        })
        .sum();
    let avg_common_neighbors = sum as f64 / sparse_matrix.len() as f64;
    let end = std::time::Instant::now();
    println!(
        "Average common neighbors par: {} in {}",
        avg_common_neighbors,
        (end - start).as_millis()
    );

    // Get maximum common neighbors
    let start = std::time::Instant::now();
    let mut max = 0;
    for (_, neighbors) in sparse_matrix.iter() {
        let mut count = 0;
        for neighbor in neighbors.keys() {
            if !sparse_matrix.contains_key(neighbor) {
                continue;
            }
            for neighbor_neighbor in sparse_matrix.get(neighbor).unwrap().keys() {
                if neighbors.contains_key(neighbor_neighbor) {
                    count += 1;
                }
            }
        }
        if count > max {
            max = count;
        }
    }
    let end = std::time::Instant::now();
    println!(
        "Maximum common neighbors: {} in {}",
        max,
        (end - start).as_millis()
    );

    // Get maximum common neighbors par
    let sparse_matrix_copy = sparse_matrix.clone();
    let start = std::time::Instant::now();
    let max = sparse_matrix_copy
        .into_par_iter()
        .map(|(_, neighbors)| {
            let mut count = 0;
            for neighbor in neighbors.keys() {
                if !sparse_matrix.contains_key(neighbor) {
                    continue;
                }
                for neighbor_neighbor in sparse_matrix.get(neighbor).unwrap().keys() {
                    if neighbors.contains_key(neighbor_neighbor) {
                        count += 1;
                    }
                }
            }
            count
        })
        .max()
        .unwrap();
    let end = std::time::Instant::now();
    println!(
        "Maximum common neighbors par: {} in {}",
        max,
        (end - start).as_millis()
    );
}

/*
Average degree: 7.9715676988139865 in 1
Average degree par: 7.9715676988139865 in 122
Max degree: 28576 in 1
Max degree par: 28576 in 92
Degree distribution in 8
Degree distribution par in 141
Clustering effect: 8.155038221913898 in 4829
Clustering effect par: 8.155038221913898 in 809
Clustering distribution in 5080
Clustering distribution par in 1074
Average common neighbors: 8.155038221913898 in 4993
Average common neighbors par: 8.155038221913898 in 885
Maximum common neighbors: 152258 in 4989
Maximum common neighbors par: 152258 in 725
*/
