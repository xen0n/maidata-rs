use nom::IResult;

pub mod insn;
pub mod materialize;

fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("usage: $0 <path/to/maidata.txt>");
    let content = read_file(&filename);
    let lexed = lex_maidata(&content);

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

#[derive(Debug)]
struct KeyVal<'a> {
    pub key: Span<'a>,
    pub val: Span<'a>,
}

type Maidata<'a> = Vec<KeyVal<'a>>;

fn lex_maidata<'a>(x: &'a str) -> Maidata<'a> {
    let input = Span::new(x);
    let output = lex_maidata_inner(input);
    output.ok().expect("parse maidata failed").1
}

fn lex_maidata_inner(s: Span) -> IResult<Span, Maidata> {
    use nom::character::complete::char;
    use nom::combinator::opt;
    use nom::multi::many0;

    // Presumably most maidata.txt edited on Windows have BOM due to being edited by Notepad,
    // which is recommended by Japanese and Chinese simai tutorials.
    //
    // Eat it if present.
    let (s, _) = opt(char('\u{feff}'))(s)?;

    many0(lex_keyval)(s)
}

fn lex_keyval(s: Span) -> IResult<Span, KeyVal> {
    use nom::bytes::complete::take_till;
    use nom::character::complete::char;
    use nom::character::complete::multispace0;
    use nom::Slice;

    // we might have whitespaces before the first key-value pair, eat them
    // later pairs have the preceding whitespaces eaten during consumption of the value
    let (s, _) = multispace0(s)?;

    let (s, _) = char('&')(s)?;
    let (s, key) = take_till(|x| x == '=')(s)?;
    let (s, _) = char('=')(s)?;
    let (s, val) = take_till(|x| x == '&')(s)?;

    // strip off trailing newlines from value
    let num_bytes_to_remove = num_rightmost_whitespaces(val.fragment());
    let val = val.slice(0..val.fragment().len() - num_bytes_to_remove);

    Ok((s, KeyVal { key, val }))
}

fn num_rightmost_whitespaces<S: AsRef<str>>(x: S) -> usize {
    let mut result = 0;

    // only work with bytes for now, simplifies things quite a bit
    let x = x.as_ref().as_bytes();
    if x.len() == 0 {
        return 0;
    }

    for i in 0..x.len() {
        let i = x.len() - 1 - i;
        match x[i] {
            // '\t' | '\n' | '\r' | ' '
            0x09 | 0x0a | 0x0d | 0x20 => {
                result += 1;
                continue;
            }
            // first non-whitespace char backwards
            _ => break,
        }
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_num_rightmost_whitespaces() {
        use super::num_rightmost_whitespaces;

        assert_eq!(num_rightmost_whitespaces(""), 0);
        assert_eq!(num_rightmost_whitespaces("foo"), 0);
        assert_eq!(num_rightmost_whitespaces("\r\n\r\n"), 4);
        assert_eq!(num_rightmost_whitespaces("foo\r\n\r\n"), 4);
        assert_eq!(num_rightmost_whitespaces("foo\r\n\r\nbar"), 0);
        assert_eq!(num_rightmost_whitespaces("\n\n\nfoo\n\nbar\n"), 1);
    }
}
