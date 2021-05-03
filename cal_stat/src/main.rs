use std::collections::HashMap;
// TODO : refactor code to not borrow the v parameter
fn main() {
    // TODO?? : try using array instead of vector ( will be much easier )
    // TODO : find out how can format float without add demicals
    let v: Vec<f32> = vec![1.0, 2.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let mean = find_mean(&v);
    println!("{:.2}", mean);  
    let median = find_median(&v);
    println!("{}", median);
    let mode = find_mode(&v);
    println!("{}", mode);
}

fn find_mean(v: &Vec<f32>) -> f32 {
    // Explain : 
    let len = v.len() as f32;
    let mut val: f32 = 0.0;
    for i in v.iter() {
        val += i;
    }
    return val/len;
}

fn find_median(v: &Vec<f32>) -> f32{

    // Explain : Clone v to v_sort becuase when sort_by() the vector is change
    // TODO: need to return the original v ( will solve by passing reference instead of borrow v ?? )
    // DONE: Change Function input to Ref of v instead of v 
    let mut v_sort = v.clone();
    // Can sort without sort_by method ??
    v_sort.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let vlen = v_sort.len()/2;

    return v_sort[vlen];
}

fn find_mode(v: &Vec<f32>) -> f32 {

    // Explain : Create new HashMap
    let mut mode = HashMap::new();
    // Explain : Add Key,Value to HashMap by add each element of v to Key and Value eq 1
    //           and if have duplicate Key, Increment Value by one
    for i in v.iter() {        
        //De-reference mode and i because mode and i are reference and can't calculate
        *mode.entry(*i as i32).or_insert(0) += 1 ;
    }

    // TODO : find the meaning of this 
    // DONE-ish : into_iter is convert paramter to iterator
    //            max_by_key is mapping reference value in results, in this case it's tuple<Key: i32,Value: f32> , to _ and count then return max value
    //            map is self explanatory
    //            expect is for error handling stuff
    let result = mode.into_iter()
    .max_by_key(|&(_, count)| count)
    .map(|(val, _)| val)
    .expect("Cannot compute the mode of zero numbers");
    
    return result as f32;


}
