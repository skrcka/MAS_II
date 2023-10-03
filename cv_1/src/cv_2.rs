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

pub fn cv_2() {
    let mut sparse_matrices: HashMap<u32, HashMap<usize, HashMap<usize, usize>>> =
        parse_author_file(
            "coauth-DBLP-nverts.txt",
            "coauth-DBLP-simplices.txt",
            "coauth-DBLP-times.txt",
        );
}
