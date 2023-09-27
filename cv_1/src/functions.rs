use rayon_hash::HashMap;

pub fn get_avg_dg(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) {
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
}

pub fn get_max_dg(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) {
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
}

pub fn get_dg_dis(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) {
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
}

pub fn get_cl_ef(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) {
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
}

pub fn get_cl_ds(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) {
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
}

pub fn get_avg_cm_nb(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) {
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
}

pub fn get_max_cm_ng(sparse_matrix: &HashMap<usize, HashMap<usize, usize>>) {
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
}
