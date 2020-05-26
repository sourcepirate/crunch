//! Implementation of LCS algorithm.
//! Uses Dynamic programming to speed up.
//! 

// runs a longest common substring algorithm 
pub fn lcs(seq_1: String, seq_2: String) -> String {
   let seq_vector_one: Vec<char> = seq_1.chars().collect();
   let seq_vector_two: Vec<char> = seq_2.chars().collect();

   let seq_length_one : usize = seq_vector_one.len();
   let seq_length_two : usize = seq_vector_two.len();
   let mut max_length: usize = 0;
   let mut _ending_index: usize = seq_length_one;

   // now initialize the dynamic table first
   let mut dynamic_table : Vec<Vec<usize>> = Vec::new();
   for _ in 0..seq_length_one {
       let mut vector : Vec<usize> = Vec::new();
       for _ in 0..seq_length_two { 
           vector.push(0)
       }
       dynamic_table.push(vector);
   }

   // now run the substring algorithm

   for i in 1..seq_length_one+1 {
       for j in 1..seq_length_two+1 {
           if seq_vector_one[i-1] == seq_vector_two[j-1] {
               dynamic_table[i][j] = dynamic_table[i-1][j-1] + 1;
               if dynamic_table[i][j] > max_length {
                   max_length = dynamic_table[i][j];
                   _ending_index = i;
               }
           }
       }
   }

   let values : Vec<char> = seq_vector_one[_ending_index - max_length.._ending_index].to_owned();

   values.into_iter().collect()
}


#[cfg(test)]
mod tests {

    use super::lcs;

    #[test]
    fn test_check_common_substring() {
        let s1 = String::from("Hello world");
        let s2 = String::from("Hello my boy");
        let value = lcs(s1, s2);
        assert_eq!(value, "Hello ");
    }
}