pub mod container;
pub mod insn;
pub mod materialize;

fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("usage: $0 <path/to/maidata.txt>");
    let content = read_file(&filename);
    let lexed = container::lex_maidata(&content);

    for kv in lexed {
        let k = kv.key.fragment();
        let v = kv.val.fragment();

        if k.starts_with("inote_") {
            // parse as insns
            println!("{}:", k);
            let insns = insn::parse_maidata_insns(kv.val);
            if let Ok((_, insns)) = insns {
                let mut mcx = materialize::context::MaterializationContext::with_offset(0.0);
                let notes = mcx.materialize_insns(insns.iter());
                println!("<{} notes>", notes.len());
            } else {
                panic!("insn parsing failed");
            }
        } else {
            println!("{} = \"{}\"", k, v);
        };
    }
}

fn read_file<P: AsRef<std::path::Path>>(path: P) -> String {
    let content = std::fs::read(path.as_ref()).expect("file reading failed");
    String::from_utf8(content).expect("decoding file content as utf-8 failed")
}

pub(crate) type Span<'a> = nom_locate::LocatedSpan<&'a str>;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Difficulty {
    /// The EASY difficulty.
    Easy = 1,
    /// The BASIC difficulty.
    Basic = 2,
    /// The ADVANCED difficulty.
    Advanced = 3,
    /// The EXPERT difficulty.
    Expert = 4,
    /// The MASTER difficulty.
    Master = 5,
    /// The Re:MASTER difficulty.
    ReMaster = 6,
    /// The ORIGINAL difficulty, previously called mai:EDIT in 2simai.
    Original = 7,
}
