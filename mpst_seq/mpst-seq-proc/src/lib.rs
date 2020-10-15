#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Result, Token};

#[derive(Debug)]
struct SeqMacroInput {
    from: syn::LitInt,
    to: syn::LitInt,
    inclusive: bool,
    objection: bool,
    exclusion: syn::LitInt,
    ident: syn::Ident,
    tt: proc_macro2::TokenStream,
}

impl Parse for SeqMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = syn::Ident::parse(input)?;
        let _in = <Token![in]>::parse(input)?;
        let from = syn::LitInt::parse(input)?;
        let inclusive = input.peek(Token![..=]);
        if inclusive {
            <Token![..=]>::parse(input)?;
        } else {
            <Token![..]>::parse(input)?;
        }
        let to = syn::LitInt::parse(input)?;
        let objection = input.peek(Token![!]);
        let exclusion = if objection {
            <Token![!]>::parse(input)?;
            syn::LitInt::parse(input)?
        } else {
            syn::LitInt::new("0", proc_macro2::Span::call_site())
        };
        let content;
        let _braces = syn::braced!(content in input);
        let tt = proc_macro2::TokenStream::parse(&content)?;

        Ok(SeqMacroInput {
            from,
            to,
            inclusive,
            objection,
            exclusion,
            tt,
            ident,
        })
    }
}

impl Into<proc_macro2::TokenStream> for SeqMacroInput {
    fn into(self) -> proc_macro2::TokenStream {
        self.expand(self.tt.clone())
    }
}

#[derive(Copy, Clone, Debug)]
enum Mode {
    ReplaceIdent(u64),
    ReplaceSequence,
}

impl SeqMacroInput {
    /// from → to + offset
    fn from_to_offset_exclusion(&self) -> (u64, u64, u64, u64) {
        let from = (self.from).base10_parse::<u64>().unwrap();
        let to = (self.to).base10_parse::<u64>().unwrap();
        let exclusion = (self.exclusion).base10_parse::<u64>().unwrap();
        let offset = if self.inclusive { 1 } else { 0 };
        (from, to, offset, exclusion)
    }

    /// from → to + offset
    fn range_0(&self, extra: u64) -> impl Iterator<Item = u64> {
        let (from, to, offset, _) = self.from_to_offset_exclusion();
        (from + extra)..(to + offset + extra)
    }

    /// from → (3 * (to + offset - 1))
    fn range_1(&self, extra: u64) -> impl Iterator<Item = u64> {
        let (from, to, offset, _) = self.from_to_offset_exclusion();
        (from + extra)..(3 * (to + offset - 1) + extra)
    }

    /// from → (2 * (to + offset) - 1)
    fn range_2(&self, extra: u64) -> impl Iterator<Item = u64> {
        let (from, to, offset, _) = self.from_to_offset_exclusion();
        (from + extra)..(2 * (to + offset) - 1 + extra)
    }

    /// to → to + (to - from)
    fn range_5(&self, extra: u64) -> impl Iterator<Item = u64> {
        let (from, to, offset, _) = self.from_to_offset_exclusion();
        (to + extra)..((to - from) + to + offset + extra)
    }

    fn expand2(
        &self,
        tt: proc_macro2::TokenTree,
        rest: &mut proc_macro2::token_stream::IntoIter,
        mutated: &mut bool,
        mode: Mode,
        mode_inclusive: Mode,
    ) -> proc_macro2::TokenStream {
        let tt = match tt {
            proc_macro2::TokenTree::Group(g) => {
                let (expanded, g_mutated) = self.expand_pass(g.stream(), mode, mode_inclusive);
                let mut expanded = proc_macro2::Group::new(g.delimiter(), expanded);
                *mutated |= g_mutated;
                expanded.set_span(g.span());
                proc_macro2::TokenTree::Group(expanded)
            }
            proc_macro2::TokenTree::Ident(ref ident) if ident == &self.ident => {
                if let Mode::ReplaceIdent(i) = mode {
                    let mut lit = proc_macro2::Literal::u64_unsuffixed(i);
                    lit.set_span(ident.span());
                    *mutated = true;
                    proc_macro2::TokenTree::Literal(lit)
                } else {
                    // not allowed to replace idents in first pass
                    proc_macro2::TokenTree::Ident(ident.clone())
                }
            }
            proc_macro2::TokenTree::Ident(mut ident) => {
                // search for # followed by self.ident at the end of an identifier
                // OR # self.ident #
                let mut peek = rest.clone();
                match (mode, mode_inclusive, peek.next(), peek.next()) {
                    (
                        Mode::ReplaceIdent(i),
                        _,
                        Some(proc_macro2::TokenTree::Punct(ref punct)),
                        Some(proc_macro2::TokenTree::Ident(ref ident2)),
                    ) if punct.as_char() == '#' && ident2 == &self.ident => {
                        // have seen ident # N
                        ident = proc_macro2::Ident::new(&format!("{}{}", ident, i), ident.span());
                        *rest = peek.clone();
                        *mutated = true;

                        // we may need to also consume another #
                        match peek.next() {
                            Some(proc_macro2::TokenTree::Punct(ref punct))
                                if punct.as_char() == '#' =>
                            {
                                *rest = peek;
                            }
                            _ => {}
                        }
                    }
                    (
                        _,
                        Mode::ReplaceIdent(j),
                        Some(proc_macro2::TokenTree::Punct(ref punct)),
                        Some(proc_macro2::TokenTree::Ident(ref ident2)),
                    ) if punct.as_char() == '^' && ident2 == &self.ident => {
                        // have seen ident # N
                        ident = proc_macro2::Ident::new(&format!("{}{}", ident, j), ident.span());
                        *rest = peek.clone();
                        *mutated = true;

                        // we may need to also consume another #
                        match peek.next() {
                            Some(proc_macro2::TokenTree::Punct(ref punct))
                                if punct.as_char() == '^' =>
                            {
                                *rest = peek;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }

                proc_macro2::TokenTree::Ident(ident)
            }
            proc_macro2::TokenTree::Punct(ref p) if p.as_char() == '#' => {
                let mut peek = rest.clone();
                match (peek.next(), peek.next(), peek.next()) {
                    // is this #(...)n* ?
                    (
                        Some(proc_macro2::TokenTree::Group(ref rep)),
                        Some(proc_macro2::TokenTree::Literal(ref extra)),
                        Some(proc_macro2::TokenTree::Punct(ref star)),
                    ) if rep.delimiter() == proc_macro2::Delimiter::Parenthesis
                        && star.as_char() == '*' =>
                    {
                        // yes! expand ... for each sequence in the range
                        *mutated = true;
                        *rest = peek;

                        // TODO
                        // extra.to_string().parse::<u64>().unwrap() → something better

                        return self
                            .range_0(extra.to_string().parse::<u64>().unwrap())
                            .map(|i| {
                                self.expand_pass(
                                    rep.stream(),
                                    Mode::ReplaceIdent(i),
                                    mode_inclusive,
                                )
                            })
                            .map(|(ts, _)| ts)
                            .collect();
                    }
                    // is this #(...)n~ ?
                    (
                        Some(proc_macro2::TokenTree::Group(ref rep)),
                        Some(proc_macro2::TokenTree::Literal(ref extra)),
                        Some(proc_macro2::TokenTree::Punct(ref tilde)),
                    ) if rep.delimiter() == proc_macro2::Delimiter::Parenthesis
                        && tilde.as_char() == '~' =>
                    {
                        // yes! expand ... for each sequence in the range
                        *mutated = true;
                        *rest = peek;

                        // TODO
                        // extra.to_string().parse::<u64>().unwrap() → something better

                        return self
                            .range_1(extra.to_string().parse::<u64>().unwrap())
                            .map(|i| {
                                self.expand_pass(
                                    rep.stream(),
                                    Mode::ReplaceIdent(i),
                                    mode_inclusive,
                                )
                            })
                            .map(|(ts, _)| ts)
                            .collect();
                    }
                    // is this #(...)n^ ?
                    (
                        Some(proc_macro2::TokenTree::Group(ref rep)),
                        Some(proc_macro2::TokenTree::Literal(ref extra)),
                        Some(proc_macro2::TokenTree::Punct(ref hat)),
                    ) if rep.delimiter() == proc_macro2::Delimiter::Parenthesis
                        && hat.as_char() == '^' =>
                    {
                        // yes! expand ... for each sequence in the range
                        *mutated = true;
                        *rest = peek;

                        // TODO
                        // extra.to_string().parse::<u64>().unwrap() → something better

                        return self
                            .range_2(extra.to_string().parse::<u64>().unwrap())
                            .map(|i| {
                                self.expand_pass(
                                    rep.stream(),
                                    Mode::ReplaceIdent(i),
                                    mode_inclusive,
                                )
                            })
                            .map(|(ts, _)| ts)
                            .collect();
                    }
                    _ => {}
                }
                proc_macro2::TokenTree::Punct(p.clone())
            }
            proc_macro2::TokenTree::Punct(ref p) if p.as_char() == '~' => {
                // is this ~(...)* ?
                let mut peek = rest.clone();
                match (peek.next(), peek.next(), peek.next()) {
                    (
                        Some(proc_macro2::TokenTree::Group(ref rep_if)),
                        Some(proc_macro2::TokenTree::Group(ref rep_else)),
                        Some(proc_macro2::TokenTree::Punct(ref star)),
                    ) if rep_if.delimiter() == proc_macro2::Delimiter::Parenthesis
                        && rep_else.delimiter() == proc_macro2::Delimiter::Parenthesis
                        && star.as_char() == '*' =>
                    {
                        // yes! expand ... for each sequence in the range
                        *mutated = true;
                        *rest = peek;

                        return self
                            .range_0(0)
                            .filter_map(|i| {
                                if i == self.exclusion.base10_parse::<u64>().unwrap()
                                    && self.objection
                                {
                                    std::option::Option::Some(self.expand_pass(
                                        rep_else.stream(),
                                        Mode::ReplaceIdent(i),
                                        mode_inclusive,
                                    ))
                                } else {
                                    std::option::Option::Some(self.expand_pass(
                                        rep_if.stream(),
                                        Mode::ReplaceIdent(i),
                                        mode_inclusive,
                                    ))
                                }
                            })
                            .map(|(ts, _)| ts)
                            .collect();
                    }
                    _ => {}
                }
                proc_macro2::TokenTree::Punct(p.clone())
            }
            proc_macro2::TokenTree::Punct(ref p) if p.as_char() == '^' => {
                let mut peek = rest.clone();
                match (peek.next(), peek.next()) {
                    // is this ^(...)* ?
                    (
                        Some(proc_macro2::TokenTree::Group(ref rep)),
                        Some(proc_macro2::TokenTree::Punct(ref star)),
                    ) if rep.delimiter() == proc_macro2::Delimiter::Parenthesis
                        && star.as_char() == '*' =>
                    {
                        // yes! expand ... for each sequence in the range
                        *mutated = true;
                        *rest = peek;

                        return self
                            .range_5(0)
                            .map(|i| {
                                self.expand_pass(
                                    rep.stream(),
                                    Mode::ReplaceIdent(i),
                                    mode_inclusive,
                                )
                            })
                            .map(|(ts, _)| ts)
                            .collect();
                    }
                    _ => {}
                }
                proc_macro2::TokenTree::Punct(p.clone())
            }
            tt => tt,
        };
        std::iter::once(tt).collect()
    }

    fn expand_pass(
        &self,
        stream: proc_macro2::TokenStream,
        mode: Mode,
        mode_inclusive: Mode,
    ) -> (proc_macro2::TokenStream, bool) {
        let mut out = proc_macro2::TokenStream::new();
        let mut mutated = false;
        let mut tts = stream.into_iter();
        while let Some(tt) = tts.next() {
            out.extend(self.expand2(tt, &mut tts, &mut mutated, mode, mode_inclusive));
        }
        (out, mutated)
    }

    fn expand(&self, stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let (out, mutated) =
            self.expand_pass(stream.clone(), Mode::ReplaceSequence, Mode::ReplaceSequence);
        if mutated {
            return out;
        }

        self.range_0(0)
            .map(|i| self.expand_pass(stream.clone(), Mode::ReplaceIdent(i), Mode::ReplaceSequence))
            .map(|(ts, _)| ts)
            .collect()
    }
}

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SeqMacroInput);
    let output: proc_macro2::TokenStream = input.into();
    output.into()
}

#[proc_macro_hack]
pub fn eseq(input: TokenStream) -> TokenStream {
    seq(input)
}
