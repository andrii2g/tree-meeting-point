//! Find the vertices of an unweighted tree that minimize the farthest walk.

use std::collections::VecDeque;
use std::fmt;

/// A connected, undirected tree with vertex IDs from 1 through its vertex count.
#[derive(Debug)]
pub struct Tree {
    neighbors: Vec<Vec<usize>>,
}

/// All optimal meeting points and their shared worst-case distance in edges.
#[derive(Debug, PartialEq, Eq)]
pub struct Center {
    /// One or two vertex IDs in ascending order.
    pub vertices: Vec<usize>,
    /// The minimum eccentricity of any vertex.
    pub radius: usize,
}

/// An invalid tree definition or vertex choice.
#[derive(Debug, PartialEq, Eq)]
pub enum TreeError {
    Empty,
    WrongEdgeCount { expected: usize, actual: usize },
    InvalidVertex { vertex: usize, vertex_count: usize },
    SelfLoop { vertex: usize },
    Disconnected,
}

impl fmt::Display for TreeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "a tree must contain at least one vertex"),
            Self::WrongEdgeCount { expected, actual } => {
                write!(f, "expected {expected} edges, found {actual}")
            }
            Self::InvalidVertex {
                vertex,
                vertex_count,
            } => write!(f, "vertex {vertex} is outside 1..={vertex_count}"),
            Self::SelfLoop { vertex } => write!(f, "vertex {vertex} has a self-loop"),
            Self::Disconnected => write!(f, "the graph is disconnected"),
        }
    }
}

impl std::error::Error for TreeError {}

impl Tree {
    /// Build a tree from undirected edges using 1-based vertex IDs.
    pub fn from_edges(vertex_count: usize, edges: &[(usize, usize)]) -> Result<Self, TreeError> {
        if vertex_count == 0 {
            return Err(TreeError::Empty);
        }
        if edges.len() != vertex_count - 1 {
            return Err(TreeError::WrongEdgeCount {
                expected: vertex_count - 1,
                actual: edges.len(),
            });
        }

        let mut tree = Self {
            neighbors: vec![Vec::new(); vertex_count],
        };
        for &(a, b) in edges {
            let a_index = tree.vertex_index(a)?;
            let b_index = tree.vertex_index(b)?;
            if a == b {
                return Err(TreeError::SelfLoop { vertex: a });
            }
            tree.neighbors[a_index].push(b_index);
            tree.neighbors[b_index].push(a_index);
        }

        // Connectivity and exactly n - 1 edges also rule out cycles and repeated edges.
        let (reached, _) = tree.breadth_first(0);
        if reached != vertex_count {
            return Err(TreeError::Disconnected);
        }
        Ok(tree)
    }

    /// Find the optimal meeting points by removing leaves in simultaneous rounds.
    pub fn center(&self) -> Center {
        let mut degree: Vec<_> = self.neighbors.iter().map(Vec::len).collect();
        let mut leaves: VecDeque<_> = degree
            .iter()
            .enumerate()
            .filter_map(|(vertex, &degree)| (degree <= 1).then_some(vertex))
            .collect();
        let mut remaining = self.neighbors.len();
        let mut rounds = 0;

        while remaining > 2 {
            let round_size = leaves.len();
            for _ in 0..round_size {
                let leaf = leaves.pop_front().expect("a tree has leaves");
                degree[leaf] = 0;
                for &neighbor in &self.neighbors[leaf] {
                    if degree[neighbor] > 0 {
                        degree[neighbor] -= 1;
                        if degree[neighbor] == 1 {
                            leaves.push_back(neighbor);
                        }
                    }
                }
            }
            remaining -= round_size;
            rounds += 1;
        }

        // A last surviving hub may have degree zero after entering the queue.
        let mut vertices: Vec<_> = leaves.into_iter().map(|vertex| vertex + 1).collect();
        vertices.sort_unstable();
        Center {
            vertices,
            radius: rounds + usize::from(remaining == 2),
        }
    }

    /// Return the farthest distance in edges from the given vertex.
    pub fn eccentricity(&self, vertex: usize) -> Result<usize, TreeError> {
        let (_, farthest) = self.breadth_first(self.vertex_index(vertex)?);
        Ok(farthest)
    }

    fn vertex_index(&self, vertex: usize) -> Result<usize, TreeError> {
        let vertex_count = self.neighbors.len();
        if (1..=vertex_count).contains(&vertex) {
            Ok(vertex - 1)
        } else {
            Err(TreeError::InvalidVertex {
                vertex,
                vertex_count,
            })
        }
    }

    fn breadth_first(&self, start: usize) -> (usize, usize) {
        let mut visited = vec![false; self.neighbors.len()];
        let mut queue = VecDeque::from([(start, 0)]);
        visited[start] = true;
        let mut reached = 0;
        let mut farthest = 0;

        while let Some((vertex, distance)) = queue.pop_front() {
            reached += 1;
            farthest = distance;
            for &neighbor in &self.neighbors[vertex] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back((neighbor, distance + 1));
                }
            }
        }
        (reached, farthest)
    }
}
