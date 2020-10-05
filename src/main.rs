fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("usage: $0 <path/to/maidata.txt>");
    let content = read_file(&filename);

    println!("{}", content);
}

fn read_file<P: AsRef<std::path::Path>>(path: P) -> String {
    let content = std::fs::read(path.as_ref()).expect("file reading failed");
    String::from_utf8(content).expect("decoding file content as utf-8 failed")
}
