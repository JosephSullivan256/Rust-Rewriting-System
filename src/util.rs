#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial(){
        assert_eq!(4*3*2*1,factorial(4));
    }

    #[test]
    fn test_permutation(){
        assert_eq!(5*4,permutation(5,2));
    }

    #[test]
    fn test_combination(){
        assert_eq!(5*4*3/(3*2*1),combination(5,3));
    }
}

pub fn factorial(n: u32) -> u32 {
    let mut k = 1;
    for i in 1..=n {
        k = k*i;
    }
    k
}

pub fn permutation(n: u32,r: u32) -> u32 {
    let mut p = 1;
    for i in (n-r+1)..=n {
        p*=i;
    }
    p
}

pub fn combination(n: u32,r: u32) -> u32 {
    permutation(n,r)/factorial(r)
}