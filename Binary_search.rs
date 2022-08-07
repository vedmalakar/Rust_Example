#![allow (unused_variables)]
#![allow (unused_imports)]
#![allow(non_snake_case)]


use std::{iter::{Iterator,Scan} ,ops::Index };



fn get_input()->String{
    let mut buffer =String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}


fn main()
{
    println!("Enter the size of the Array");
    let n:usize = get_input().trim().parse::<usize>().unwrap();
   
    
    println!("Enter elements in the array with sorted order assending");
    let mut v=Vec::<i32>::with_capacity(n);
    for _ in 0..n{
        let elem = get_input().trim().parse::<i32>().unwrap();
        v.push(elem)
    } 
    
    println!("Enter the target element to search ");
    let target= get_input().trim().parse::<i32>().unwrap();

    let searched_element = Binaryv_search(v,target);
    if searched_element >= 0
    {
        println!("Search element found at index {}",searched_element);
    }

    else 
    {
                println!("Search element not found"); 



    
    }            
}



fn Binaryv_search(v:Vec<i32>,target:i32) -> i32
{
           
    let len_n: usize = v.len();
    let mut l_index: isize = 0;
    let mut r_index: isize = len_n as isize-1;       
    
    while l_index <= r_index
    {
        let m_index =l_index + (r_index -l_index)/2;
           
           if v[m_index as usize] == target
              {return m_index as i32;}
           if v[m_index as usize] < target
                { l_index = m_index + 1;}
        
            
           else
                {r_index = m_index -1;}

    }

    return -1;
}          