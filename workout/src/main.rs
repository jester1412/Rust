use std::thread;
use std::time::Duration;

struct Cacher<T,K>
where
    // Closure Trait Fn
    T: Fn(K) -> K,
    K: Copy,
{
    calculation: T,
    // value need to be option because we can have value or no value
    value: Option<K>,
}

impl<T,K> Cacher<T,K>
where
    T: Fn(K) -> K,
    K: Copy
{
    // Generate Empty Struct Cacher
    fn new(calculation: T) -> Cacher<T,K> {
        Cacher {
            calculation,
            value: None,
        }
    }
    // With this value logic we cannot have 2 differnce value because when we have some value it will return that value
    // and if we have 2 value it will return only first value in logic
    // TODO : Fix value logic to store more than 1 value, use Hashmap
    // Done : Fix value type to generic type
    fn value(&mut self, arg: K) -> K {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


// Refactor
//      Before : use simulated_expensive_calculation as a function
//      After : use simulated_expensive_calculation as a closure, expensive_result
//fn simulated_expensive_calculation(intensity: u32) -> u32 {
//    println!("calculating slowly...");
//    thread::sleep(Duration::from_secs(2));
//    intensity
//}

fn generate_workout(intensity: u32, random_number: u32) {
    
    // Refactor 
    //      Before : Call simulated_expensive_calculation at most twice and at least once
    //      After : Call simulated_expensive_calculation only once
    //let expensive_result = simulated_expensive_calculation(intensity);

    // Refactor
    //      Before : Store Result from Function call simulated_expensive_calculation in expensive_result
    //      After : Store Closure in expensive_result, So its will not execute logic if expensive_result do not get call, in case of random_number = 3
    //              but we still call its twice in firse loop
    /*let expensive_result = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    */

    /* Closure example
    fn  add_one_v1   (x: u32) -> u32 { x + 1 } --> Regular function
    let add_one_v2 = |x: u32| -> u32 { x + 1 };--> Closure with annotation
    let add_one_v3 = |x|             { x + 1 };--> Closure without annotation, if add_one_v3 get call 2 time with difference input type we will have complie error
        Ex: 
            let example_closure = |x| x;
            let s = example_closure(String::from("hello"));
            let n = example_closure(5);
        Error:
            |
            |     let n = example_closure(5);
            |                             ^
            |                             |
            |                             expected struct `String`, found integer
            |                             help: try using a conversion method: `5.to_string()`
    
    let add_one_v4 = |x|               x + 1  ;--> Closure with only one expression, do not have to add curly brackets
    */

    /* Refactor
            Before : Use Regular Closure
            After : implement Closure with Generic Type and Fn trait 
                    with this we will only call expensive_result closure logic only once and only when we need to
    */
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });


    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }

    /* Refactor Code
            simulated_expensive_calculation is expensive fn so instead of call it everytime in loop. we change to call it only once before loop
    if intensity < 25 {
       println!(
           "Today, do {} pushups!",
           simulated_expensive_calculation(intensity)
       );
       println!(
           "Next, do {} situps!",
           simulated_expensive_calculation(intensity)
       );
    } else {
       if random_number == 3 {
           println!("Take a break today! Remember to stay hydrated!");
       } else {
           println!(
               "Today, run for {} minutes!",
               simulated_expensive_calculation(intensity)
           );
       }
    }
    */
}


fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}