use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use rayon_hash::HashMap;

fn parse_author_file(
    nverts_path: &str,
    simplices_path: &str,
    times_path: &str,
) -> HashMap<u32, HashMap<usize, HashMap<usize, usize>>> {
    let nverts_file = BufReader::new(File::open(nverts_path).unwrap());
    let mut simplices_file = BufReader::new(File::open(simplices_path).unwrap()).lines();
    let times_file = BufReader::new(File::open(times_path).unwrap());

    let mut result: HashMap<u32, HashMap<usize, HashMap<usize, usize>>> = HashMap::new();

    for (num, time) in nverts_file.lines().zip(times_file.lines()) {
        let num: usize = num.unwrap().parse().unwrap();
        let time: u32 = time.unwrap().parse().unwrap();

        let mut simplex = Vec::with_capacity(num);
        for _ in 0..num {
            simplex.push(
                simplices_file
                    .next()
                    .unwrap()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            );
        }

        for i in 0..simplex.len() {
            for j in i + 1..simplex.len() {
                let author1 = simplex[i];
                let author2 = simplex[j];
                let year_entry = result.entry(time).or_insert_with(HashMap::new);
                let author1_entry = year_entry
                    .entry(author1.min(author2))
                    .or_insert_with(HashMap::new);
                *author1_entry.entry(author1.max(author2)).or_insert(0) += 1;
            }
        }
    }

    result
}

fn compute_degrees_and_weighted_clustering(
    data: &HashMap<u32, HashMap<usize, HashMap<usize, usize>>>,
) -> HashMap<u32, (f64, f64, f64)> {
    let mut result: HashMap<u32, (f64, f64, f64)> = HashMap::new();

    for (&year, matrix) in data {
        let mut total_degree = 0;
        let mut total_weighted_degree = 0;

        for author_links in matrix.values() {
            let degree = author_links.len();
            let weighted_degree: usize = author_links.values().sum();
            total_degree += degree;
            total_weighted_degree += weighted_degree;
        }

        let avg_degree = total_degree as f64 / matrix.len() as f64;
        let avg_weighted_degree = total_weighted_degree as f64 / matrix.len() as f64;
        let weighted_clustering_coeff = compute_weighted_clustering_coefficient(matrix);

        result.insert(
            year,
            (avg_degree, avg_weighted_degree, weighted_clustering_coeff),
        );
    }

    result
}

fn compute_weighted_clustering_coefficient(matrix: &HashMap<usize, HashMap<usize, usize>>) -> f64 {
    let mut total_coefficient = 0.0;

    let w_max = matrix
        .values()
        .flat_map(|v| v.values())
        .cloned()
        .max()
        .unwrap_or(1) as f64;

    for (&node, neighbors) in matrix {
        let k = neighbors.len() as f64;

        if k <= 1.0 {
            continue; // The clustering coefficient is 0 for k=1
        }

        let mut sum_weights = 0.0;

        for (&neighbor1, &weight1) in neighbors {
            if !matrix.contains_key(&neighbor1) {
                continue;
            }
            for (&neighbor2, &weight2) in neighbors {
                if neighbor1 != neighbor2
                    && matrix.get(&neighbor1).unwrap().contains_key(&neighbor2)
                {
                    sum_weights += (weight1 as f64 + weight2 as f64);
                }
            }
        }

        let c_w = sum_weights / (2.0 * k * (k - 1.0) * w_max);
        total_coefficient += c_w;
    }

    total_coefficient / matrix.len() as f64
}

fn highest_avg_weight_simplex(
    data: &HashMap<u32, HashMap<usize, HashMap<usize, usize>>>,
) -> (usize, usize, f64) {
    let mut max_avg_weight = 0.0;
    let mut max_simplex = (0, 0);

    for matrix in data.values() {
        for (&author1, links) in matrix {
            let total_weight: usize = links.values().sum();
            let avg_weight = total_weight as f64 / links.len() as f64;

            if avg_weight > max_avg_weight {
                max_avg_weight = avg_weight;
                max_simplex = (author1, *links.keys().next().unwrap_or(&0));
            }
        }
    }

    (max_simplex.0, max_simplex.1, max_avg_weight)
}

pub fn cv_2() {
    let mut sparse_matrices: HashMap<u32, HashMap<usize, HashMap<usize, usize>>> =
        parse_author_file(
            "coauth-DBLP-nverts.txt",
            "coauth-DBLP-simplices.txt",
            "coauth-DBLP-times.txt",
        );

    let degrees_over_time = compute_degrees_and_weighted_clustering(&sparse_matrices);
    let (author1, author2, avg_weight) = highest_avg_weight_simplex(&sparse_matrices);
}
