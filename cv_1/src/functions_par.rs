use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use rayon_hash::HashMap;
use std::fs::write;

pub fn get_avg_dg_par(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) {
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
}

pub fn get_max_dg_par(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) {
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
}

pub fn get_dg_dis_par(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) {
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
}

pub fn get_cl_ef_par(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) {
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
}

fn get_cl_coef(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>, node: usize) -> f64 {
    let neighbors = match sparse_matrix.get(&node) {
        Some(neigh) => neigh.keys().collect::<Vec<&usize>>(),
        None => return 0.0, // node doesn't exist
    };

    if neighbors.len() < 2 {
        return 0.0; // no way to form a triangle with less than 2 neighbors
    }

    let mut triangles = 0;
    for i in 0..neighbors.len() {
        for j in i + 1..neighbors.len() {
            if let Some(inner) = sparse_matrix.get(neighbors[i]) {
                if inner.contains_key(neighbors[j]) {
                    triangles += 1;
                }
            }
        }
    }

    let triples = neighbors.len() * (neighbors.len() - 1) / 2;
    triangles as f64 / triples as f64
}

pub fn get_cl_ef_dis_par(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) {
    let start = std::time::Instant::now();

    let coefficients: HashMap<usize, f64> = sparse_matrix
        .into_par_iter()
        .map(|(&node, _)| (node, get_cl_coef(sparse_matrix, node)))
        .collect();

    let mut degree_to_coefficients: HashMap<usize, Vec<f64>> = HashMap::new();

    for (node, coeff) in coefficients.iter() {
        let degree = sparse_matrix[node].len();
        degree_to_coefficients
            .entry(degree)
            .or_default()
            .push(*coeff);
    }

    let mut distribution_vec: Vec<(usize, f64)> = degree_to_coefficients
        .into_iter()
        .map(|(degree, coeffs)| {
            let avg_coeff = coeffs.iter().sum::<f64>() / coeffs.len() as f64;
            (degree, avg_coeff)
        })
        .collect();

    let end = std::time::Instant::now();
    println!(
        "Clustering effect distribution par in {}",
        (end - start).as_millis()
    );

    distribution_vec.sort_by(|a, b| a.0.cmp(&b.0));
    write(
        "cls_distribution.txt",
        distribution_vec
            .iter()
            .map(|(k, v)| format!("{} {}", k, v))
            .collect::<Vec<String>>()
            .join("\n")
            .as_bytes(),
    )
    .unwrap();
}

pub fn get_cl_ds_par(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) {
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
}

pub fn get_avg_cm_nb_par(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) {
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
}

pub fn get_max_cm_ng_par(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) {
    let sparse_matrix_copy = sparse_matrix.clone();
    let sparse_matrix_copy2 = sparse_matrix.clone();
    let start = std::time::Instant::now();
    let max = sparse_matrix_copy
        .into_par_iter()
        .map(|(_, neighbors)| {
            let mut count = 0;
            for neighbor in neighbors.keys() {
                for neighbor_neighbor in sparse_matrix_copy2.keys() {
                    let n = sparse_matrix_copy2.get(neighbor_neighbor).unwrap();
                    if n.contains_key(neighbor) {
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
