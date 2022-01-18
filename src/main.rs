fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    let contents = format!("{}/t{}/n", key, value);

    std::fs::write("kv.db", contents);
}
