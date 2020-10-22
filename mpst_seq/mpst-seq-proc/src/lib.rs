#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use std::convert::TryInto;
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
        self.expand(self.tt.clone(), &self.diag(), &self.matrix())
    }
}

#[derive(Copy, Clone, Debug)]
enum Mode {
    ReplaceIdent(u64),
    ReplaceSequence,
}

impl SeqMacroInput {
    /// from → to + offset
    fn from_to_offset_exclusion(&self) -> (u64, u64, u64) {
        let from = (self.from).base10_parse::<u64>().unwrap();
        let to = (self.to).base10_parse::<u64>().unwrap();
        let exclusion = (self.exclusion).base10_parse::<u64>().unwrap();
        let offset = if self.inclusive { 1 } else { 0 };
        (from, to + offset, exclusion)
    }

    /// from → to + offset + limit + extra
    fn range_0(&self, limit: u64, extra: u64, include: u64) -> impl Iterator<Item = u64> {
        let (from, to, _) = self.from_to_offset_exclusion();
        (from + extra)..(to + include + extra + limit)
    }

    /// from → 3 * (to + offset - 1) + limit + extra
    fn range_1(&self, limit: u64, extra: u64) -> impl Iterator<Item = u64> {
        let (from, to, _) = self.from_to_offset_exclusion();
        (from + extra)..(3 * (to - 1) + extra + limit)
    }

    /// from → 2 * (to + offset) - 1 + limit + extra
    fn range_2(&self, limit: u64, extra: u64) -> impl Iterator<Item = u64> {
        let (from, to, _) = self.from_to_offset_exclusion();
        (from + extra)..(2 * to - 1 + extra + limit)
    }

    /// to → 2 * (to + offset) - from + extra + limit + extra
    fn range_3(&self, limit: u64, extra: u64) -> impl Iterator<Item = u64> {
        let (from, to, _) = self.from_to_offset_exclusion();
        (to + extra)..(2 * to - from + extra + limit)
    }

    /// from → diff * (diff + 1) + limit + extra
    fn range_4(&self, limit: u64, extra: u64) -> impl Iterator<Item = u64> {
        let (from, to, _) = self.from_to_offset_exclusion();
        (from + extra)..((to - from) * (to - from + 1) + extra + limit)
    }

    /// from → 2 * (to + 1) + limit + extra
    fn range_5(&self, limit: u64, extra: u64) -> impl Iterator<Item = u64> {
        let (from, to, _) = self.from_to_offset_exclusion();
        (from + extra)..(2 * (to + 1) + extra + limit)
    }

    /// lower_limit → upper_limit + limit + extra
    fn range_6(&self, limit: u64, lower_limit: u64, upper_limit: u64) -> impl Iterator<Item = u64> {
        (lower_limit)..(upper_limit + limit)
    }

    /// from → 3 * to - 1 + limit + extra
    fn range_7(&self, limit: u64, extra: u64) -> impl Iterator<Item = u64> {
        let (from, to, _) = self.from_to_offset_exclusion();
        (from + extra)..(3 * to - 1 + extra + limit)
    }

    /// 0 → to + limit + extra
    fn range_8(&self, limit: u64, extra: u64) -> impl Iterator<Item = u64> {
        let (_, to, _) = self.from_to_offset_exclusion();
        0..(to + extra + limit)
    }

    /// to - 1 → 0 + limit  + extra
    fn range_9(&self, limit: u64, extra: u64) -> impl Iterator<Item = u64> {
        let (_, to, _) = self.from_to_offset_exclusion();
        (to - 1 + extra)..(limit + extra)
    }

    // Create the whole matrix of index according to line and column
    fn diag(&self) -> Vec<(u64, u64, u64)> {
        let (from, to, _) = self.from_to_offset_exclusion();
        let diff = to - from;

        let mut column = 0;
        let mut line = 0;

        // Create the upper diag
        (0..(diff * (diff + 1) / 2))
            .map(|i| {
                if line == column {
                    column += 1;
                } else if column >= (to - 1) {
                    line += 1;
                    column = line + 1;
                } else {
                    column += 1;
                }
                (line + 1, column + 1, i + 1)
            })
            .collect()
    }

    // Create the whole matrix of index according to line and column
    fn matrix(&self) -> Vec<Vec<(u64, u64, u64)>> {
        let diag = self.diag();

        // Create the whole matrix
        self.range_0(0, 0, 1)
            .map(|i| {
                let temp = diag
                    .iter()
                    .filter_map(|(k, l, m)| {
                        if i == *k || i == *l {
                            std::option::Option::Some((*k, *l, *m))
                        } else {
                            std::option::Option::None
                        }
                    })
                    .collect();

                temp
            })
            .collect()
    }

    fn get_tuple_diag(&self, diag: &[(u64, u64, u64)], i: u64) -> (u64, u64, u64) {
        let mut iter_diag = diag.iter();

        if let Some((k, l, m)) = iter_diag.nth((i - 1).try_into().unwrap()) {
            (*k, *l, *m)
        } else {
            (0, 0, 0)
        }
    }

    fn get_tuple_matrix(&self, matrix: &[Vec<(u64, u64, u64)>], i: u64, j: u64) -> (u64, u64, u64) {
        let mut iter_matrix = matrix.iter();

        let mut list: Vec<(u64, u64, u64)> = Vec::new();

        if let Some(temp) = iter_matrix.nth((i - 1).try_into().unwrap()) {
            list = temp.to_vec();
        };

        let mut iter_list = list.iter();

        if let Some((k, l, m)) = iter_list.nth((j - 1).try_into().unwrap()) {
            (*k, *l, *m)
        } else {
            (0, 0, 0)
        }
    }

    // TODO
    //i.to_string().parse::<u64>().unwrap() → something better
    fn unwrap_literal(&self, i: &proc_macro2::Literal) -> u64 {
        i.to_string().parse::<u64>().unwrap()
    }

    fn expand2(
        &self,
        tt: proc_macro2::TokenTree,
        rest: &mut proc_macro2::token_stream::IntoIter,
        mutated: &mut bool,
        modes: (Mode, Mode, Mode),
        diag: &[(u64, u64, u64)],
        matrix: &[Vec<(u64, u64, u64)>],
    ) -> proc_macro2::TokenStream {
        let tt = match tt {
            proc_macro2::TokenTree::Group(g) => {
                let (expanded, g_mutated) = self.expand_pass(g.stream(), modes, diag, matrix);
                let mut expanded = proc_macro2::Group::new(g.delimiter(), expanded);
                *mutated |= g_mutated;
                expanded.set_span(g.span());
                proc_macro2::TokenTree::Group(expanded)
            }
            proc_macro2::TokenTree::Ident(ref ident) if ident == &self.ident => {
                if let Mode::ReplaceIdent(i) = modes.0 {
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
                match (
                    modes.0,
                    modes.1,
                    modes.2,
                    peek.next(),
                    peek.next(),
                    peek.next(),
                    peek.next(),
                ) {
                    (
                        Mode::ReplaceIdent(i),
                        _,
                        _,
                        Some(proc_macro2::TokenTree::Punct(ref punct)),
                        Some(proc_macro2::TokenTree::Ident(ref ident2)),
                        Some(proc_macro2::TokenTree::Punct(ref sign)),
                        Some(proc_macro2::TokenTree::Literal(ref select_mode)),
                    ) if punct.as_char() == '#'
                        && ident2 == &self.ident
                        && sign.as_char() == ':' =>
                    {
                        // have seen ident # N : n
                        ident = match self.unwrap_literal(select_mode) {
                            0 => proc_macro2::Ident::new(&format!("{}{}", ident, i), ident.span()),
                            1 => {
                                let mode = if let Mode::ReplaceIdent(j) = modes.1 {
                                    j
                                } else {
                                    0
                                };

                                proc_macro2::Ident::new(&format!("{}{}", ident, mode), ident.span())
                            }
                            2 => {
                                let mode = if let Mode::ReplaceIdent(j) = modes.2 {
                                    j
                                } else {
                                    0
                                };

                                proc_macro2::Ident::new(&format!("{}{}", ident, mode), ident.span())
                            }
                            3 => {
                                // Left branch (3K-2)
                                let (_, to, _) = self.from_to_offset_exclusion();
                                proc_macro2::Ident::new(
                                    &format!("{}{}", ident, 3 * to - 2),
                                    ident.span(),
                                )
                            }
                            4 => {
                                // Right branch (3K-1)
                                let (_, to, _) = self.from_to_offset_exclusion();
                                proc_macro2::Ident::new(
                                    &format!("{}{}", ident, 3 * to - 1),
                                    ident.span(),
                                )
                            }
                            5 => {
                                let (k, l, _) = self.get_tuple_diag(diag, i);
                                proc_macro2::Ident::new(
                                    &format!("{}{}_{}", ident, k, l),
                                    ident.span(),
                                )
                            }
                            6 => {
                                let (k, l, _) = self.get_tuple_diag(diag, i);
                                proc_macro2::Ident::new(
                                    &format!("{}{}_{}", ident, l, k),
                                    ident.span(),
                                )
                            }
                            7 => proc_macro2::Ident::new(
                                &format!("{}{}", ident, 3 * (i - 1) + 1),
                                ident.span(),
                            ),
                            _ => ident,
                        };

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
                        Mode::ReplaceIdent(i),
                        Mode::ReplaceIdent(j),
                        _,
                        Some(proc_macro2::TokenTree::Punct(ref punct)),
                        Some(proc_macro2::TokenTree::Ident(ref ident2)),
                        Some(proc_macro2::TokenTree::Punct(ref sign)),
                        Some(proc_macro2::TokenTree::Literal(ref select_mode)),
                    ) if punct.as_char() == '~'
                        && ident2 == &self.ident
                        && sign.as_char() == ':' =>
                    {
                        // have seen ident ~ N : n
                        ident = match self.unwrap_literal(select_mode) {
                            0 => {
                                let (k, l, _) = self.get_tuple_matrix(matrix, i, j);
                                proc_macro2::Ident::new(
                                    &format!("{}{}_{}", ident, k, l),
                                    ident.span(),
                                )
                            }
                            1 => {
                                let (k, l, _) = self.get_tuple_matrix(matrix, i, j);
                                proc_macro2::Ident::new(
                                    &format!("{}{}_{}", ident, l, k),
                                    ident.span(),
                                )
                            }
                            2 => proc_macro2::Ident::new(
                                &format!("{}{}", ident, i + j),
                                ident.span(),
                            ),
                            3 => {
                                let (from, to, _) = self.from_to_offset_exclusion();
                                let diff = to - from;

                                proc_macro2::Ident::new(
                                    &format!("{}{}", ident, diff * (diff + 1) / 2 + to + i + j),
                                    ident.span(),
                                )
                            }
                            4 => proc_macro2::Ident::new(
                                &format!("{}{}", ident, 3 * (i - 1) + 1 + j),
                                ident.span(),
                            ),
                            _ => ident,
                        };

                        *rest = peek.clone();
                        *mutated = true;

                        // we may need to also consume another #
                        match peek.next() {
                            Some(proc_macro2::TokenTree::Punct(ref punct))
                                if punct.as_char() == '~' =>
                            {
                                *rest = peek;
                            }
                            _ => {}
                        }
                    }
                    // ^ % |
                    _ => {}
                }

                proc_macro2::TokenTree::Ident(ident)
            }
            proc_macro2::TokenTree::Punct(ref p) if p.as_char() == '#' => {
                let mut peek = rest.clone();
                match (peek.next(), peek.next(), peek.next(), peek.next()) {
                    // is this #(...)m:n ?
                    (
                        Some(proc_macro2::TokenTree::Group(ref rep)),
                        Some(proc_macro2::TokenTree::Literal(ref range)),
                        Some(proc_macro2::TokenTree::Punct(ref sign)),
                        Some(proc_macro2::TokenTree::Literal(ref extra)),
                    ) if rep.delimiter() == proc_macro2::Delimiter::Parenthesis
                        && sign.as_char() == ':' =>
                    {
                        // yes! expand ... for each sequence in the range
                        *mutated = true;
                        *rest = peek;

                        match self.unwrap_literal(range) {
                            0 => {
                                return self
                                    .range_0(0, self.unwrap_literal(extra), 0)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            diag,
                                            matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            1 => {
                                return self
                                    .range_1(0, self.unwrap_literal(extra))
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            diag,
                                            matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            2 => {
                                return self
                                    .range_2(0, self.unwrap_literal(extra))
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            diag,
                                            matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            3 => {
                                return self
                                    .range_3(0, self.unwrap_literal(extra))
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            diag,
                                            matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            4 => {
                                return self
                                    .range_4(0, self.unwrap_literal(extra))
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            diag,
                                            matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            5 => {
                                return self
                                    .range_5(0, self.unwrap_literal(extra))
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            diag,
                                            matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            6 => {
                                let mode = if let Mode::ReplaceIdent(i) = modes.0 {
                                    i + self.unwrap_literal(extra)
                                } else {
                                    self.unwrap_literal(extra)
                                };

                                return self
                                    .range_6(0, mode, mode + 3)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            diag,
                                            matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            7 => {
                                let mode = if let Mode::ReplaceIdent(i) = modes.0 {
                                    i + self.unwrap_literal(extra)
                                } else {
                                    self.unwrap_literal(extra)
                                };

                                return self
                                    .range_6(0, mode, mode + 2)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            diag,
                                            matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            8 => {
                                let (from, to, _) = self.from_to_offset_exclusion();
                                let mode = if let Mode::ReplaceIdent(i) = modes.0 {
                                    3 * (to - from) + i + self.unwrap_literal(extra)
                                } else {
                                    3 * (to - from) + self.unwrap_literal(extra)
                                };

                                return self
                                    .range_6(0, mode, mode + 2)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            diag,
                                            matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            9 => {
                                return self
                                    .range_7(0, self.unwrap_literal(extra))
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            diag,
                                            matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            10 => {
                                return self
                                    .range_4(1, self.unwrap_literal(extra))
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            diag,
                                            matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            11 => {
                                return self
                                    .range_7(1, self.unwrap_literal(extra))
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            diag,
                                            matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            12 => {
                                return self
                                    .range_9(0, self.unwrap_literal(extra))
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            diag,
                                            matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            _ => {}
                        };
                    }
                    _ => {}
                }
                proc_macro2::TokenTree::Punct(p.clone())
            }
            proc_macro2::TokenTree::Punct(ref p) if p.as_char() == '%' => {
                // is this %(...)(...)n* ?
                let mut peek = rest.clone();
                match (peek.next(), peek.next(), peek.next(), peek.next()) {
                    (
                        Some(proc_macro2::TokenTree::Group(ref rep_if)),
                        Some(proc_macro2::TokenTree::Group(ref rep_else)),
                        Some(proc_macro2::TokenTree::Literal(ref select_mode)),
                        Some(proc_macro2::TokenTree::Punct(ref star)),
                    ) if rep_if.delimiter() == proc_macro2::Delimiter::Parenthesis
                        && rep_else.delimiter() == proc_macro2::Delimiter::Parenthesis
                        && star.as_char() == '*' =>
                    {
                        // yes! expand ... for each sequence in the range
                        *mutated = true;
                        *rest = peek;

                        match self.unwrap_literal(select_mode) {
                            0 => {
                                return self
                                    .range_0(0, 0, 0)
                                    .filter_map(|j| {
                                        if j == self.exclusion.base10_parse::<u64>().unwrap()
                                            && self.objection
                                        {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (Mode::ReplaceIdent(j), modes.1, modes.2),
                                                diag,
                                                matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (Mode::ReplaceIdent(j), modes.1, modes.2),
                                                diag,
                                                matrix,
                                            ))
                                        }
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            1 => {
                                return self
                                    .range_0(0, 0, 1)
                                    .map(|j| {
                                        self.expand_pass(
                                            rep_if.stream(),
                                            (Mode::ReplaceIdent(j), modes.1, modes.2),
                                            diag,
                                            matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
                proc_macro2::TokenTree::Punct(p.clone())
            }
            proc_macro2::TokenTree::Punct(ref p) if p.as_char() == '~' => {
                // is this ~(...)(...)n* ?
                let mut peek = rest.clone();
                match (modes.0, peek.next(), peek.next(), peek.next(), peek.next()) {
                    (
                        Mode::ReplaceIdent(i),
                        Some(proc_macro2::TokenTree::Group(ref rep_if)),
                        Some(proc_macro2::TokenTree::Group(ref rep_else)),
                        Some(proc_macro2::TokenTree::Literal(ref select_mode)),
                        Some(proc_macro2::TokenTree::Punct(ref star)),
                    ) if rep_if.delimiter() == proc_macro2::Delimiter::Parenthesis
                        && rep_else.delimiter() == proc_macro2::Delimiter::Parenthesis
                        && star.as_char() == '*' =>
                    {
                        // yes! expand ... for each sequence in the range
                        *mutated = true;
                        *rest = peek;

                        match self.unwrap_literal(select_mode) {
                            0 => {
                                return self
                                    .range_0(0, 0, 0)
                                    .filter_map(|j| {
                                        let (k, _, _) = self.get_tuple_matrix(matrix, i, j);

                                        if k != i {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                diag,
                                                matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                diag,
                                                matrix,
                                            ))
                                        }
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            1 => {
                                return self
                                    .range_8(0, 0)
                                    .filter_map(|j| {
                                        let (k, _, _) = self.get_tuple_matrix(matrix, i, j);

                                        if k != i {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                diag,
                                                matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                diag,
                                                matrix,
                                            ))
                                        }
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            2 => {
                                return self
                                    .range_7(1, 0)
                                    .map(|j| {
                                        self.expand_pass(
                                            rep_if.stream(),
                                            (modes.0, Mode::ReplaceIdent(j), modes.2),
                                            diag,
                                            matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            _ => {}
                        }
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
        modes: (Mode, Mode, Mode),
        diag: &[(u64, u64, u64)],
        matrix: &[Vec<(u64, u64, u64)>],
    ) -> (proc_macro2::TokenStream, bool) {
        let mut out = proc_macro2::TokenStream::new();
        let mut mutated = false;
        let mut tts = stream.into_iter();
        while let Some(tt) = tts.next() {
            out.extend(self.expand2(tt, &mut tts, &mut mutated, modes, diag, matrix));
        }
        (out, mutated)
    }

    fn expand(
        &self,
        stream: proc_macro2::TokenStream,
        diag: &[(u64, u64, u64)],
        matrix: &[Vec<(u64, u64, u64)>],
    ) -> proc_macro2::TokenStream {
        let (out, mutated) = self.expand_pass(
            stream.clone(),
            (
                Mode::ReplaceSequence,
                Mode::ReplaceSequence,
                Mode::ReplaceSequence,
            ),
            diag,
            matrix,
        );
        if mutated {
            return out;
        }

        self.range_0(0, 0, 0)
            .map(|i| {
                self.expand_pass(
                    stream.clone(),
                    (
                        Mode::ReplaceIdent(i),
                        Mode::ReplaceSequence,
                        Mode::ReplaceSequence,
                    ),
                    diag,
                    matrix,
                )
            })
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
