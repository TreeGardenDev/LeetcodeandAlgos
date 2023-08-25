pub fn compress(chars: &mut Vec<char>) -> i32 {
    let mut map = std::collections::HashMap::new();
    for i in chars.clone(){
        if map.contains_key(&i){
            let count=map.get(&i).unwrap();

            map.insert(i, *count+1);
            //chars.remove(chars.iter().position(|&r| r == i).unwrap());
        }
        else{
            map.insert(i, 1);
        }
    }
    
    chars.clear();
    let mapvec: Vec<_> = map.iter().collect();
    for val in mapvec{
        
        chars.push(*val.0);
        if *val.1>1{

            let counts= val.1.to_string().chars().collect::<Vec<char>>();
            for n in counts{
                chars.push(n);
            }
        }
    } 
    println!("{:?}", chars);
    return chars.len() as i32;

}
fn main() {
    let mut chars = vec!['a','a','b','b','c','c','c'];
    let mut chars = vec!['a'];
    let mut chars=vec!['a','b','b','b','b','b','b','b','b','b','b','b','b'];
    let mut chars=vec!['a','a','b','b','c','c','c'];
    let result = compress(&mut chars);
    println!("result: {}", result);
}
