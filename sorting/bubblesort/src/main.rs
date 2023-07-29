fn bubblemain(vectosort:Vec<i32>)->Vec<i32>{
    let mut sortedarray=vectosort.clone();
    let mut swapped=false;
    for i in 0..sortedarray.len(){
        for j in 0..sortedarray.len(){
            if sortedarray[i]<sortedarray[j]{
                let oldbefore=sortedarray[i];
                let oldafter=sortedarray[j];
                sortedarray[i]=oldafter;
                sortedarray[j]=oldbefore;
                if !swapped{
                    swapped=true;
                }

            }
            
        }
        if !swapped{
            break;
        }
    }
    sortedarray
    
}


fn main() {
    //let vec2sort=vec![4,2,6,7,9];
    let vec2sort=vec![64, 34, 25, 12, 22, 11, 90];
    let result=bubblemain(vec2sort);

    println!("{:?}", result);

}
