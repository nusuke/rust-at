use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

// https://atcoder.jp/contests/abc297/tasks/abc297_d
#[allow(dead_code)]
pub fn main() {
    let input = {
        let mut s1 = String::new();
        std::io::stdin().read_line(&mut s1).unwrap();
        std::io::stdin().read_line(&mut s1).unwrap();
        s1.trim_end().to_owned()
    };

    let res = answer(input);

    println!("{}", res);
}

fn answer(input: String) -> String {
    let (n, k, a) = {
        let mut ws = input.split_whitespace();
        let n: usize = ws.next().unwrap().parse().unwrap();
        let m: usize = ws.next().unwrap().parse().unwrap();
        let mut a = ws.map(|s| s.parse().unwrap()).collect::<Vec<usize>>();
        a.sort();
        (n, m, a)
    };

    if a.last().unwrap() == &1 {
        return k.to_string();
    };

    let mut ans: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    let mut used = HashSet::new();

    ans.push(Reverse(0));

    for _i in 0..k {
        let pop = ans.pop().unwrap().0;
        for ni in 0..n {
            let res = pop + a[ni];
            if !used.contains(&res) {
                ans.push(Reverse(res));
                used.insert(res);
            }
        }
    }

    return ans.pop().unwrap().0.to_string();
}

#[cfg(test)]
mod tests {
    use crate::abc297::e::answer;

    #[test]
    fn e_1() {
        let input = "4 6\n20 25 30 100".to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "50");
    }

    #[test]
    fn e_1_2() {
        let input = "4 7\n20 25 30 100".to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "55");
    }

    #[test]
    fn e_2() {
        let input = "2 10\n2 1".to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "10");
    }

    #[test]
    fn e_3() {
        let input = "10 200000\n955277671 764071525 871653439 819642859 703677532 515827892 127889502 881462887 330802980 503797872".to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "5705443819");
    }

    #[test]
    fn e_4() {
        let input = "3 9\n3 13 22".to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "19");
    }
}

/*
a=[3,13,22]
3 | 3,6,9,12,15,18,21,24,27,30,33,36,39,42,45,48,51
13| 13,26,39,52 (3を基準に1/4)
22| 22,44,66 (3を基準に/7)

ans = 3,6,9,12,13,15,16,18,19,21,22,24,26,27,30,33,36,39(2),42,44,45....

i<a.lengthと置く。
つまり、a[0]を基準とすると、a[i]はa[i]/a[0]に一回挟まる。

j <  ans.length
j=9では、
13が5番目
22が9番目に挟まる (8番目に挟まるはずが、13が1度挟まっているため)

9*a[0] / a[1]  =  2
↑(27/13)
9*a[0]/ a[2] = 1

なので、ansには3(2+1)回a[0]以外の結果が入り込む。これをyとする。
3(a[0]) * 9(j) - 3(a[0]) * (3 (y)-1) =  21 < X <= 24

j-y =



j= 11では、

11*a[0] / a[1]  =  2
↑(33/13)
11*a[0]/ a[2] = 1

なので、ansには3(2+1)回a[0]以外の結果が入り込む。
 */

/*
4 6
20 25 30 100

let graph = vec![
    vec![0]
    0 vec![20,25,30]
    20 vec![25,30,40,45,50]
    25 vec![30,40,45,50,55]
];

let graph = vec![
    vec![]
    vec![20] 0
    vec![20,25] 0
    vec![20,25,30] 0
    vec![20,25,30,40] 1 + 1
    vec![20,25,30,40.45] 1 + 2
    vec![20,25,30,40,45,50] 1 + 3
    vec![20,25,30,40,45,50,55] 2 + 3 < 1 + 4
    vec![20,25,30,40,45,50,55, 60] 3 + 3
];
 */

/*
[999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999]
0[3, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999]
0+0[3, 6, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999]
0+1[3, 6, 9, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999]
1+1[3, 6, 9, 12, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999]
0[3, 6, 9, 12, 13, 999999999999999999, 999999999999999999, 999999999999999999, 999999999999999999]
1+2 or 4+0[3, 6, 9, 12, 13, 15, 999999999999999999, 999999999999999999, 999999999999999999]
2+2 or 4+0 [3, 6, 9, 12, 13, 15, 18, 999999999999999999, 999999999999999999]
[3, 6, 9, 12, 13, 15, 18, 21, 999999999999999999]
 */
