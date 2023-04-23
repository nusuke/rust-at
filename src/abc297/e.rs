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
    let (_n, k, mut a) = {
        let mut ws = input.split_whitespace();
        let n: i64 = ws.next().unwrap().parse().unwrap();
        let m: usize = ws.next().unwrap().parse().unwrap();
        let mut a = ws.map(|s| s.parse().unwrap()).collect::<Vec<i64>>();
        a.sort_by(|a, b| b.cmp(a));
        (n, m, a)
    };

    if a.last().unwrap() == &1 {
        return k.to_string();
    };

    static INF: i64 = 999999999999999999;
    let mut ans = vec![INF; k];
    let mut left: usize = 0;
    let mut right: usize = 0;
    for i in 0..k {
        println!(
            "i={} left={} right={} {}+{}={}",
            i,
            left,
            right,
            ans[left],
            ans[right],
            ans[left] + ans[right]
        );

        let last = match a.last() {
            Some(n) => *n,
            None => INF,
        };
        if ans[left] + ans[right] < last {
            ans[i] = ans[left] + ans[right];
            if right < k - 1 {
                right = right + 1;
            } else {
                left += 1;
                // right = 0;
            }
        } else {
            let poped = match a.pop() {
                Some(n) => n,
                None => INF,
            };
            ans[i] = poped;
        }
    }
    println!("{:?}", ans);

    return ans[k - 1].to_string();

    // let a_eles = a.clone().split_off(1);
    // let c = get_amari(a_eles.clone(), a[0] * k);

    // let min = (a[0] * (k - c)).abs();
    // let plus_index = get_amari(a_eles.clone(), min);
    // let min_index = if min / a[0] == 1 {
    //     0
    // } else {
    //     min / a[0] + plus_index
    // };

    // let key = if k - min_index - 1 >= 0 {
    //     k - min_index - 1
    // } else {
    //     0
    // } as usize;

    // println!(
    //     "c={} 最小値{} ←index{}  求める値のindexは{}",
    //     c, min, min_index, key
    // );

    // let r = (min..=a[0] * k)
    //     .into_iter()
    //     .filter(|&i| {
    //         let r: Vec<&i64> = a
    //             .iter()
    //             .filter(|&ele| i % ele == 0 || (i - ele) % a[0] == 0)
    //             .collect::<Vec<&i64>>();
    //         r.len() > 0
    //     })
    //     .take(key + 1)
    //     .collect::<Vec<i64>>();

    // println!("{:?}", r);
    // return r[key].to_string();
}

// fn get_amari(array: Vec<i64>, n: i64) -> i64 {
//     return array
//         .iter()
//         .map(|ele| n / ele)
//         .fold(0, |left, right| left + right);
// }
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
    fn e_2() {
        let input = "2 10\n2 1".to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "10");
    }

    #[test]
    fn e_3() {
        let input = "10 200000\n955277671 764071525 871653439 819642859 703677532 515827892 127889502 881462887 330802980 503797872
        ".to_string();
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
    vec![]
    vec![20] 0
    vec![20,25] 0
    vec![20,25,30] 0
    vec![20,25,30,40] 1 + 1
    vec![20,25,30,40.45] 1 + 2
    vec![20,25,30,40,45,50] 1 + 3
    vec![20,25,30,40,45,50,55] 2 + 3
];
 */
