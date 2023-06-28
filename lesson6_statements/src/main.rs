fn main() {
    let x = 10;

    let y = {
        let x_square = x * x;
        let x_cube = x_square * x;

        //this expression will be assigned to 'y'
        x_cube + x_square + x
    };

    let z = {
        // the semicolon suppresses this expression and '()' is assigned to 'z '
        2 * x; // <---- ;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

// 1
    let v = {
        let mut x = 1;
        x += 2
    };
 
    assert_eq!(v, ());
 
    println!("Success!");

//2
    let v = 3;

    assert!(v == 3);

    println!("Success!");
//3
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");


    fn sum(x: i32, y: i32) -> i32 {
    x + y
}

// functions 1
    {
        let (x, y) = (1, 2);
        let s = sum(x, y);

        let ans = assert_eq!(
            s, 3,
        f!("Answer is {s}"),
    );

    }
}
