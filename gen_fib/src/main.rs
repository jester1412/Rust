use std::io;

fn main() {
    println!("Enter nth Fibonacci number");
    let mut num = String::new();
    io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");
    let num = num.trim().parse::<usize>().unwrap();
    let fib = gen_fib(num);
    println!("{}th Fibonacci number is {}",num,fib);
}


fn gen_fib(num: usize )-> u128{

    let mut fib = vec![1, 1];
    for i in 2..num {
        let next_fib = fib[i - 1] + fib[i - 2];
        fib.push(next_fib);
    }
    return fib[num-1];

    //match num {
    //    0 => 1,
    //    1 => 1,
    //    _ => gen_fib(num - 1) + gen_fib(num - 2),
    //}
}
