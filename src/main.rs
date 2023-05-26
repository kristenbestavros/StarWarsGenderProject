use crate::gendercounting::gendercounting::{percentwomen, interactionpercentages,sepbetweenwomen};
use crate::makedata::makedata::{makedata};
use std::time::{Instant};
mod graph;
mod gendercounting;
mod makedata;
pub use crate::graph::graph::Graph;
pub use crate::graph::graph::Link;
pub use crate::graph::graph::Node;


#[cfg(test)]
mod tests {
    //use crate::graph::graph::read_graph;

    use std::collections::HashMap;

    use super::*;

    #[test]
    fn namesmatch() {
        
        let episodes=vec!(
            "starwars-episode-1-interactions-allCharacters.json","starwars-episode-2-interactions-allCharacters.json","starwars-episode-3-interactions-allCharacters.json",
            "starwars-episode-4-interactions-allCharacters.json","starwars-episode-5-interactions-allCharacters.json","starwars-episode-6-interactions-allCharacters.json",
            "starwars-episode-7-interactions-allCharacters.json",
        );
        let genders=graph::graph::read_gender_map("starwars_genders.csv");
        for movie in episodes{
            let graph=graph::graph::read_graph(movie);
            let nodes=graph.nodes;
            for node in nodes{
                match genders.get(&node.name){
                    Some(_s)=>assert_eq!(1,1),
                    None=>assert_eq!(0,1),
                }
            }
        }
        let (graph,genders)=makedata();
        let nodes=graph.nodes;
        for node in nodes{
            match genders.get(&node.name){
                Some(_s)=>assert_eq!(1,1),
                None=>assert_eq!(0,1),
            }
        }
    }

    #[test]
    fn checkcalcs(){
        let graph=Graph{
            nodes:vec![Node{name:"boy1".to_string(),value:2,colour:"".to_string()},Node{name:"boy2".to_string(),value:2,colour:"".to_string()},
                        Node{name:"boy3".to_string(),value:2,colour:"".to_string()},Node{name:"girl1".to_string(),value:2,colour:"".to_string()},
                        Node{name:"girl2".to_string(),value:2,colour:"".to_string()},Node{name:"girl3".to_string(),value:2,colour:"".to_string()}],
            links:vec![Link{source:0,target:1,value:1},Link{source:0,target:2,value:1},Link{source:0,target:3,value:1},Link{source:4,target:5,value:1},
            Link{source:4,target:1,value:1}],
        };
        let mut genders=HashMap::new();
        genders.insert("boy1".to_string(), "m".to_string());
        genders.insert("boy2".to_string(), "m".to_string());
        genders.insert("boy3".to_string(), "m".to_string());
        genders.insert("girl1".to_string(), "f".to_string());
        genders.insert("girl2".to_string(), "f".to_string());
        genders.insert("girl3".to_string(), "f".to_string());
        let (women,sep)=sepbetweenwomen(&graph, &genders);
        
        let interactionpercents=interactionpercentages(&graph, &genders);
        let womenpercents=percentwomen(&graph, &genders);

        assert_eq!(women[0],3);
        assert_eq!(women[1],4);
        assert_eq!(women[2],5);
        assert_eq!(sep[0],2);
        assert_eq!(sep[1],0);
        assert_eq!(sep[2],0);
        assert_eq!(interactionpercents.0,0.2);
        assert_eq!(interactionpercents.1,(1.0/3.0));
        assert_eq!(interactionpercents.2,0.4);
        assert_eq!(interactionpercents.3,0.5);
        assert_eq!(womenpercents.0,0.5);
        assert_eq!(womenpercents.1,0.5);
        
    }
    


}


fn main() {

    let prequels=vec!("starwars-episode-1-interactions-allCharacters.json","starwars-episode-2-interactions-allCharacters.json",
                                "starwars-episode-3-interactions-allCharacters.json");
    let originals=vec!("starwars-episode-4-interactions-allCharacters.json","starwars-episode-5-interactions-allCharacters.json",
                                "starwars-episode-6-interactions-allCharacters.json");

    let genders=graph::graph::read_gender_map("starwars_genders.csv");
    let mut episodenum=1;
    let mut ffall:f32=0.0;
    let mut ffwomen:f32=0.0;
    let mut mmmen:f32=0.0;
    let mut mmall:f32=0.0;
    let mut pctwomen:f32=0.0;
    let mut womennodroids:f32=0.0;

    for movie in prequels{
        let graph=graph::graph::read_graph(movie);
        let (women,seps)=sepbetweenwomen(&graph, &genders);
        println!("EPISODE {} - DEGREE OF SEPERATION BETWEEN WOMEN",episodenum);
        for i in 0..women.len(){
            println!("\t{} - {}",graph.nodes[women[i]].name,seps[i]);
        }
        println!("");
        let interactiontuple=&interactionpercentages(&graph, &genders);
        ffall=ffall+interactiontuple.0;
        ffwomen=ffwomen+interactiontuple.1;
        mmall=mmall+interactiontuple.2;
        mmmen=mmmen+interactiontuple.3;
        let percenttuple=&percentwomen(&graph, &genders);
        pctwomen=pctwomen+percenttuple.0;
        womennodroids=womennodroids+percenttuple.1;

        episodenum=episodenum+1;
    }
    println!("PREQUEL TRILOGY STATS");
    println!("\tPercentage of female characters among all characters: {}",pctwomen/3.0);
    println!("\tPercentage of female characters among all non-droid characters: {}",womennodroids/3.0);
    println!("\tPercentage of interactions that are between two women among all interactions: {}",ffall/3.0);
    println!("\tPercentage of interactions that are between two women among female interactions: {}",ffwomen/3.0);
    println!("\tPercentage of interactions that are between two men among all interactions: {}",mmall/3.0);
    println!("\tPercentage of interactions that are between two men among male interactions: {}",mmmen/3.0);
    println!("");
    
    let mut ffall:f32=0.0;
    let mut ffwomen:f32=0.0;
    let mut mmmen:f32=0.0;
    let mut mmall:f32=0.0;
    let mut pctwomen:f32=0.0;
    let mut womennodroids:f32=0.0;

    for movie in originals{
        let graph=graph::graph::read_graph(movie);
        let (women,seps)=sepbetweenwomen(&graph, &genders);
        println!("EPISODE {} - DEGREE OF SEPERATION BETWEEN WOMEN",episodenum);
        for i in 0..women.len(){
            println!("\t{} - {}",graph.nodes[women[i]].name,seps[i]);
        }
        println!("");
        let interactiontuple=&interactionpercentages(&graph, &genders);
        ffall=ffall+interactiontuple.0;
        ffwomen=ffwomen+interactiontuple.1;
        mmall=mmall+interactiontuple.2;
        mmmen=mmmen+interactiontuple.3;
        let percenttuple=&percentwomen(&graph, &genders);
        pctwomen=pctwomen+percenttuple.0;
        womennodroids=womennodroids+percenttuple.1;

        episodenum=episodenum+1;
    }
    println!("ORIGINAL TRILOGY STATS");
    println!("\tPercentage of female characters among all characters: {}",pctwomen/3.0);
    println!("\tPercentage of female characters among all non-droid characters: {}",womennodroids/3.0);
    println!("\tPercentage of interactions that are between two women among all interactions: {}",ffall/3.0);
    println!("\tPercentage of interactions that are between two women among female interactions: {}",ffwomen/3.0);
    println!("\tPercentage of interactions that are between two men among all interactions: {}",mmall/3.0);
    println!("\tPercentage of interactions that are between two men among male interactions: {}",mmmen/3.0);
    println!("");

    let graph=graph::graph::read_graph("starwars-episode-7-interactions-allCharacters.json");
    

    //let graph=graph::graph::read_graph("starwars-full-interactions-allCharacters.json");
    //let (graph,genders)=makedata();
    let (women,seps)=sepbetweenwomen(&graph, &genders);
    println!("EPISODE {} - DEGREE OF SEPERATION BETWEEN WOMEN",episodenum);
    for i in 0..women.len(){
        println!("\t{} - {}",graph.nodes[women[i]].name,seps[i]);
    }

    let interactiontuple=&interactionpercentages(&graph, &genders);
    let percenttuple=&percentwomen(&graph, &genders);
    
    println!("");
    println!("EPISODE 7 STATS");
    println!("\tPercent female characters: {}",percenttuple.0);
    println!("\tPercent female characters no droids: {}",percenttuple.1);
    println!("\tPercentage of female interactions that are between two women: {}",interactiontuple.0);
    println!("\tPercentage of all interactions between two women: {}",interactiontuple.1);
    println!("\tPercentage of male interactions that are between two men:{}",interactiontuple.2);
    println!("\tPercentage of all interactions that are between two men:{}",interactiontuple.3);
    println!("");

    let (graph,genders)=makedata();
    let start = Instant::now();
    sepbetweenwomen(&graph, &genders);
    let duration = start.elapsed();
    let interactionpercents=interactionpercentages(&graph, &genders);
    let womenpercents=percentwomen(&graph, &genders);
    println!("GENERATED DATA");
    println!("\tExample of node name and getting gender: name - {}, gender - {}",graph.nodes[0].name,genders[&graph.nodes[0].name]);
    println!("\tTime to calculate degrees of seperation: {:?}",duration);
    println!("\tPercent female characters: {}",womenpercents.0);
    println!("\tPercent female characters no droids: {}",womenpercents.1);
    println!("\tPercentage of female interactions that are between two women: {}",interactionpercents.0);
    println!("\tPercentage of all interactions that are between two women: {}",interactionpercents.1);
    println!("\tPercentage of male interactions that are between two men:{}",interactionpercents.2);
    println!("\tPercentage of all interactions that are between two men:{}",interactionpercents.3);




    
}