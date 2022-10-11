fn main() {
    const FILENAME: &str = "sample.tgf";

    println!("Deserializing {}...", FILENAME);
    let g =
        graph::Graph::<String, String>::deserialize_from(FILENAME).expect("couldn't deserialize");
    println!("Deserialization finished!\n");

    println!("Traversing all connectivity components with bfs...");
    for v in g.traverse_bfs() {
        println!(
            "Vertex #{} ({}) is connected with",
            v,
            g.get_vertex_value(v).unwrap()
        );
        for adj in g.get_adjacents(v).unwrap() {
            println!("|- Vertex #{} ({})", adj, g.get_vertex_value(*adj).unwrap());
        }
    }
    println!("Traversing is finished!");
}
