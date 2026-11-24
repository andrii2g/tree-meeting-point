# tree-meeting-point

A Rust library and CLI that find every vertex in an unweighted tree that minimizes the farthest walk to any other vertex. It uses only the Rust standard library.

## CLI

Pass a UTF-8 text file with the vertex count on the first line and one undirected edge per line after that. Number the vertices from 1 through the vertex count. Use whitespace between the two IDs. Blank lines after the first line are ignored.

```shell
cargo run -- examples/path.txt --compare 2
```

```text
Optimal meeting points: 3, 4
Optimal worst-case walk: 3 edges
Vertex 2 worst-case walk: 4 edges (+1 edge)
```

The comparison is optional. Either middle vertex of this six-vertex path gives the same worst-case walk of three edges.

## Library

```rust
use tree_meeting_point::{Tree, TreeError};

fn main() -> Result<(), TreeError> {
    let tree = Tree::from_edges(4, &[(1, 2), (2, 3), (3, 4)])?;
    let center = tree.center();
    assert_eq!(center.vertices, vec![2, 3]);
    assert_eq!(center.radius, 2);
    assert_eq!(tree.eccentricity(1)?, 3);
    Ok(())
}
```

The library validates that the graph is a tree. It finds the center by removing all current leaves in simultaneous rounds until one vertex or two adjacent vertices remain. Each round reduces the farthest distance from every surviving vertex by one. The radius is the number of rounds plus one if the final two vertices share an edge. Breadth-first search measures the farthest distance from a chosen vertex.

Validation, center finding, and each chosen-vertex comparison take linear time and space in the number of vertices for a valid tree. Run `cargo test` to check the path and star examples and compare the pruning result with an all-vertices breadth-first search on seeded trees.
