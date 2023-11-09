struct Message{
    id:u32,
    msg:String,
}

fn main() {
    println!("Vector data structure!");
    let mut vec = vec![1, 2, 3, 4];
    let mut vec_string: Vec<String> = Vec::new();
    let mut vec_struct: Vec<Message> = Vec::new();
    //vec.push(1);
    //vec.push(2);
    //vec.push(3);
    //vec.pop();
    //assert_eq!(vec.len(), 2);
    //assert_eq!(vec[0], 1);
    vec_string.push(String::from("v1"));
    vec_string.push(String::from("v2"));

    println!("size: {}", vec.len());
    println!("position 0: {}", vec[0]);

    for x in &vec {
        println!("{x}");
    }

    for y in &vec_string {
        println!("{y}");
    }

    /*
    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);

    vec[0] = 7;
    assert_eq!(vec[0], 7);

    vec.extend([1, 2, 3]);

    for x in &vec {
        println!("{x}");
    }
    assert_eq!(vec, [7, 1, 2, 3]);
    */
}
