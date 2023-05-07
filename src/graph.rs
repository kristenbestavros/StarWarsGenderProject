pub mod graph{
    use std::io::prelude::*;
    use std::fs::File;
    use std::collections::HashMap;
    use serde::Deserialize;

    #[derive(Debug,Deserialize)]
    pub struct Node{
        pub name: String,
        pub value: u64,
        pub colour: String,
    }

    #[derive(Debug,Deserialize)]
    pub struct Link{
        pub source: u64,
        pub target: u64,
        pub value: u64,
    }

    #[derive(Debug,Deserialize)]
    pub struct Graph{
        pub nodes: Vec<Node>,
        pub links: Vec<Link>,
    }

    impl Graph{
        pub fn adjacencies(&self)->Vec<Vec<u64>>{
            let mut adjacencies: Vec<Vec<u64>>=Vec::new();
            for node in 0..self.nodes.len(){
                let mut thisvec=Vec::new();
                for link in 0..self.links.len(){
                    if self.links[link].source==node as u64{
                        thisvec.push(self.links[link].target);
                    }
                    else if self.links[link].target==node as u64{
                        thisvec.push(self.links[link].source);
                    }
                }
                adjacencies.push(thisvec);
            }
            return adjacencies
        }
    }


    pub fn read_graph(path: &str) -> Graph {
        let file = File::open(path).expect("Could not open file");
        let json: Graph = serde_json::from_reader(file).expect("file should be proper JSON");
        return json;
    }

    pub fn read_gender_map(path: &str) -> HashMap<String,String>{
        let file = File::open(path).expect("Could not open file");
        let buf_reader = std::io::BufReader::new(file).lines();
        let mut map: HashMap<String,String>=HashMap::new();
        for line in buf_reader{
            let line_str = line.expect("Error reading");
            let v: Vec<&str> = line_str.trim().split(',').collect();
            map.insert(v[0].parse::<String>().unwrap(),v[1].parse::<String>().unwrap());
        }
        return map;
    }

}