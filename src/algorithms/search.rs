use graph::{Graph, Vertex};
use std::collections::HashSet;

pub fn dfs(g: Graph) {

}

// pub fn explore<'a>(g: &Graph, v: &Vertex, visited: &'a mut Option<HashSet<usize>>, clock: usize) -> &'a Option<HashSet<usize>> {
//     let mut visited = &visited.unwrap_or(HashSet::new());
//     visited.insert(v.get_id());
//     v.pre = Some(clock);
//     clock += 1;
//     for u in g.get_adjacent_vertices(&v.get_id()) {
//         if !visited.contains(&u) {
//             explore(&mut g, g.get_vertex(&u), Option::from(visited), clock);
//         }
//     }
//     v.post = Some(clock);
//     Option::from(visited)
// }