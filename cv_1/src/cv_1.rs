use rayon_hash::HashMap;
use std::fs::read_to_string;

use crate::functions::{
    get_avg_cm_nb, get_avg_dg, get_cl_ds, get_cl_ef, get_cl_ef_dis, get_dg_dis, get_max_cm_ng,
    get_max_dg,
};
use crate::functions_par::{
    get_avg_cm_nb_par, get_avg_dg_par, get_cl_ds_par, get_cl_ef_dis_par, get_cl_ef_par,
    get_dg_dis_par, get_max_cm_ng_par, get_max_dg_par,
};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

pub fn cv_1() {
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

    get_avg_dg(&sparse_matrix);
    get_avg_dg_par(&sparse_matrix);

    get_max_dg(&sparse_matrix);
    get_max_dg_par(&sparse_matrix);

    get_dg_dis(&sparse_matrix);
    get_dg_dis_par(&sparse_matrix);

    /*
    get_cl_ef(&sparse_matrix);
    get_cl_ef_par(&sparse_matrix);
    */

    get_cl_ef_dis(&sparse_matrix);
    get_cl_ef_dis_par(&sparse_matrix);

    get_cl_ds(&sparse_matrix);
    get_cl_ds_par(&sparse_matrix);

    get_avg_cm_nb(&sparse_matrix);
    get_avg_cm_nb_par(&sparse_matrix);

    get_max_cm_ng(&sparse_matrix);
    get_max_cm_ng_par(&sparse_matrix);
}
