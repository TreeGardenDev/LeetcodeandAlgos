fn main() {
    let candidates = vec![2,3,6,7];
    let target = 7;
    let result=combination_sum(candidates, target);

    println!("Result: {:?}", result);
}

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut returnvec:Vec<Vec<i32>>=Vec::new();
    let mut sum:i32=0;
    let mut index:usize=0;
    let mut tempsum:i32=0;
    let mut tempvec:Vec<i32>=Vec::new();
    
    let mut orderedvec=candidates.clone();
    for i in 0..orderedvec.len() {
        if orderedvec[i]>target {
            orderedvec.remove(i);
        }
        for j in i..orderedvec.len() {
            if orderedvec[i]>orderedvec[j] {
                let temp=orderedvec[i];
                orderedvec[i]=orderedvec[j];
                orderedvec[j]=temp;
            }
        }
    }


    
    returnvec

}
