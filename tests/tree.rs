use std::collections::VecDeque;
use tree_meeting_point::{Center, Tree, TreeError};

#[test]
fn six_vertex_path_has_two_optimal_meeting_points() {
    let tree = Tree::from_edges(6, &[(1, 2), (2, 3), (3, 4), (4, 5), (5, 6)]).unwrap();
    assert_eq!(
        tree.center(),
        Center {
            vertices: vec![3, 4],
            radius: 3,
        }
    );
    assert_eq!(tree.eccentricity(3), Ok(3));
    assert_eq!(tree.eccentricity(4), Ok(3));
    assert_eq!(tree.eccentricity(2), Ok(4));
}

#[test]
fn star_has_one_optimal_meeting_point() {
    let tree = Tree::from_edges(5, &[(3, 1), (3, 2), (3, 4), (3, 5)]).unwrap();
    assert_eq!(
        tree.center(),
        Center {
            vertices: vec![3],
            radius: 1,
        }
    );
    assert_eq!(tree.eccentricity(1), Ok(2));
}

#[test]
fn one_or_two_vertices_require_no_pruning() {
    assert_eq!(
        Tree::from_edges(1, &[]).unwrap().center(),
        Center {
            vertices: vec![1],
            radius: 0,
        }
    );
    assert_eq!(
        Tree::from_edges(2, &[(2, 1)]).unwrap().center(),
        Center {
            vertices: vec![1, 2],
            radius: 1,
        }
    );
}

#[test]
fn invalid_tree_inputs_are_rejected() {
    assert_eq!(Tree::from_edges(0, &[]).unwrap_err(), TreeError::Empty);
    assert_eq!(
        Tree::from_edges(2, &[]).unwrap_err(),
        TreeError::WrongEdgeCount {
            expected: 1,
            actual: 0,
        }
    );
    assert_eq!(
        Tree::from_edges(2, &[(1, 3)]).unwrap_err(),
        TreeError::InvalidVertex {
            vertex: 3,
            vertex_count: 2,
        }
    );
    assert_eq!(
        Tree::from_edges(2, &[(1, 1)]).unwrap_err(),
        TreeError::SelfLoop { vertex: 1 }
    );
    assert_eq!(
        Tree::from_edges(4, &[(1, 2), (2, 3), (3, 1)]).unwrap_err(),
        TreeError::Disconnected
    );
    assert_eq!(
        Tree::from_edges(3, &[(1, 2), (2, 1)]).unwrap_err(),
        TreeError::Disconnected
    );
    assert_eq!(
        Tree::from_edges(1, &[]).unwrap().eccentricity(0),
        Err(TreeError::InvalidVertex {
            vertex: 0,
            vertex_count: 1,
        })
    );
}

#[test]
fn leaf_pruning_agrees_with_all_vertices_bfs_on_seeded_trees() {
    let mut seed = 42_u64;
    for vertex_count in 1..=32 {
        for _ in 0..16 {
            let edges: Vec<_> = (2..=vertex_count)
                .map(|vertex| {
                    seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                    let parent = 1 + ((seed >> 32) % (vertex as u64 - 1)) as usize;
                    (vertex, parent)
                })
                .collect();
            let tree = Tree::from_edges(vertex_count, &edges).unwrap();
            let distances: Vec<_> = (1..=vertex_count)
                .map(|vertex| bfs_eccentricity(&edges, vertex_count, vertex))
                .collect();
            let radius = *distances.iter().min().unwrap();
            let vertices = (1..=vertex_count)
                .filter(|&vertex| distances[vertex - 1] == radius)
                .collect();

            assert_eq!(tree.center(), Center { vertices, radius }, "{edges:?}");
            for (index, distance) in distances.into_iter().enumerate() {
                assert_eq!(tree.eccentricity(index + 1), Ok(distance), "{edges:?}");
            }
        }
    }
}

fn bfs_eccentricity(edges: &[(usize, usize)], vertex_count: usize, start: usize) -> usize {
    let mut distances = vec![None; vertex_count + 1];
    let mut queue = VecDeque::from([start]);
    distances[start] = Some(0);

    while let Some(vertex) = queue.pop_front() {
        let next_distance = distances[vertex].unwrap() + 1;
        for &(a, b) in edges {
            let neighbor = if a == vertex {
                b
            } else if b == vertex {
                a
            } else {
                continue;
            };
            if distances[neighbor].is_none() {
                distances[neighbor] = Some(next_distance);
                queue.push_back(neighbor);
            }
        }
    }

    distances[1..]
        .iter()
        .map(|distance| distance.unwrap())
        .max()
        .unwrap()
}
