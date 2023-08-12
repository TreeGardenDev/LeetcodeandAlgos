
/*
6 - total genes
a b c aa d - possible value of genes
1 2 3 4 5 6 - health of each gene
3 - number of strands
1 5 caaab -strand first, last and gene
0 4 xyz
2 4 bcdybc
*/

use std::collections::HashMap;

fn main() {
    let n = 6;

    let genes = vec![String::from("a"), String::from("b"), String::from("c"), String::from("aa"), String::from("d")];

    let health = vec![1, 2, 3, 4, 5, 6];

    let s = 3;
    let strands = vec![String::from("caaab"), String::from("xyz"), String::from("bcdybc")];

    let (min, max) = health_of_genes(n, genes, health, s, strands);


}

fn health_of_genes(count_genes:i32, possible_genes:Vec<String>, health:Vec<i32>, n:i32, strands:Vec<String>)-> (i32, i32){
    let mut map = HashMap::new();
    
    for i in 0..count_genes-1 {
        map.insert(possible_genes[i as usize].clone(), health[i as usize]);
    }
    println!("{:?}", map);

    let max = 0;
    let min = 0;
    for i in 0..n-1 {
        let mut health = 0;
        let strand = &strands[i as usize];
        let first = strand.chars().nth(0).unwrap();
        let last = strand.chars().nth(1).unwrap();
        let gene = &strand[2..];
        for j in 0..gene.len()-1 {
            let mut temp = String::new();
            temp.push(gene.chars().nth(j).unwrap());
            temp.push(gene.chars().nth(j+1).unwrap());
            if temp == first.to_string() || temp == last.to_string() {
                health += map.get(&temp).unwrap();
            }
        }
         
        println!("{}", health);
    }
    
    
    (0, 0)

}


