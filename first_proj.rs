use std:: io;

fn main()
{
 let  x= [10,23,44,56,78,90,101];
 let mut first =0;
 let mut last = x.len()-1;
 let mut middle = (first+last)/2;
 let mut found = false; 
 let mut input = String::new();

 
 
 io::stdin().read_line(&mut input).expect("Failed to read line");
 let input = input.trim().parse::<u32>().unwrap();

 while first<=last
 {
    middle = (first + last) / 2;  
    if x[middle] == input
    {
        found = true;
        break;
    }
    else if x[middle] < input
    {
        first = middle + 1;
    } 
    else if x[middle] > input
    {
        last = middle - 1;
    }
 }
 if found
 {
 println!("Found {} at index {}", input, middle);
 }
 else
 {
 print!("whigga");
 }
}

