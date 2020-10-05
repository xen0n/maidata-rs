use nom::IResult;
use nom_locate::position;

fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("usage: $0 <path/to/maidata.txt>");
    let content = read_file(&filename);
    let lexed = lex_maidata(&content);
    println!("{:?}", lexed);
}

fn read_file<P: AsRef<std::path::Path>>(path: P) -> String {
    let content = std::fs::read(path.as_ref()).expect("file reading failed");
    String::from_utf8(content).expect("decoding file content as utf-8 failed")
}

type Span<'a> = nom_locate::LocatedSpan<&'a str>;

#[derive(Debug)]
struct KeyVal<'a> {
    pub key_pos: Span<'a>,
    pub val_pos: Span<'a>,
    pub key: Span<'a>,
    pub val: Span<'a>,
}

type Maidata<'a> = Vec<KeyVal<'a>>;

fn lex_maidata<'a>(x: &'a str) -> Maidata<'a> {
    // Presumably most maidata.txt edited on Windows have BOM due to being edited by Notepad,
    // which is recommended by Japanese and Chinese simai tutorials.
    //
    // Eat it if present.
    let x = x.strip_prefix("\u{feff}").unwrap_or(x);

    let input = Span::new(x);
    let output = lex_maidata_inner(input);
    output.ok().expect("parse maidata failed").1
}

fn lex_maidata_inner(s: Span) -> IResult<Span, Maidata> {
    use nom::multi::many0;

    many0(lex_keyval)(s)
}

fn lex_keyval(s: Span) -> IResult<Span, KeyVal> {
    use nom::bytes::complete::take_till;
    use nom::character::complete::char;

    let (s, _) = char('&')(s)?;
    let (s, key_pos) = position(s)?;
    let (s, key) = take_till(|x| x == '=')(s)?;
    let (s, _) = char('=')(s)?;
    let (s, val_pos) = position(s)?;
    let (s, val) = take_till(|x| x == '&')(s)?;

    Ok((
        s,
        KeyVal {
            key_pos,
            val_pos,
            key,
            val,
        },
    ))
}
