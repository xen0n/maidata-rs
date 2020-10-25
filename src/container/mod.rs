use nom::IResult;

use crate::{NomSpan, PResult};
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct KeyVal<'a> {
    pub key: NomSpan<'a>,
    pub val: NomSpan<'a>,
}

#[derive(Clone, Debug)]
pub struct Maidata {
    title: String,
    artist: String,

    fallback_designer: Option<String>,
    fallback_offset: Option<f32>,
    fallback_single_message: Option<String>,

    // XXX: is wholebpm mandatory?
    star_bpm: Option<f32>,

    difficulties: Vec<BeatmapData>,
}

impl Default for Maidata {
    fn default() -> Self {
        Self {
            title: String::default(),
            artist: String::default(),
            fallback_designer: None,
            fallback_offset: None,
            fallback_single_message: None,
            star_bpm: None,
            difficulties: vec![],
        }
    }
}

impl Maidata {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn artist(&self) -> &str {
        &self.artist
    }

    pub fn iter_difficulties(&self) -> impl Iterator<Item = AssociatedBeatmapData> {
        self.difficulties
            .iter()
            .map(move |diff| AssociatedBeatmapData {
                global: self,
                map: diff,
            })
    }
}

#[derive(Clone, Debug)]
pub struct BeatmapData {
    difficulty: crate::Difficulty,
    designer: Option<String>,
    offset: Option<f32>,
    level: Option<crate::Level>,
    insns: Vec<crate::Sp<crate::insn::RawInsn>>,
    single_message: Option<String>,
}

impl BeatmapData {
    pub(crate) fn default_with_difficulty(difficulty: crate::Difficulty) -> Self {
        Self {
            difficulty,
            designer: None,
            offset: None,
            level: None,
            insns: vec![],
            single_message: None,
        }
    }
}

pub struct AssociatedBeatmapData<'a> {
    global: &'a Maidata,
    map: &'a BeatmapData,
}

impl<'a> AssociatedBeatmapData<'a> {
    pub fn difficulty(&self) -> crate::Difficulty {
        self.map.difficulty
    }

    pub fn designer(&self) -> Option<&str> {
        self.map
            .designer
            .as_deref()
            .or(self.global.fallback_designer.as_deref())
    }

    pub fn offset(&self) -> Option<f32> {
        self.map.offset.or(self.global.fallback_offset)
    }

    pub fn level(&self) -> Option<crate::Level> {
        self.map.level.clone()
    }

    pub fn iter_insns(&self) -> impl Iterator<Item = &crate::Sp<crate::insn::RawInsn>> {
        self.map.insns.iter()
    }

    pub fn single_message(&self) -> Option<&str> {
        self.map
            .single_message
            .as_deref()
            .or(self.global.fallback_single_message.as_deref())
    }
}

pub fn lex_maidata<'a>(x: &'a str) -> Maidata {
    let input = NomSpan::new(x);
    let output = lex_maidata_inner(input);

    let kvs = output.ok().expect("parse maidata failed").1;

    let mut result = Maidata::default();
    let mut diff_map: HashMap<crate::Difficulty, BeatmapData> = HashMap::new();
    for kv in kvs {
        let k = *kv.key.fragment();
        let v = *kv.val.fragment();

        let mut handled = false;
        // difficulty-specific variables
        macro_rules! handle_one_diff {
            ( $num: literal => $diff: expr ) => {
                match k {
                    concat!("des_", stringify!($num)) => {
                        let mut data = diff_map
                            .entry($diff)
                            .or_insert(BeatmapData::default_with_difficulty($diff));
                        data.designer = Some(v.to_owned());
                        handled = true;
                    }
                    concat!("first_", stringify!($num)) => {
                        let mut data = diff_map
                            .entry($diff)
                            .or_insert(BeatmapData::default_with_difficulty($diff));
                        data.offset = Some(v.parse().expect("parse offset failed"));
                        handled = true;
                    }
                    concat!("inote_", stringify!($num)) => {
                        let mut data = diff_map
                            .entry($diff)
                            .or_insert(BeatmapData::default_with_difficulty($diff));
                        data.insns = crate::insn::parse_maidata_insns(kv.val)
                            .ok()
                            .expect("parse insns failed")
                            .1;
                        handled = true;
                    }
                    concat!("lv_", stringify!($num)) => {
                        use std::convert::TryInto;

                        let mut data = diff_map
                            .entry($diff)
                            .or_insert(BeatmapData::default_with_difficulty($diff));
                        match kv.val.try_into() {
                            Ok(lv) => {
                                data.level = Some(lv);
                            }
                            Err(_) => {
                                // TODO
                            }
                        }
                        handled = true;
                    }
                    concat!("smsg_", stringify!($num)) | concat!("freemsg_", stringify!($num)) => {
                        let mut data = diff_map
                            .entry($diff)
                            .or_insert(BeatmapData::default_with_difficulty($diff));
                        data.single_message = Some(v.to_owned());
                        handled = true;
                    }
                    _ => {}
                }
            };
        }

        handle_one_diff!(1 => crate::Difficulty::Easy);
        handle_one_diff!(2 => crate::Difficulty::Basic);
        handle_one_diff!(3 => crate::Difficulty::Advanced);
        handle_one_diff!(4 => crate::Difficulty::Expert);
        handle_one_diff!(5 => crate::Difficulty::Master);
        handle_one_diff!(6 => crate::Difficulty::ReMaster);
        handle_one_diff!(7 => crate::Difficulty::Original);
        if handled {
            continue;
        }

        // global variables
        match k {
            "title" => {
                result.title = v.to_owned();
            }
            "artist" => {
                result.artist = v.to_owned();
            }
            "first" => {
                match v.parse() {
                    Ok(offset) => {
                        result.fallback_offset = Some(offset);
                    }
                    Err(_) => {
                        // TODO
                    }
                }
            }
            "des" => {
                result.fallback_designer = Some(v.to_owned());
            }
            "smsg" | "freemsg" => {
                result.fallback_single_message = Some(v.to_owned());
            }
            _ => println!("unimplemented property: {} = {}", k, v),
        }
    }

    // put parsed difficulties into result
    result
        .difficulties
        .extend(diff_map.into_iter().map(|(_, data)| data));

    result
}

fn lex_maidata_inner(s: NomSpan) -> IResult<NomSpan, Vec<KeyVal>> {
    use nom::character::complete::char;
    use nom::combinator::opt;
    use nom::multi::many0;

    // Presumably most maidata.txt edited on Windows have BOM due to being edited by Notepad,
    // which is recommended by Japanese and Chinese simai tutorials.
    //
    // Eat it if present.
    let (s, _) = opt(char('\u{feff}'))(s)?;

    let (s, result) = many0(lex_keyval)(s)?;

    // require EOF
    let (s, _) = t_eof(s)?;

    Ok((s, result))
}

// TODO: dedup (with insn::parser::t_eof)
fn t_eof(input: NomSpan) -> PResult<NomSpan> {
    nom::eof!(input,)
}

fn lex_keyval(s: NomSpan) -> IResult<NomSpan, KeyVal> {
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

fn t_level(s: NomSpan) -> PResult<crate::Level> {
    use nom::branch::alt;

    alt((t_level_num, t_level_char))(s)
}

fn t_level_num(s: NomSpan) -> PResult<crate::Level> {
    use nom::character::complete::char;
    use nom::character::complete::digit1;
    use nom::character::complete::multispace0;
    use nom::combinator::opt;

    let (s, num) = digit1(s)?;
    let (s, _) = multispace0(s)?;
    let (s, plus) = opt(char('+'))(s)?;
    let (s, _) = multispace0(s)?;

    let lv = num.fragment().parse().unwrap();

    Ok((
        s,
        if plus.is_some() {
            crate::Level::Plus(lv)
        } else {
            crate::Level::Normal(lv)
        },
    ))
}
fn t_level_char(s: NomSpan) -> PResult<crate::Level> {
    use nom::character::complete::anychar;
    use nom::character::complete::char;
    use nom::character::complete::multispace0;

    let (s, _) = char('※')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, ch) = anychar(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, crate::Level::Char(ch)))
}

impl std::convert::TryFrom<NomSpan<'_>> for crate::Level {
    type Error = nom::Err<nom::error::ErrorKind>;

    fn try_from(value: NomSpan) -> Result<Self, Self::Error> {
        match t_level(value) {
            Ok((_, value)) => Ok(value),
            Err(e) => Err(e.map(|(_, x)| x)),
        }
    }
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
