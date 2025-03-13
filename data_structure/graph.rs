#[derive(Debug)]
pub struct Graph{
    nodes: usize,
    map: Vec<Vec<i32>>,
    vertex: Vec<String>,
}

impl Graph {
    pub fn new(nodes: usize) -> Self {
        Graph {
            nodes: nodes,
            map: vec![vec![i32::MAX;nodes];nodes],
            vertex: vec![String::new();nodes],
        }
    }

    pub fn add_edge(&mut self, node1: usize, node2: usize, cost: i32){
        if node1 < self.nodes && node2 < self.nodes {
            self.map[node1][node2] = cost;
            self.map[node2][node1] = cost;
        }
    }
    
    pub fn add_vertex_data(&mut self, vertex: usize, data: &str){
        if vertex < self.nodes{
            self.vertex[vertex] = data.to_string();
        }
    }

    pub fn get_amount_nodes(&self) -> usize{
        return self.nodes;
    }

    pub fn get_edge(&self, node1: usize, node2: usize) -> i32{
        return self.map[node1][node2];
    }

    pub fn get_vertex_data(&mut self, vertex: usize) -> String{
        if vertex < self.nodes{
            return self.vertex[vertex].clone();
        }
        return String::new();
    }

}