
// fn add(n1:i32, n2:i32)-> i32{
//     return n1+n2;
//
// }


use serde::{Serialize, Deserialize};
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{:?}",user1);

    // Use the fields in a meaningful way
    println!("User info:");
    println!("Active: {}", user1.active);
    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Sign-in count: {}", user1.sign_in_count);

    let number = 5;


    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),

        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),

    }
    #[derive(Serialize, Deserialize, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 1, y: 2 };

// Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    println!("{}", serialized);

// Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("X:{}, Y:{}",deserialized.x, deserialized.y);
    // let a: i32=38;
    // let b: i32=4;
    // let sum = add(a,b);
    // let mut sum= a;
    // sum+=b;
    let add =|n1:i32, n2:i32 | n1+n2;
    let mut values=vec![8,3];
    values.append(&mut vec![6,7,4]);
    let mut sum=0;
    for n in &values[0..2]{
        sum= add(sum,*n)
    }
    for n in &values[2..4]{
        sum=add(sum,*n)
    }

    println!("Sum array values is {}",sum);

    let is_even =|val:i32|->bool{
        if val %2==0{
            return true;
        }
        return false;
    };
    let is_odd =|val:i32|->bool{!val%2==0};

    println!("{}",is_even(25));
    println!("Is odd:{}",is_odd(25));

    let sum = fastrand::choose_multiple (1..100,10)
        .into_iter()
        .filter(|n|n%2==0)
        .inspect(|n|print!("{}",n))
        .sum::<i32>();
    println!("={}",sum);

    let numbers =(1..11)
        .filter(|n| *n > 8)
        .inspect(|n|println!("n= {}",n));


// Use `numbers` for further operations, e.g.,
    let sum:i32 = numbers.sum();
    println!("The sum of numbers is: {}", sum);

    let t=(1..13)
        .filter(|n| n%2 ==0)
        .inspect(|n|println!("n= {}",n))
        .fold(0,|tally , n| tally+n);
    println!("sum iterator ={}", t);
}
