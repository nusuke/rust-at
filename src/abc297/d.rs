// https://atcoder.jp/contests/abc297/tasks/abc297_d
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
    let (a, b) = {
        let mut ws = input.split_whitespace();
        let n: i64 = ws.next().unwrap().parse().unwrap();
        let m: i64 = ws.next().unwrap().parse().unwrap();
        (n, m)
    };
    fn cnt_fn(cnt: i64, a: i64, b: i64) -> i64 {
        // print!("cnt={}, a={} b={}", cnt, a, b);
        if a == b {
            return cnt;
        } else if a > b {
            // println!(" a/b={}...{}", a / b, a % b);
            if a % b == 0 {
                return cnt + a / b - 1;
            } else if a / b != 1 {
                return cnt + cnt_fn(a / b, a % b, b);
            } else {
                return cnt_fn(cnt + 1, a - b, b);
            }
        } else if a < b {
            // println!(" b/a={}...{}", b / a, b % a);
            if b % a == 0 {
                return cnt + b / a - 1;
            } else if b / a != 1 {
                return cnt + cnt_fn(b / a, a, b % a);
            } else {
                return cnt_fn(cnt + 1, a, b - a);
            }
        } else {
            return 0;
        }
    }
    let cnt = cnt_fn(0, a, b);
    return cnt.to_string();
}

#[cfg(test)]
mod tests {
    use crate::abc297::d::{answer, d_answer};

    #[test]
    fn it_no_solutioin_case_1() {
        let input = "3 8".to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "4");
    }

    #[test]
    fn it_no_solutioin_case_2() {
        let input = "1234567890 1234567890".to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "0");
    }

    #[test]
    fn it_no_solutioin_case_3() {
        let input = "1597 987".to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "15");
    }

    #[test]
    fn it_no_solutioin_case_4() {
        let input = "30 300".to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "9");
    }

    #[test]
    fn d1() {
        let input = "230 999".to_string();
        let res = answer(input.clone());
        // assert
        assert_eq!(res, d_answer(input));
    }
}

// 30 300
//1 30 270
//2 30 240
//3 30 210
//4 30 180
//5 30 150
//6 30 120
//7 30 90
//8 30 60
//9 30 30

// 愚直解
#[allow(dead_code)]
fn d_answer(input: String) -> String {
    let (a, b) = {
        let mut ws = input.split_whitespace();
        let n: i64 = ws.next().unwrap().parse().unwrap();
        let m: i64 = ws.next().unwrap().parse().unwrap();
        (n, m)
    };
    fn cnt_fn(cnt: i64, a: i64, b: i64) -> i64 {
        println!("cnt={}, a={} b={}", cnt, a, b);
        if a == b {
            return cnt;
        } else if a > b {
            if a % b == 0 {
                return cnt + a / b - 1;
            } else {
                return cnt_fn(cnt + 1, a - b, b);
            }
        } else if a < b {
            if b % a == 0 {
                return cnt + b / a - 1;
            } else {
                return cnt_fn(cnt + 1, a, b - a);
            }
        } else {
            return 0;
        }
    }
    let cnt = cnt_fn(0, a, b);
    return cnt.to_string();
}

/*
愚直会との比較
cnt=0, a=230 b=999
cnt=1, a=230 b=769
cnt=2, a=230 b=539
cnt=3, a=230 b=309
cnt=4, a=230 b=79
cnt=5, a=151 b=79
cnt=6, a=72 b=79
cnt=7, a=72 b=7
cnt=8, a=65 b=7
cnt=9, a=58 b=7
cnt=10, a=51 b=7
cnt=11, a=44 b=7
cnt=12, a=37 b=7
cnt=13, a=30 b=7
cnt=14, a=23 b=7
cnt=15, a=16 b=7
cnt=16, a=9 b=7
cnt=17, a=2 b=7
cnt=18, a=2 b=5
cnt=19, a=2 b=3
cnt=20, a=2 b=1


cnt=0, a=230 b=999 b/a=4...79
cnt=4, a=230 b=4 a/b=57...2
cnt=57, a=57 b=4 a/b=14...1
cnt=14, a=14 b=4 a/b=3...2
cnt=3, a=3 b=4 b/a=1...1
cnt=4, a=3 b=1 a/b=3...0
81
 */
