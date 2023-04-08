// https://atcoder.jp/contests/abc296/tasks/abc296_a
pub fn main() {
    let input = {
        let mut _s = String::new();
        let mut s = String::new();
        std::io::stdin().read_line(&mut _s).unwrap();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };

    let f = input.contains("FF");
    let m = input.contains("MM");

    let result = if f || m { "No" } else { "Yes" };
    println!("{}", result);
}
