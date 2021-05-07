use std::collections::HashMap;
use std::hash::Hash;
// TODO -> DONE : refactor code to not borrow the v parameter
fn main() {
    // TODO?? : try using array instead of vector ( will be much easier )
    // TODO : find out how can format float without add demicals
    let v: Vec<i32> = vec![1, 2, 2, 3, 4, 5, 6, 7, 8];
    let mean = find_mean(&v);
    println!("{:.2}", mean);  
    let median = find_median(&v);
    println!("{}", median);
    let mode = find_mode(&v);
    println!("{}", mode);
}

fn find_mean(v: &Vec<i32>) -> f32 {
    // Explain : 
    v.iter().sum::<i32>() as f32 / v.len() as f32
}

fn find_median<T>(v: &Vec<T>) -> T
// Refactor : Change f32 Type to Generics Type
where
    T: PartialOrd + Copy
{

    // Explain : Clone v to v_sort becuase when sort_by() the vector is change
    // TODO: need to return the original v ( will solve by passing reference instead of borrow v ?? )
    // DONE: Change Function input to Ref of v instead of v 
    let mut v_sort = v.clone();
    // Can sort without sort_by method ??
    v_sort.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let vlen = v_sort.len()/2;

    return v_sort[vlen];
}

fn find_mode<T>(v: &Vec<T>) -> T 
where
    T: Hash + Eq + Copy
{

    // Explain : Create new HashMap
    let mut mode = HashMap::new();
    // Explain : Add Key,Value to HashMap by add each element of v to Key and Value eq 1
    //           and if have duplicate Key, Increment Value by one
    for i in v.iter() {        
        //De-reference mode and i because mode and i are reference and can't calculate
        *mode.entry(*i).or_insert(0) += 1 ;
    }

    // TODO : find the meaning of this 
    // DONE-ish : into_iter is convert paramter to iterator
    //            max_by_key is mapping reference value in results, in this case it's tuple<Key: i32,Value: f32> , to _ and count then return max value
    //            map is self explanatory
    //            expect is for error handling stuff
    let result = mode.into_iter() //-> make mode into iter
    .max_by_key(|&(_, count)| count) // -> |&(_, count)| is Closure that input is reference of tuple<_,count> and output return count, basically find max count
    .map(|(val, _)| val) // -> Closure Mapping Key
    .expect("Cannot compute the mode of zero numbers");
    
    return result;
}
