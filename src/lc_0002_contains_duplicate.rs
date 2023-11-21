use std::collections::HashSet;


pub fn contains_duplicate(nums: Vec<i32>) -> bool {
     
    let mut set: HashSet<i32> = HashSet::new();
    
    for i in nums {
        if set.contains(&i) {
            return true;
        }
        set.insert(i);
    }
    false

}





#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1(){
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    }



}
