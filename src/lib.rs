use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Write};
use std::fs;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn add_nodes() {
        let mut g = Graph::<String, u32>::new();
        assert_eq!(g.insert_node(1, "the first".to_string()), None);
        assert_eq!(g.insert_node(2, "the second".to_string()), None);
        assert_eq!(
            g.insert_node(1, "the first_overrided".to_string()),
            Some("the first".to_string())
        );
        let mut sorted_vertices = g.traverse_bfs();
        sorted_vertices.sort();
        assert_eq!(sorted_vertices, vec![1, 2]);
    }

    #[test]
    fn add_edges() {
        let mut g = Graph::<u32, u32>::new();
        g.insert_node(1, 1);
        g.insert_node(2, 2);
        g.insert_node(3, 3);
        assert_eq!(g.insert_edge(OrientedEdge(1, 3), 5), None);
        assert_eq!(g.insert_edge(OrientedEdge(1, 2), 3), None);
        assert_eq!(g.insert_edge(OrientedEdge(5, 3), 5), None);
        assert_eq!(g.insert_edge(OrientedEdge(1, 3), 7), Some(5));
        assert_eq!(g.insert_edge(OrientedEdge(3, 1), 3), None);
        let mut adjacents = g.get_adjacents(1).unwrap();
        adjacents.sort();
        assert_eq!(adjacents, vec![&2, &3]);
    }

    #[test]
    fn remove_nodes() {
        let mut g = Graph::<u32, u32>::new();
        g.insert_node(1, 1);
        g.insert_node(2, 2);
        g.insert_node(3, 3);
        g.insert_edge(OrientedEdge(1, 3), 5);
        g.insert_edge(OrientedEdge(1, 2), 3);
        g.insert_edge(OrientedEdge(3, 1), 3);
        g.remove_node(1);
        let mut adjacents = g.get_adjacents(3).unwrap();
        adjacents.sort();
        assert_eq!(adjacents, Vec::<&u32>::new());
        assert_eq!(g.get_adjacents(1), None);
    }

    #[test]
    fn remove_edges() {
        let mut g = Graph::<u32, u32>::new();
        g.insert_node(1, 1);
        g.insert_node(2, 2);
        g.insert_node(3, 3);
        g.insert_edge(OrientedEdge(1, 3), 5);
        g.insert_edge(OrientedEdge(1, 2), 3);
        g.insert_edge(OrientedEdge(3, 1), 3);
        g.remove_edge(OrientedEdge(1, 3));
        let mut adjacents = g.get_adjacents(1).unwrap();
        adjacents.sort();
        assert_eq!(adjacents, vec![&2]);
    }
}

pub type VertexId = u32;
pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct OrientedEdge(pub VertexId, pub VertexId);

pub struct Graph<V, E> {
    adj_list: HashMap<VertexId, HashMap<VertexId, E>>,
    vertices: HashMap<VertexId, V>,
}

impl<V: Display + FromStr, E: Display + FromStr> Graph<V, E> {
    pub fn new() -> Graph<V, E> {
        Graph {
            adj_list: HashMap::new(),
            vertices: HashMap::new(),
        }
    }

    pub fn serialize_to(&self, filename: &str) -> Result<(), GenericError> {
        let mut tgf = String::new();
        for (v_id, v_value) in self.vertices.iter() {
            write!(tgf, "{} {}\n", v_id.to_string(), v_value.to_string())?;
        }
        tgf += "#\n";
        for (v_from, v_map) in self.adj_list.iter() {
            for (v_to, e_value) in v_map.iter() {
                write!(
                    tgf,
                    "{} {} {}\n",
                    v_from.to_string(),
                    v_to.to_string(),
                    e_value.to_string()
                )?;
            }
        }
        fs::write(filename, &tgf)?;
        Ok(())
    }

    pub fn deserialize_from(filename: &str) -> Result<Graph<V, E>, GenericError>
    where
        <V as FromStr>::Err: std::error::Error + Send + Sync + 'static,
        <E as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {
        let input = fs::read_to_string(filename)?;
        let (mut vertices, mut edges) = input.split_once('#').ok_or("# is missing")?;
        vertices = vertices.trim();
        edges = edges.trim();

        let mut g = Graph::new();
        for line in vertices.lines() {
            let (id, val) = line
                .split_once(' ')
                .ok_or("value for each vertex is required")?;
            g.insert_node(id.trim().parse::<VertexId>()?, val.trim().parse::<V>()?);
        }
        for line in edges.lines() {
            let (from, suffix) = line.split_once(' ').ok_or("vertex_to is missing")?;
            let (to, value) = suffix.split_once(' ').ok_or("edge value is missing")?;
            g.insert_edge(
                OrientedEdge(
                    from.trim().parse::<VertexId>()?,
                    to.trim().parse::<VertexId>()?,
                ),
                value.trim().parse::<E>()?,
            );
        }
        Ok(g)
    }

    pub fn insert_node(&mut self, vertex_id: VertexId, value: V) -> Option<V> {
        self.adj_list.entry(vertex_id).or_insert(HashMap::new());
        self.vertices.insert(vertex_id, value)
    }

    pub fn remove_node(&mut self, vertex_id: VertexId) -> Option<V> {
        // remove edges that point to the removing vertex
        for map in self.adj_list.values_mut() {
            map.remove(&vertex_id);
        }
        self.adj_list.remove(&vertex_id);
        self.vertices.remove(&vertex_id)
    }

    pub fn insert_edge(&mut self, edge: OrientedEdge, value: E) -> Option<E> {
        self.adj_list.get_mut(&edge.0)?.insert(edge.1, value)
    }

    pub fn remove_edge(&mut self, edge: OrientedEdge) -> Option<E> {
        self.adj_list.get_mut(&edge.0)?.remove(&edge.1)
    }

    pub fn traverse_bfs(&self) -> Vec<VertexId> {
        let mut traverse = Vec::new();
        let mut queue = VecDeque::new();
        let mut used = HashSet::new();

        for start_vertex in self.vertices.keys() {
            if !used.contains(start_vertex) {
                queue.push_back(*start_vertex);
                used.insert(*start_vertex);
                while !queue.is_empty() {
                    let current_vertex = queue.pop_front().unwrap();
                    traverse.push(current_vertex);
                    for adjacent in self.adj_list.get(&current_vertex).unwrap().keys() {
                        if !used.contains(adjacent) {
                            queue.push_back(*adjacent);
                            used.insert(*adjacent);
                        }
                    }
                }
            }
        }
        traverse
    }

    pub fn get_adjacents(&self, vertex: VertexId) -> Option<Vec<&VertexId>> {
        Some(self.adj_list.get(&vertex)?.keys().collect())
    }

    pub fn get_vertex_value(&self, vertex: VertexId) -> Option<&V> {
        Some(self.vertices.get(&vertex)?)
    }
}
