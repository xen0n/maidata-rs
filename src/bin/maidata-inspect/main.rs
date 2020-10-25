fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("usage: $0 <path/to/maidata.txt>");
    let content = read_file(&filename);
    let maidata = maidata::container::lex_maidata(&content);

    println!("title = {}", maidata.title());
    println!("artist = {}", maidata.artist());

    for diff in maidata.iter_difficulties() {
        use std::borrow::Cow;

        println!();
        println!("difficulty {:?}", diff.difficulty());
        println!(
            "  level {}",
            diff.level()
                .map_or(Cow::Borrowed("<not set>"), |x| Cow::Owned(format!("{}", x)))
        );
        println!(
            "  offset {}",
            diff.offset()
                .map_or(Cow::Borrowed("<not set>"), |x| Cow::Owned(format!("{}", x)))
        );
        println!("  designer {}", diff.designer().unwrap_or("<not set>"));
        println!(
            "  static message {}",
            diff.single_message().unwrap_or("<not set>")
        );

        let mut mcx = maidata::materialize::MaterializationContext::with_offset(0.0);
        let notes = mcx.materialize_insns(diff.iter_insns());
        println!("  <{} notes materialized>", notes.len());

        for insn in diff.iter_insns() {
            println!("{:?}", insn);
        }
    }
}

fn read_file<P: AsRef<std::path::Path>>(path: P) -> String {
    let content = std::fs::read(path.as_ref()).expect("file reading failed");
    String::from_utf8(content).expect("decoding file content as utf-8 failed")
}

