pub mod makedata{
    use std::collections::HashMap;
    use crate::graph::graph::Graph;
    use crate::graph::graph::Node;
    use crate::graph::graph::Link;
    use random_string::generate;
    use rand::Rng;


    pub fn makedata()->(Graph,HashMap<String,String>){
        let mut rng = rand::thread_rng();
        let charset="QWERTYUIOPASDFGHJKLZXCVBNM' -";
        let genders=vec!["f","m","d"];
        let mut namevec=Vec::new();
        for _i in 0..1000{
            namevec.push(generate(rng.gen_range(4..15),charset));
        }
        let mut nodes: Vec<Node>=Vec::new();
        let mut links: Vec<Link>=Vec::new();
        let mut map: HashMap<String,String>=HashMap::new();
        for (idx,rname) in namevec.iter().enumerate(){
            map.insert(String::from(rname),String::from(genders[rng.gen_range(0..3)]));
            let scenes=rng.gen_range(10..100);
            nodes.push(Node{name:rname.to_string(),value:rng.gen_range(10..100),colour:rname.to_string()});
            for _i in 0..rng.gen_range(3..10){
                links.push(Link{source:idx as u64,target:rng.gen_range(idx..1000) as u64,value:rng.gen_range(1..scenes)});
            }
        }
        return (Graph{nodes:nodes,links:links},map)

    }

}