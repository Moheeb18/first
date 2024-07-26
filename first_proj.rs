use std:: io;

fn main()
{
 let mut x :[u32;7] = [10,23,44,56,78,90,101];
 let mut first : u32;
 let mut last :u32;
 let mut middle : u32;
 let found = false; 
 let input = String::new();

 
 print!("Enter the number: ");
 io::stdin().read_line(&mut input).expect("Failed to read line");
 let input = input.trim().parse::<u32>().unwrap();

 while !found
 {
    first = 0;
    last = x.len() as u32 - 1;
    middle = (first + last) / 2;
    if x[middle] == input
    {
        found = true;
    }
    else if x[middle] < input
    {
        first = middle + 1;
    } 
    else
    {
        last = middle - 1;
    }
 }
 println!("Found {} at index {}", input, middle);
}

