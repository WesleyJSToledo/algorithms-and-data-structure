use std::collections::LinkedList;

use crate::graph::Graph;

pub fn dijkstra() {
    let mut graph = Graph::new(7);

    graph.add_vertex_data(0, "book");
    graph.add_vertex_data(1, "rare_vinyl");
    graph.add_vertex_data(2, "poster");
    graph.add_vertex_data(3, "bass");
    graph.add_vertex_data(4, "drums");
    graph.add_vertex_data(5, "piano");

    graph.add_edge(0, 1, 5);
    graph.add_edge(0, 2, 0);
    graph.add_edge(1, 3, 15);
    graph.add_edge(1,4, 20);
    graph.add_edge(2, 3, 30);
    graph.add_edge(2,4, 35);
    graph.add_edge(3,5, 20);
    graph.add_edge(4, 5, 10);

    let start = 0;
    let end = 5;

    let path = dijkstra_logic(&mut graph, start, end);

    println!("{:?}", path);
}

fn dijkstra_logic(mut graph: &mut Graph, start: usize, end: usize) -> Vec<(String, usize)> {
    const INFINITY: i32 = i32::MAX;
    
    let mut dist: Vec<i32> = vec![INFINITY; graph.get_amount_nodes()];
    let mut process: Vec<bool> = vec![false;graph.get_amount_nodes()];
    let mut parent: Vec<i32> = vec![-1; graph.get_amount_nodes()];

    dist[start] = 0;

    for _count in 0..graph.get_amount_nodes(){
        let node = minimum_distance(dist.clone(), process.clone(), graph.get_amount_nodes());
        process[node] = true;

        for i in 0..graph.get_amount_nodes(){
            let current_node = graph.get_edge(node, i);
            if !process[i] && current_node != i32::MAX  && dist[node] != i32::MAX && dist[node] + current_node < dist[i]{
                dist[i] = dist[node]+ current_node;
                parent[i] = node as i32;
            }
        }
    }
    return path(&mut graph, parent, end.try_into().unwrap());
}


fn minimum_distance(dist: Vec<i32>, process: Vec<bool>, nodes: usize) -> usize{
    let mut min_dist = i32::MAX;
    let mut min_index = 0;

    for v in 0..nodes {
        if !process[v] && dist[v] <= min_dist{
            min_dist = dist[v];
            min_index = v;
        }
    }
    return min_index;
}

fn path(graph: &mut Graph, parent: Vec<i32>, end: i32) -> Vec<(String, usize)> {
    let mut path = Vec::new();
    let mut crawl: usize = end as usize;
    let mut list: LinkedList<usize> =  LinkedList::new();

    list.push_back(crawl);
    
    while parent[crawl] != -1 {
        list.push_back(parent[crawl] as usize);
        crawl = parent[crawl] as usize;
    }
    
    if let Some(node) = list.pop_back() {
        path.push((graph.get_vertex_data(node), node));
    }
    
    while let Some(node) = list.pop_back() {
        path.push((graph.get_vertex_data(node), node));
    }
    
    return path;
}