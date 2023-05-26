pub mod gendercounting{
    use std::collections::HashMap;
    use crate::graph::graph::Graph;

    pub fn percentwomen(g: &Graph,map: &HashMap<String,String>)->(f32,f32){
        let chars=&g.nodes;
        let numchars=chars.len() as f32;
        let mut numcharsnodroids=chars.len() as f32;
        let mut numwomen: f32=0.0;
        for node in chars{
            match map.get(&node.name){
                Some(s)=>match s.as_str(){
                    "f"=>numwomen=numwomen+1.0,
                    "d"=>numcharsnodroids=numcharsnodroids-1.0,
                    _d=>(),
                },
                None=>(),
            }
        }
        let percent=numwomen/numchars;
        let percentnodroids=numwomen/numcharsnodroids;
        return (percent,percentnodroids)

    }

    pub fn interactionpercentages(g:&Graph,map:&HashMap<String,String>)->(f32,f32,f32,f32){
        let links=&g.links;
        let nodes=&g.nodes;
        let mut allinteractions:f32=0.0;
        let mut allwomeninteractions:f32=0.0;
        let mut meninteractions:f32=0.0;
        let mut mminteractions:f32=0.0;
        let mut ffinteractions:f32=0.0;
        for link in links{
            allinteractions=allinteractions+link.value as f32;
            let source=link.source as usize;
            let target=link.target as usize;
            let sourcegender=map.get(&nodes[source].name);
            let targetgender=map.get(&nodes[target].name);
            let tstuple=(sourcegender,targetgender);
            match tstuple{
                (Some(s),Some(t))=>match (s.as_str(),t.as_str()){
                    ("f","f")=>{
                        allwomeninteractions=allwomeninteractions+link.value as f32;
                        ffinteractions=ffinteractions+link.value as f32;
                    },
                    ("m","m")=>{
                        meninteractions=meninteractions+link.value as f32;
                        mminteractions=mminteractions+link.value as f32;
                    },
                    ("f",_d)=>allwomeninteractions=allwomeninteractions+link.value as f32,
                    (_r,"f")=>allwomeninteractions=allwomeninteractions+link.value as f32,
                    ("m",_d)=>meninteractions=meninteractions+link.value as f32,
                    (_r,"m")=>meninteractions=meninteractions+link.value as f32,
                    (_f,_g)=>()
                },
                (None,Some(_x))=>(),
                (Some(_y),None)=>(),
                (None,None)=>(),
            }

        }
        let percent=ffinteractions/allwomeninteractions;
        let percentall=ffinteractions/allinteractions;
        let menpercentall=mminteractions/allinteractions;
        let menpercent=mminteractions/meninteractions;
        return (percentall,percent,menpercentall,menpercent)
    }

    pub fn sepbetweenwomen(g:&Graph,map:&HashMap<String,String>)->(Vec<usize>,Vec<i32>){
        let mut womenvec=Vec::new();
        for i in 0..g.nodes.len(){
            match map.get(&g.nodes[i].name){
                Some(s)=>{
                    match s.as_str(){
                    "f"=>womenvec.push(i),
                    _x=>(),
                    }
                },
                None=>(),
            }
        }

        let adjacency=g.adjacencies();
        let mut degrees:Vec<i32>=Vec::new();
        let mut visited:Vec<bool>=vec![false;g.nodes.len()];
        for woman in 0..womenvec.len(){
            let mut degree=0;
            visited[womenvec[woman]]=true;
            let mut queue=adjacency[womenvec[woman]].clone();
            'outer: while degree<=6{
                for person in queue.clone(){
                    match map.get(&g.nodes[person as usize].name){
                        Some(g)=>match g.as_str(){
                            "f"=>{
                                break 'outer;
                            },
                            _x=>visited[person as usize]=true,
                        },
                        None=>(),
                    }
                }
                degree=degree+1;
                let mut newadjacency=Vec::new();
                for person in queue.clone(){
                    for link in &g.links{
                        if (link.source==person) && (visited[link.target as usize]==false){
                            newadjacency.push(link.target);
                        }
                        else if (link.target==person) && (visited[link.source as usize]==false){
                            newadjacency.push(link.source);
                        }
                    }
                }
                queue=newadjacency;
            }
            visited=vec![false;g.nodes.len()];
            degrees.push(degree);
        }

        return (womenvec,degrees)

    }



}