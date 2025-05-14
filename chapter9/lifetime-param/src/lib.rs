/// LifeTime as struct member
///
/// 任意の生存期間'eltに対して、生存期間があ;eltの参照を保持するExtrema<'elt>を作ることができる
///
/// # Examples
struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least {
            least = &slice[i];
        }
        if slice[i] > *greatest {
            greatest = &slice[i]
        }
    }
    Extrema { greatest, least }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn it_work_find_extreama() {
        let a = [0, -3, 0, 15, 48];
        let e = find_extrema(&a);
        assert_eq!(*e.least, -3);
        assert_eq!(*e.greatest, 48);
    }
}
