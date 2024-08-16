#![feature(return_position_impl_trait_in_trait)]

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    ops::Add,
};

use map_macro::{binary_heap, hash_map};
use ordered_float::NotNan;

pub type Distance = NotNan<f64>;

// pub trait Graph {
//     fn as_graph(&self) -> &HashMap<usize, HashMap<usize, Distance>>;
//     fn hueristic(&self, begin: usize, end: usize) -> Distance;
//     fn neighbors(&self) -> impl Iterator<Item = (usize, Distance)>;
// }
// impl Graph for HashMap<usize, HashMap<usize, Distance>> {
//     fn as_graph(&self) -> &HashMap<usize, HashMap<usize, Distance>> {
//         self
//     }
//     fn hueristic(&self, begin: usize, end: usize) -> Distance {
//         (end as f32 - begin as f32).try_into().unwrap()
//     }
// }

#[derive(PartialEq, Eq)]
pub struct EdgeTo<T>(usize, T);

impl<T: PartialOrd> PartialOrd for EdgeTo<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.1.partial_cmp(&other.1)
    }
}
impl<T: Ord> Ord for EdgeTo<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

fn reconstruct_path(mut came_from: HashMap<usize, usize>, mut current: usize) -> Vec<usize> {
    let mut vec = vec![current];
    while let Some(node) = came_from.remove(&current) {
        current = node;
        vec.push(node);
    }
    vec
}

pub fn a_star<Neighbors: IntoIterator<Item = (usize, Distance)>>(
    mut neighbors: impl FnMut(usize) -> Neighbors,
    begin: usize,
    end: usize,
    mut hueristic: impl FnMut(usize, usize) -> Distance,
) -> Option<Vec<usize>> {
    // assert!(neighbors.as_graph().contains_key(&begin));
    // assert!(neighbors.as_graph().contains_key(&end));

    let mut from_start = hash_map!(begin => NotNan::default());
    let mut to_end = hash_map!(begin => hueristic(begin, end));

    let mut came_from: HashMap<usize, usize> = HashMap::new();

    let mut open_set = binary_heap![EdgeTo(begin, NotNan::default())];

    while let Some(EdgeTo(node, _)) = open_set.pop() {
        println!("Mainloop with {}", node);
        if node == end {
            return Some(reconstruct_path(came_from, end));
        }

        for (neighbor, dist) in neighbors(node).into_iter() {
            let newpath_len = from_start[&node] + dist;
            if newpath_len
                < *from_start
                    .get(&neighbor)
                    .unwrap_or(&f64::MAX.try_into().unwrap())
            {
                came_from.insert(neighbor, node);
                from_start.insert(neighbor, newpath_len);
                to_end.insert(neighbor, newpath_len + hueristic(neighbor, end));
            }
            open_set.push(EdgeTo(neighbor, newpath_len));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

// #[derive(PartialEq, Eq, Hash)]
// struct Node(usize);

// #[derive(PartialEq, Eq, Hash)]
// struct Edge {
//     source: Node,
//     target: Node,
//     weight: f32,
// }

// type Graph = HashMap<usize, HashMap<usize, f32>>;

// Node {
//     to_neighbors: HashMap<usize, f32>
// }

// struct EdgeTo(usize, f32);
// impl Ord for EdgeTo {
//     fn ord(&self, other: &self) {
//         self.1.cmp(other.1)
//     }
// }

// fn main(graph: &Graph, start: usize, end: usize, h: impl FnMut(&Graph, usize, usize) -> f32) {
//     let from_start = map!(start => 0);
//     let to_end = map!(start => h(graph, start, end));

//     let mut tracker = HashMap::new();

//     let mut open_set = {start};
//     while let Some(node) = open_set.pop() {
//         if node == end {
//             return reconstruct_path()
//         }

//         for (neighbor, dist) in graph[node] {
//             let newpath_len = from_start(node) + dist;
//             if newpath_len < from_start.get_or(neighbor, f32::MAX) {
//                 from_start[neighbor] = newpath_len;
//                 to_end[neighbor] = newpath_len + h(graph, neighbor, end);
//             }
//             open_set.add(neighbor);
//         }
//     }
//     return failure
// }
