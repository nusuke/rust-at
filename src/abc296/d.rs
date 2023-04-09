// https://atcoder.jp/contests/abc296/tasks/abc296_d
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
    // 一番大きい積でmに達するか調べる
    if n * n < m {
        return String::from("-1");
    }

    // * 1で到達できる場合
    if n >= m {
        return m.to_string();
    };

    let mut result = String::new();
    // 余り無しmを作れる場合
    for i in 2..=n {
        // println!("{}/{}={} あまりは{}", m, i, m / i, m % i);
        if m % i == 0 && m / i <= n {
            result = m.to_string();
            break;
        }
    }
    if !result.is_empty() {
        return result;
    }

    // 余りが有る場合
    let mut remainder = m;
    for i in (2..=n).rev() {
        if remainder == 1 {
            break;
        }
        if m / i == 1 {
            continue;
        };
        let target = m / i + 1;

        if target > n {
            break;
        }

        let sa = i * target - m;
        println!("{}*{}-{}={}", i, target, m, sa);
        if remainder > sa {
            remainder = sa;
        }
    }
    return (m + remainder).to_string();
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
}
