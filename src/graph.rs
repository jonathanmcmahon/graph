/// Common graph types and graph components.
///
/// # Examples
///
/// ```
/// use graph::graph::Graph;
///
/// let mut G = Graph::new();
/// G.add_vertex();
/// G.add_vertex();
/// let vertex1 = G.get_vertex(&0).unwrap().id();
/// let vertex2 = G.get_vertex(&1).unwrap().id();
/// G.add_edge(&vertex1, &vertex2);
/// assert_eq!(G.count_vertices(), 2);
/// G.display();
/// ```

pub struct Vertex {
    id: usize,
    pub pre: Option<usize>,
    pub post: Option<usize>,
}

impl Vertex {
    pub fn id(&self) -> usize { self.id }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool { self.id == other.id }
}

pub struct Edge {
    endpoint: Vertex,
}

pub struct Graph {
    pub name: String,
    next_id: usize,
    pub vertices: Vec<Vertex>,
    pub adjacency_list: AdjacencyList,
}

impl Graph { 

    pub fn new() -> Graph {
        Graph { name: String::from(""), next_id: 0, vertices: Vec::new(), adjacency_list: AdjacencyList::new() }
    }

    pub fn add_vertex(&mut self) {
        self.vertices.push(Vertex { id: self.next_id, pre: None, post: None });
        assert_eq!(self.next_id, self.adjacency_list.vertices.len());
        self.adjacency_list.vertices.push(Vec::new());
        self.next_id += 1;
    }

    pub fn add_edge(&mut self, src: &usize, dst: &usize) {
        self.adjacency_list.add_edge(src, dst);
    }

    pub fn add_vertices(&mut self, n_vertices: usize) -> &mut Graph {
        for _ in 0..n_vertices {
            self.add_vertex();
        }
        self
    }
    
    pub fn fully_connect(&mut self) -> &mut Graph {
        for i in 0..self.vertices.len() {
            let vertex1 = self.vertices[i].id;
            for j in 0..self.vertices.len() {
                let vertex2 = self.vertices[j].id;
                self.add_edge(&vertex1, &vertex2);
            }
        }
        self
    }

    pub fn count_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn count_edges(&self) -> usize {
        let mut total = 0;
        for v in &self.adjacency_list.vertices {
            total += v.len();
        }
        total
    }

    pub fn delete_vertex(&mut self, v: &usize) {
        self.vertices.retain(|element| element.id() != *v);
        self.adjacency_list.delete_vertex(&v);
    }

    pub fn display(&self) {
        for v in &self.vertices {
            println!("|vertex {}| edges: {:?}", v.id, self.adjacency_list.vertices[v.id]);
        }
    }

    pub fn get_vertex(&self, id: &usize) -> Option<&Vertex> {
        for v in &self.vertices {
            if v.id() == *id {
                return Some(&v);
            }
        }
        None
    }

    pub fn get_adjacent_vertices(&self, v: usize) -> &Vec<usize> {
        self.adjacency_list.get_adjacent_vertices(v)
    }

}

pub struct AdjacencyList {
    vertices: Vec<Vec<usize>>,
}

impl AdjacencyList {

    pub fn new() -> AdjacencyList {
        AdjacencyList { vertices: Vec::new()}
    }

    pub fn add_edge(&mut self, src: &usize, dst: &usize) {
        self.vertices[*src].push(*dst);
    }

    pub fn delete_vertex(&mut self, v: &usize) {
        self.vertices[*v] = vec![];
    }

    pub fn get_adjacent_vertices(&self, src: usize) -> &Vec<usize> {
        &self.vertices[src]
    }

}

// FUTURE:
// struct UndirectedGraph {
//     graph: Graph,
// }

// impl UndirectedGraph {

//     pub fn add_edge(&mut self, src: &usize, dst: &usize) {
//         self.graph.add_edge(src, dst);
//         self.graph.add_edge(dst, src);
//     }

// }

#[cfg(test)]
mod tests {
    use super::*;

    fn half_connect_graph(g: &mut Graph) {
        for i in 0..g.vertices.len() {
            let vertex1 = g.vertices[i].id;
            for j in 0..(g.vertices.len() / 2) {
                let vertex2 = g.vertices[j].id;
                g.add_edge(&vertex1, &vertex2);
            }
        }
    }

    #[test]
    fn test_graph() {
        let mut G = Graph::new();
        G.add_vertex();
        G.add_vertex();
        let vertex1 = G.vertices[0].id;
        let vertex2 = G.vertices[1].id;
        G.add_edge(&vertex1, &vertex2);
        assert_eq!(G.count_vertices(), 2);
        G.display();
    }

    #[test]
    fn test_large_clique() {
        let n_vertices = 1000;
        let mut G = Graph::new();
        G.add_vertices(n_vertices).fully_connect();
        assert_eq!(G.count_vertices(), n_vertices);
        assert_eq!(G.count_edges(), n_vertices.pow(2));
    }

    #[test]
    fn test_delete_vertices() {
        let n_vertices = 20;
        let mut G = Graph::new();
        G.add_vertices(n_vertices).fully_connect();
        assert_eq!(G.count_vertices(), n_vertices);
        for i in 0..5 {
            let v_id = G.get_vertex(&i).unwrap().id();
            G.delete_vertex(&v_id);
        }
        assert_eq!(G.count_vertices(), n_vertices - 5);
        G.display();
    }

    #[test]
    fn test_get_adjacent_nodes() {
        let n_vertices = 10;
        let mut G = Graph::new();
        G.add_vertices(n_vertices);
        half_connect_graph(&mut G);
        let v = G.get_vertex(&0).unwrap();
        let truth: Vec<usize> = vec![0, 1, 2, 3, 4];
        let av = G.get_adjacent_vertices(v.id());
        assert_eq!(*av, truth);
    }


}
