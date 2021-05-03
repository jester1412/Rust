use std::ops::Add;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


struct Point<T> {
    x: T,
    y: T,
}

impl<T: PartialEq + Add + Copy> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
    
    fn distance_from_origin(&self) -> T::Output{
        self.x + self.y
    }
    
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let p = Point { x: 5, y: 10 };

    println!("Method p.x = {}", p.x());
    println!("p.x = {}", p.x);
    println!("Method p.y = {}", p.y());
    println!("p.y = {}", p.y);
    println!("Add = {}", p.distance_from_origin());
}