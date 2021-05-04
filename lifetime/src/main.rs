fn main() {
    let string1 = String::from("long string is long");            //--------- lifetime of string1 start --
                                                                  //                                     |
    {                                                             //                                     |
        let string2 = String::from("xyz");                        //------ lifetime of string2 start --  |
        let result = longest(string1.as_str(), string2.as_str()); //                                  |  |
        println!("The longest string is {}", result);             //------ lifetime of string2 end-----  |
    }                                                             //                                     |
                                                                  //---------- lifetime of string1 end ---
}

//fn main() {
//    this function will not work because result lifetime end before println!
//    let string1 = String::from("long string is long");
//    let result;
//    {
//        let string2 = String::from("xyz");
//        result = longest(string1.as_str(), string2.as_str()); // ----> string2 lifetime end here 
//    }
//    println!("The longest string is {}", result); // -----> compile error because string2 lifetime is out of scope
//}



fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 'a is genetic lifetime anotation, its use to tell lifetime scope of output to rust complier
    // let's say x lifetime goes out of scope before y then the lifetime of output will be the smaller one, x lifetime, so it can not outlive the input that output reference to 
    // for fn main in example above we will get compiler error because string2 lifetime is out of scope and result, output of this function, borrow the lifetime of string2 
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//fn longest(x: &str, y: &str) -> &str {
//   this function will not work because rust complier can not detetermined the lifetime scope of output &str, x or y
//    if x.len() > y.len() {
//        x
//    } else {
//        y
//    }
//}


//fn longest_test<'a>(x: &str, y: &str) -> &'a str {
//   this function will not work either even though we specify the lifetime of output
//   because the lifetime of output have on relation with the lifetime of input and the lifetime of output goes out of scope when function finished, its will create dangling reference
//   so its can not return output, we can fix that by change the output type form reference to any other owned data type
//   let result = String::from("really long string");
//    result.as_str()
//}


// Example Generic Type Parameters, Trait Bounds, and Lifetimes Together
//fn main() {
//    let string1 = String::from("abcd");
//    let string2 = "xyz";
//
//    let result = longest_with_an_announcement(
//        string1.as_str(),
//        string2,
//        "Today is someone's birthday!",
//    );
//    println!("The longest string is {}", result);
//}
//
//use std::fmt::Display;
//
//fn longest_with_an_announcement<'a, T>( // generic type T and generic lifetime 'a
//    x: &'a str,
//    y: &'a str,
//    ann: T,
//) -> &'a str
//where
//    T: Display, // this is trait of genetic type T
//{
//    println!("Announcement! {}", ann);
//    if x.len() > y.len() {
//        x
//    } else {
//        y
//    }
//}