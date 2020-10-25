pub(crate) type NomSpan<'a> = nom_locate::LocatedSpan<&'a str>;

/// Convenient alias for parsing result with spans.
pub(crate) type PResult<'a, T> = nom::IResult<NomSpan<'a>, T>;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Span {
    pub byte_offset: usize,
    pub line: usize,
    pub col: usize,
    pub end_line: usize,
    pub end_col: usize,
    pub len: usize,
}

impl Span {
    pub fn from_start_end(start: NomSpan<'_>, end: NomSpan<'_>) -> Self {
        use nom::Offset;

        let byte_offset = start.location_offset();
        let line = start.location_line() as usize;
        let col = start.get_utf8_column();
        let end_line = end.location_line() as usize;
        let end_col = end.get_utf8_column();
        let len = start.offset(&end);

        Self {
            byte_offset,
            line,
            col,
            end_line,
            end_col,
            len,
        }
    }
}

impl From<(NomSpan<'_>, NomSpan<'_>)> for Span {
    fn from(x: (NomSpan<'_>, NomSpan<'_>)) -> Self {
        Span::from_start_end(x.0, x.1)
    }
}

/// Thing with span information attached.
pub struct Sp<T>(T, crate::Span);

/// Convenient alias for working with lists of Sp-ed things.
pub type VecSp<T> = Vec<Sp<T>>;

impl<T> std::ops::Deref for Sp<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Sp<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Copy for Sp<T> where T: Copy {}

impl<T> Clone for Sp<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1)
    }
}

impl<T> PartialEq for Sp<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T> Eq for Sp<T> where T: Eq + PartialEq {}

impl<T> std::fmt::Display for Sp<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let span = self.1;
        write!(
            f,
            "[{}:{}-{}:{}]{}",
            span.line, span.col, span.end_line, span.end_col, self.0
        )
    }
}

impl<T> std::fmt::Debug for Sp<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let span = self.1;
        write!(
            f,
            "[{}:{}-{}:{}]{:?}",
            span.line, span.col, span.end_line, span.end_col, self.0
        )
    }
}

impl<T> Sp<T> {
    pub fn new(obj: T, span: crate::Span) -> Self {
        Self(obj, span)
    }

    pub fn span(&self) -> crate::Span {
        self.1
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}
