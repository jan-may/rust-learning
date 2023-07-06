fn main() {
    let x: i32 = 1234567;
    let y: i32 = 7654321;
    println!("{}", x.abs_diff(y));
    println!("{:?}", x.to_be_bytes());
    println!("{}", x.count_ones());
    println!("{}", x.leading_zeros());
    println!("{:b}", x);
    println!("{:b}", x.reverse_bits());
    println!("{}", 10i32.pow(5));

    //vec
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    assert_eq!(vec.len(), 2);
    println!("{:?}", vec.pop().expect("pop failed"));
    println!("{:?}", vec.len());
    vec[0] = 7;
    vec.push(3);
    println!("{:?}", vec);
    vec.extend([1, 2, 3]);
    println!("{:?}", vec);
    let mut vec2 = vec![0; 5]; // vec![T; size]
    println!("{:?}", vec2);
    vec2.resize(3, 0);
    println!("{:?}", vec2);
    let v = vec![0, 1];
    read_slice(&v);
}

fn read_slice(_slice: &[usize]) {
    // ...
}
