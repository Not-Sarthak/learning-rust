// a vector of references to copy types (like &i32) can be dereferenced one by one using *. 
// but what if the inner values are not copy types?
// so we can clone them individually v[2].clone() to get a new vector with the contents of an inner vector
// fn main() {
//     let a = vec![50];
//     let b = vec![100];
//     let c = vec![150];
//     let group: Vec<&Vec<i32>> = vec![&a, &b, &c];
//     println!("{:?}", group[2].clone());
// }

fn main() {
    let a = vec![50, 10, 25];
    let b = vec![100, 400];
    let c = vec![150, 600, 700];

    let group_vec: Vec<&Vec<i32>> = vec![&a, &b, &c];
    
    let append = append_sum(group_vec);
    println!("{:?}", append); // [150, 600, 700, 1450]
}

pub fn append_sum(v: Vec<&Vec<i32>>)->Vec<i32>{
    let mut new_v = v[2].clone();
    
    let mut result = 0;

    for i in v[2]{
        result +=i;
    }

    new_v.push(result);
    new_v
}
