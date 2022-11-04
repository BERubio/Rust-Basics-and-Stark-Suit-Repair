use std::convert::TryInto;
/**
    Returns the sum 1 + 2 + ... + n
    If n is less than 0, return -1
**/
pub fn gauss(n: i32) -> i32 {
 if n < 0 {-1}
 else{
  let x = n*(n+1);
  let result = x/2;
  return result;
 }
}

/**
    Returns the number of elements in the list that 
    are in the range [s,e]
**/
pub fn in_range(ls: &[i32], s: i32, e: i32) -> i32 {
  let mut count = 0;
  for (i, elem) in ls.iter().enumerate(){
   let check = elem;
   if elem >= &s && elem <= &e{
   count = count+1;
   }
  }
  return count;
}

/**
    Returns true if target is a subset of set, false otherwise

    Ex: [1,3,2] is a subset of [1,2,3,4,5]
**/
pub fn subset<T: PartialEq>(set: &[T], target: &[T]) -> bool {
   let target_length = target.len();
   let mut count = 0;
   for(i, elem) in target.iter().enumerate(){
    let curr = elem;
    for(j, ele) in set.iter().enumerate(){
     let check = ele;
     if curr == check{count = count+1;}
    }
   }
   if target.len() == 0 {return true;}
   if count == target_length {true} else {false}
   
}

/**
    Returns the mean of elements in ls. If the list is empty, return None
    It might be helpful to use the fold method of the Iterator trait
**/
pub fn mean(ls: &[f64]) -> Option<f64> {
    if ls.len() == 0 {return None;}
    else{
     let mut sum: f64 = 0.0;
     let length: f64 = ls.len() as f64;
     for (i, elem) in ls.iter().enumerate(){
       let curr = elem;
       sum = sum + elem;
     }
     let avg: f64 = sum/length;
     return Some(avg);
    }
}

/**
    Converts a binary number to decimal, where each bit is stored in order in the array
    
    Ex: to_decimal of [1,0,1,0] returns 10
**/
pub fn to_decimal(ls: &[i32]) -> i32 {
    let mut dec = 0;
    let mut count = 1;
    let base: i32 = 2;
    let check: i32 = 1;
    let length = ls.len() as i32;
    for (i, elem) in ls.iter().enumerate(){
      if elem == &check {
        let curr = base.pow((length-(count)).try_into().unwrap());
        dec = dec + curr;
      }
      count = count +1;
   }
   return dec;
}

/**
    Decomposes an integer into its prime factors and returns them in a vector
    You can assume factorize will never be passed anything less than 2

    Ex: factorize of 36 should return [2,2,3,3] since 36 = 2 * 2 * 3 * 3
**/
pub fn factorize(n: u32) -> Vec<u32> {
 let mut factors = vec![];
 let mut factor = 2;
 let mut num = n;
 while num > 1 {
   while num % factor == 0 {
      factors.push(factor);
      num = num/factor;
   }
   factor = if factor == 2 { 3 } else { factor + 2 };
 }
 return factors;
}

/** 
    Takes all of the elements of the given slice and creates a new vector.
    The new vector takes all the elements of the original and rotates them, 
    so the first becomes the last, the second becomes first, and so on.
    
    EX: rotate [1,2,3,4] returns [2,3,4,1]
**/
pub fn rotate(lst: &[i32]) -> Vec<i32> {
  let mut result = lst.to_vec();
  if lst.len() == 0 { return result;}
  result.rotate_left(1);
  return result;
}

/**
    Returns true if target is a subtring of s, false otherwise
    You should not use the contains function of the string library in your implementation
    
    Ex: "ace" is a substring of "rustacean"
**/
pub fn substr(s: &String, target: &str) -> bool {
  let s_length = s.len();
  let t_length = target.len();
  if s_length < t_length { return false }
  else if s_length == t_length {
      if s == target { return true } else { return false }
  }
  else {
      for i in 0..(s_length - t_length) {
        if &s[i..(i+t_length)] == target { return true }
      }
      return false
  }
}

/**
    Takes a string and returns the first longest substring of consecutive equal characters

    EX: longest_sequence of "ababbba" is Some("bbb")
    EX: longest_sequence of "aaabbb" is Some("aaa")
    EX: longest_sequence of "xyz" is Some("x")
    EX: longest_sequence of "" is None
**/
pub fn longest_sequence(s: &str) -> Option<&str> {
 if s.len() == 0 { return None }
 let mut curr = s.chars().nth(0);
 let mut count = 0;
 let mut st = 0;
 let mut max = 0;
 let mut max_st = 0;

 for i in 0..s.len() {
   let x = s.chars().nth(i);
   if x == curr { count += 1; }
   else {
       if count > max {
         max = count;
         max_st = st;
       }
       curr = x.clone();
       count = 1;
       st = i;
   }
 }
 if count > max {
    max = count;
    max_st = st;
 }
 Some(&s[max_st..(max_st + max)])
}
