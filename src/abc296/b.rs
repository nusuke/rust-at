// https://atcoder.jp/contests/abc296/tasks/abc296_b
pub fn main() {
    let input = {
        let mut s1 = String::new();

        for _n in 0..8 {
            std::io::stdin().read_line(&mut s1).unwrap();
        }
        s1.trim_end().to_owned()
    };

    let arr = input.split("\n");
    for (i, row) in arr.enumerate() {
        let res = match row.find("*") {
            Some(index) => index + 1,
            None => 99,
        };

        if res < 99 {
            println!("{}", conver_row(res) + &(7 - i + 1).to_string());
        }
    }
}

fn conver_row(n: usize) -> String {
    if n == 1 {
        "a".to_string()
    } else if n == 2 {
        "b".to_string()
    } else if n == 3 {
        "c".to_string()
    } else if n == 4 {
        "d".to_string()
    } else if n == 5 {
        "e".to_string()
    } else if n == 6 {
        "f".to_string()
    } else if n == 7 {
        "g".to_string()
    } else {
        "h".to_string()
    }
}
