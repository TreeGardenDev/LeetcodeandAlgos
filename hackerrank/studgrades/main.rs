fn main() {
    let grades = vec![73, 67, 38, 33];
    let result = gradingStudents(&grades);

    println!("{:?}", result);
}

fn gradingStudents(grades: &[i32])-> Vec<i32>{
    let mut grady = grades.to_vec();
    for (i, grade) in grades.iter().enumerate() {
        let nextmultiple5 = (grade/5+1)*5;
        if grade>=&38&&nextmultiple5-grade<3{
            grady[i]=nextmultiple5;
        }
        else{
            grady[i]=*grade;
        }
        
    }
    grady
    
}
