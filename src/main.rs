use std::collections::HashMap;

use map_macro::hash_map;
use ordered_float::NotNan;
use pathfind_rs::{a_star, Distance};

fn main() {
    let grid = [
        [0, 0, 1, 0, 0usize], //
        [0, 0, 0, 0, 0],      //
        [0, 1, 1, 1, 0],      //
        [0, 0, 0, 0, 0],      //
        [0, 0, 0, 0, 0],      //
    ];

    const SIZE: usize = 5;

    fn index_to_pos(i: usize) -> (usize, usize) {
        (i / SIZE, i % SIZE)
    }

    fn pos_to_index(p: (usize, usize)) -> usize {
        p.0 * SIZE + p.1
    }

    fn access(grid: &[[usize; SIZE]; SIZE], p: (usize, usize)) -> usize {
        if p.0 < SIZE && p.1 < SIZE {
            grid[p.0][p.1]
        } else {
            panic!("out of range")
        }
    }
    fn neighbors(grid: &[[usize; SIZE]; SIZE], p: (usize, usize)) -> [(usize, NotNan<f64>); 4] {
        [(-1, 0), (0, -1), (1, 0), (0, 1)].map(|d| {
            let (dx, dy) = d;
            let (x, y) = p;

            (
                pos_to_index((x.saturating_add_signed(dx), y.saturating_add_signed(dy))),
                NotNan::new(1.0).unwrap(),
            )
        })
    }
    fn distance(p0: (usize, usize), p1: (usize, usize)) -> NotNan<f64> {
        let x0 = p0.0 as f64;
        let y0 = p0.1 as f64;
        let x1 = p1.0 as f64;
        let y1 = p1.1 as f64;
        NotNan::new((x0 - x1).powi(2) + (y0 - y1).powi(2)).unwrap()
    }
    fn hueristic(i: usize, j: usize) -> NotNan<f64> {
        distance(index_to_pos(i), index_to_pos(j))
    }

    dbg!(a_star(
        |n| neighbors(&grid, index_to_pos(n)),
        pos_to_index((0, 0)),
        pos_to_index((4, 4)),
        hueristic,
    ));

    // assert_eq!(a_star(|n|graph., 0, 1), Some(vec![1, 0]));
}
