use::collections::HashSet;


pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut n: Vec<i32> = HashSet::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&index) = n.get(&complement) {
            return vec![index as i32, i as i32];
        }
        n.insert(num,i);

    }
    
    vec![]
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1(){
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
    }

    #[test]
    fn ex2(){
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1,2])
    }

    #[test]
    fn ex3(){
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1])
    }
}
