// https://atcoder.jp/contests/abc297/tasks/abc297_a
#[allow(dead_code)]
pub fn main() {
    let input = {
        let mut s1 = String::new();
        std::io::stdin().read_line(&mut s1).unwrap();
        std::io::stdin().read_line(&mut s1).unwrap();
        s1.trim_end().to_owned()
    };
    println!("{}", input);
    let res = answer(input);
    println!("{}", res);
}

fn answer(input: String) -> String {
    let (_n, d, t) = {
        let ws = input.lines().collect::<Vec<&str>>();
        let mut line1 = ws[0].split_whitespace();
        let n: i32 = line1.next().unwrap().parse().unwrap();
        let d: i32 = line1.next().unwrap().parse().unwrap();
        let t: Vec<i32> = ws[1]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        (n, d, t)
    };

    let mut res_time = -1;
    t.iter().reduce(|l, r| {
        if r - l <= d && res_time == -1 {
            res_time = r + 0;
        };
        r
    });

    return res_time.to_string();
}
#[cfg(test)]
mod tests {
    use crate::abc297::a::answer;

    #[test]
    fn it_no_solutioin_case_1() {
        let input = "4 500\n300 900 1300 1700".to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "1300");
    }

    #[test]
    fn it_no_solutioin_case_2() {
        let input = "5 99\n100 200 300 400 500".to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "-1");
    }

    #[test]
    fn it_no_solutioin_case_3() {
        let input = "4 500\n100 600 1100 1600
        "
        .to_string();
        let res = answer(input);
        // assert
        assert_eq!(res, "600");
    }
}
