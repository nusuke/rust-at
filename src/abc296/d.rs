// https://atcoder.jp/contests/abc296/tasks/abc296_d
#[allow(dead_code)]
pub fn main() {
    let input = {
        let mut s1 = String::new();
        std::io::stdin().read_line(&mut s1).unwrap();

        s1.trim_end().to_owned()
    };
    let res = answer(input);
    println!("{}", res);
}

fn answer(input: String) -> String {
    let (n, m) = {
        let mut ws = input.split_whitespace();
        let n: i64 = ws.next().unwrap().parse().unwrap();
        let m: i64 = ws.next().unwrap().parse().unwrap();
        (n, m)
    };

    let inf = 999999999999999999;
    let mut ans = inf;
    for i in 1..=n {
        println!("x={},i={}", (m + i - 1) / i, i);
        let x = (m + i - 1) / i;
        if x <= n {
            ans = if ans < i * x { ans } else { i * x }
        }
        if i > x {
            break;
        }
    }

    if ans == inf {
        return "-1".to_string();
    } else {
        return ans.to_string();
    }

    // if n * n < m {
    //     return String::from("-1");
    // }

    // if n >= m {
    //     return m.to_string();
    // };

    // let mut result = 0;
    // for i in 2..=n {
    //     if m % i == 0 && m / i <= n {
    //         result = m;
    //         break;
    //     }
    // }
    // if result != 0 {
    //     return result.to_string();
    // }

    // // あまり
    // let mut remainder = m;

    // for i in (1..=n).rev() {
    //     println!(
    //         "{}*{}={}。目指す数値は{}。あまりは{}",
    //         m / i + 1,
    //         i,
    //         (m / i + 1) * i,
    //         m,
    //         ((m / i + 1) * i) - m
    //     );
    //     if remainder == 1 {
    //         break;
    //     }

    //     let target = m / i + 1;
    //     if target > i {
    //         break;
    //     }

    //     let sa = i * target - m;
    //     if remainder > sa {
    //         remainder = sa;
    //     }
    // }
    // return (m + remainder).to_string();
}
#[cfg(test)]
mod tests {
    use crate::abc296::d::answer;

    #[test]
    fn it_no_solutioin_case_1() {
        let input = "2 5".to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "-1");
    }

    #[test]
    fn it_works_2() {
        let input = "5 7
            "
        .to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "8");
    }

    #[test]
    fn it_works_3() {
        let input = "100000 10000000000".to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "10000000000");
    }

    #[test]
    fn it_multiplication_1() {
        let input = "8 6".to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "6");
    }

    #[test]
    fn it_multiplication_2() {
        let input = "4 8".to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "8");
    }

    #[test]
    fn it_multiplication_3() {
        let input = "10 98".to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "100");
    }

    #[test]
    fn it_multiplication_4() {
        let input = "888 2302".to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "2303");
    }

    #[test]
    fn it_multiplication_5() {
        let input = "200 2003".to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "2004");
    }
}

// 99999999 100000000007
