use crate::core::{LineColumn, Span};
use crate::parse::{ParseResult, ParseResultNaked, ParserSpan};
use std::fmt::{self, Debug};

pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

impl<T> Debug for Spanned<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            self.value.fmt(f)
        } else {
            f.debug_struct("Spanned")
                .field("value", &self.value)
                .field("span", &self.span)
                .finish()
        }
    }
}

impl<T> Spanned<T> {
    pub fn wrap<'a, F>(f: F) -> impl for<'b> Fn(ParserSpan<'a>) -> ParseResult<'a, T>
    where
        F: Fn(ParserSpan<'a>) -> ParseResultNaked<'a, T>,
    {
        move |s| {
            let start = LineColumn::create_from(&s);
            let (s, value) = f(s)?;
            let end = LineColumn::create_from(&s);
            Ok((
                s,
                Spanned {
                    value,
                    span: Span { start, end },
                },
            ))
        }
    }

    pub fn wrap_mut<'a, F>(mut f: F) -> impl for<'b> FnMut(ParserSpan<'a>) -> ParseResult<'a, T>
    where
        F: FnMut(ParserSpan<'a>) -> ParseResultNaked<'a, T>,
    {
        move |s| {
            let start = LineColumn::create_from(&s);
            let (s, value) = f(s)?;
            let end = LineColumn::create_from(&s);
            Ok((
                s,
                Spanned {
                    value,
                    span: Span { start, end },
                },
            ))
        }
    }
}

impl<T> Spanned<T> {
    /// Maps an Spanned<T> to Spanned<U> by applying a function to a contained value.
    pub fn map<F, U: Debug>(self, f: F) -> Spanned<U>
    where
        F: FnOnce(T) -> U,
    {
        Spanned {
            value: f(self.value),
            span: self.span,
        }
    }
    /// Maps an Spanned<T> to Spanned<U> by applying a function to a self.
    pub fn map_wrapped<F, U: Debug>(self, f: F) -> Spanned<U>
    where
        F: FnOnce(Spanned<T>) -> U,
    {
        Spanned {
            value: f(Spanned {
                value: self.value,
                span: self.span.clone(),
            }),
            span: self.span,
        }
    }
}

impl<T> Spanned<Box<T>> {
    /// Tranpose a Spanned of Box into a Box of Spanned
    pub fn transpose(self) -> Box<Spanned<T>> {
        Box::new(Spanned {
            value: *self.value,
            span: self.span,
        })
    }
}
