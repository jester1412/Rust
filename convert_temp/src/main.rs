use std::io;
use std::process;

fn main() {
    
    println!("Convert Tempareture Program [ Celsius and Fahrenheit ]");

    let mut temp_t = String::new();
    let mut temp = String::new();

    println!("Chose Type to Convert to");
    println!("1. Celsius");
    println!("2. Fahrenheit");
    println!("Your Choice : ");
    io::stdin()
            .read_line(&mut temp_t)
            .expect("Failed to read line");

    println!("Enter Temperature : ");

    io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

    let temp_t = temp_t.trim().parse::<i32>().unwrap();
    let temp = temp.trim().parse::<f32>().unwrap();

    let (a,b) =  convert(temp_t,temp);
    println!("Your Temperatue is : {} Degree {}", b,a);

}

fn convert(temp_option: i32,temp: f32) -> (String,f32) {
    //let mut temp_convert: f32;
    //let mut temp_t = String::new();
    
    if temp_option == 1{
        let temp_convert = (temp - 32.0) * 0.5556;
        let temp_t = String::from("Celsius"); 
        (temp_t,temp_convert)
    }
    else if temp_option == 2{
        let temp_convert = (temp * 1.8) + 32.0;
        let temp_t = String::from("Fahrenheit");
        (temp_t,temp_convert)
    }
    else {
        println!("Wrong Temperature Choice!!!!");
        process::exit(1);
    }

   
}
