// https://atcoder.jp/contests/abc296/tasks/abc296_d
#[allow(dead_code)]
pub fn main() {
    let (w, s) = {
        let mut s1 = String::new();
        std::io::stdin().read_line(&mut s1).unwrap();
        let (n, w) = {
            let mut ws = s1.split_whitespace();
            let n: i32 = ws.next().unwrap().parse().unwrap();
            let w: usize = ws.next().unwrap().parse().unwrap();
            (n, w)
        };
        let mut s2 = String::new();
        for _i in 0..n {
            std::io::stdin().read_line(&mut s2).unwrap();
        }

        (w, s2.trim_end().to_owned())
    };
    let res = answer(w, s);
    println!("{}", res);
}

fn answer(_w: usize, s: String) -> String {
    let s_vec: Vec<&str> = s.split_whitespace().collect();

    let res: Vec<String> = s_vec.iter().map(|s| s.replace("TT", "PC")).collect();

    return res.join("\n");
}
// #[cfg(test)]
// mod tests {
//     use crate::abc297::c::answer;

//     #[test]
//     fn it_no_solutioin_case_1() {
//         let w = 3;
//         let s = "TTT T.T".to_string();
//         let res = answer(w, s);
//         //  assert
//         assert_eq!(res, "-PCT\nT.T");
//     }
// }
