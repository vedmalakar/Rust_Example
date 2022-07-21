
  #![allow (unused_variables)]
  #![allow(unused_imports)]

//use core::iter::Scan;

use std::{iter::{Iterator, Scan}, ops::Index};

  fn get_input() -> String {
      let mut buffer = String::new();
      std::io::stdin().read_line(&mut buffer).expect("Failed");
      buffer
  }
  
  
  fn main() {
        
        
        println!("Enter the size of array ");
        let n = get_input().trim().parse::<usize>().unwrap();
      
        println!("Enter the element in the array");		
        let mut v = Vec::with_capacity(n);
        for _ in 0..n  {
                let elem = get_input().trim().parse::<i32>().unwrap();
                v.push(elem)
                }
          
        println!("Enter the target element to search ");
        let target = get_input().trim().parse::<i32>().unwrap();

        let searched_element = linear_search(v, target);
        if searched_element >=0
         {
            println!("Search element fount at index {}", searched_element);
         }
         else 
         {
             println!("search element not found");    
         }
        
        }
  
       fn linear_search(v: Vec<i32>, target: i32) -> i32 
       {
        let mut i:i32 =0;
        
       for num in v 
       {
           println!("value is {}",num);
            
           if num == target
            {
               return i;
            }

             i = i+1;

       }
        
       return -1;
     
     
     }