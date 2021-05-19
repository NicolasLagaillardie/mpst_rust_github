#![recursion_limit = "128"]

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use std::convert::TryFrom;
use std::usize;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Result, Token};

mod functionmpst;

use functionmpst::recv::RecvAuxSimpleMacroInput;

type Diag = Vec<(u64, u64, u64)>;
type Matrix = Vec<Vec<(u64, u64, u64)>>;

#[derive(Debug)]
struct SeqMacroInput {
    from: syn::LitInt,
    to: syn::LitInt,
    inclusive: bool,
    objection: bool,
    exclusion: syn::LitInt,
    labels: proc_macro2::TokenStream,
    receivers: proc_macro2::TokenStream,
    role_name: proc_macro2::TokenStream,
    ident: syn::Ident,
    tt: proc_macro2::TokenStream,
}

impl Parse for SeqMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // identifier for indexing
        let ident = syn::Ident::parse(input)?;
        let _in = <Token![in]>::parse(input)?;
        // limits (lower and upper) and possible inclusion of upper limit
        let from = syn::LitInt::parse(input)?;
        let inclusive = input.peek(Token![..=]);
        if inclusive {
            <Token![..=]>::parse(input)?;
        } else {
            <Token![..]>::parse(input)?;
        }
        let to = syn::LitInt::parse(input)?;
        // exclusion of one index
        let objection = input.peek(Token![!]);
        let exclusion = if objection {
            <Token![!]>::parse(input)?;
            syn::LitInt::parse(input)?
        } else {
            syn::LitInt::new("0", proc_macro2::Span::call_site())
        };
        // labels for recursion (choose_mpst_X_to_all)
        let start_labels = input.peek(Token![:]);
        let labels = if start_labels {
            <Token![:]>::parse(input)?;
            let content_labels;
            let _parentheses = syn::parenthesized!(content_labels in input);
            proc_macro2::TokenStream::parse(&content_labels)?
        } else {
            proc_macro2::TokenStream::new()
        };
        // receivers for recursion (choose_mpst_X_to_all)
        let start_receivers = input.peek(Token![:]);
        let receivers = if start_receivers {
            <Token![:]>::parse(input)?;
            let content_receivers;
            let _parentheses = syn::parenthesized!(content_receivers in input);
            proc_macro2::TokenStream::parse(&content_receivers)?
        } else {
            proc_macro2::TokenStream::new()
        };
        // role name for creating normal
        let start_role_name = input.peek(Token![>]);
        let role_name = if start_role_name {
            <Token![>]>::parse(input)?;
            let content_role_name;
            let _parentheses = syn::parenthesized!(content_role_name in input);
            proc_macro2::TokenStream::parse(&content_role_name)?
        } else {
            proc_macro2::TokenStream::new()
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
            labels,
            receivers,
            role_name,
            ident,
            tt,
        })
    }
}

impl Into<proc_macro2::TokenStream> for SeqMacroInput {
    fn into(self) -> proc_macro2::TokenStream {
        self.expand(
            self.tt.clone(),
            self.labels.clone(),
            self.receivers.clone(),
            self.role_name.clone(),
            (
                &self.diag(false),
                &self.matrix(false),
                &self.diag_w_offset(false),
                &self.matrix_w_offset(false),
                &self.diag(true),
                &self.matrix(true),
                &self.diag_w_offset(true),
                &self.matrix_w_offset(true),
            ),
        )
    }
}

#[derive(Copy, Clone, Debug)]
enum Mode {
    ReplaceIdent(u64),
    ReplaceSequence,
}

impl SeqMacroInput {
    /// from, to + offset, exclusion, number_roles
    fn from_to_offset_exclusion(&self) -> (u64, u64, u64, u64) {
        let from = (self.from).base10_parse::<u64>().unwrap();
        let to = (self.to).base10_parse::<u64>().unwrap();
        let exclusion = (self.exclusion).base10_parse::<u64>().unwrap();
        let offset = if self.inclusive { 1 } else { 0 };
        let number_roles = self
            .expand_pass_options(self.role_name.clone())
            .len()
            .to_string()
            .parse::<u64>()
            .unwrap();
        (from, to + offset, exclusion, number_roles)
    }

    /// from → to + limit + extra
    fn one(&self) -> impl Iterator<Item = u64> {
        (1)..(2)
    }

    /// from → to + limit + extra
    fn range_0(
        &self,
        limit: u64,
        extra: u64,
        include: u64,
        roles_size: bool,
    ) -> impl Iterator<Item = u64> {
        if roles_size {
            let (from, _, _, number_roles) = self.from_to_offset_exclusion();
            (from + extra)..(number_roles + include + extra + limit)
        } else {
            let (from, to, _, _) = self.from_to_offset_exclusion();
            (from + extra)..(to + include + extra + limit)
        }
    }

    /// from → 3 * (to - 1) + limit + extra
    fn range_1(&self, limit: u64, extra: u64, roles_size: bool) -> impl Iterator<Item = u64> {
        if roles_size {
            let (from, _, _, number_roles) = self.from_to_offset_exclusion();
            (from + extra)..(3 * (number_roles - 1) + extra + limit)
        } else {
            let (from, to, _, _) = self.from_to_offset_exclusion();
            (from + extra)..(3 * (to - 1) + extra + limit)
        }
    }

    /// from → 2 * (to) - 1 + limit + extra
    fn range_2(&self, limit: u64, extra: u64, roles_size: bool) -> impl Iterator<Item = u64> {
        if roles_size {
            let (from, _, _, number_roles) = self.from_to_offset_exclusion();
            (from + extra)..(2 * number_roles - 1 + extra + limit)
        } else {
            let (from, to, _, _) = self.from_to_offset_exclusion();
            (from + extra)..(2 * to - 1 + extra + limit)
        }
    }

    /// to → 2 * to - from + extra + limit + extra
    fn range_3(&self, limit: u64, extra: u64, roles_size: bool) -> impl Iterator<Item = u64> {
        if roles_size {
            let (from, _, _, number_roles) = self.from_to_offset_exclusion();
            (number_roles + extra)..(2 * number_roles - from + extra + limit)
        } else {
            let (from, to, _, _) = self.from_to_offset_exclusion();
            (to + extra)..(2 * to - from + extra + limit)
        }
    }

    /// from + extra → diff * (diff + 1) + limit + extra
    fn range_4(&self, limit: u64, extra: u64, roles_size: bool) -> impl Iterator<Item = u64> {
        if roles_size {
            let (from, _, _, number_roles) = self.from_to_offset_exclusion();
            (from + extra)..((number_roles - from) * (number_roles - from + 1) + extra + limit)
        } else {
            let (from, to, _, _) = self.from_to_offset_exclusion();
            (from + extra)..((to - from) * (to - from + 1) + extra + limit)
        }
    }

    /// from → 2 * (to + 1) + limit + extra
    fn range_5(&self, limit: u64, extra: u64, roles_size: bool) -> impl Iterator<Item = u64> {
        if roles_size {
            let (from, _, _, number_roles) = self.from_to_offset_exclusion();
            (from + extra)..(2 * (number_roles + 1) + extra + limit)
        } else {
            let (from, to, _, _) = self.from_to_offset_exclusion();
            (from + extra)..(2 * (to + 1) + extra + limit)
        }
    }

    /// lower_limit → upper_limit + limit
    fn range_6(&self, limit: u64, lower_limit: u64, upper_limit: u64) -> impl Iterator<Item = u64> {
        (lower_limit)..(upper_limit + limit)
    }

    /// from → 3 * to - 1 + limit + extra
    fn range_7(&self, limit: u64, extra: u64, roles_size: bool) -> impl Iterator<Item = u64> {
        if roles_size {
            let (from, _, _, number_roles) = self.from_to_offset_exclusion();
            (from + extra)..(3 * number_roles - 1 + extra + limit)
        } else {
            let (from, to, _, _) = self.from_to_offset_exclusion();
            (from + extra)..(3 * to - 1 + extra + limit)
        }
    }

    /// 0 → to + limit + extra
    fn range_8(&self, limit: u64, extra: u64, roles_size: bool) -> impl Iterator<Item = u64> {
        if roles_size {
            let (_, _, _, number_roles) = self.from_to_offset_exclusion();
            0..(number_roles + extra + limit)
        } else {
            let (_, to, _, _) = self.from_to_offset_exclusion();
            0..(to + extra + limit)
        }
    }

    /// to + 1 → 0 + limit  + extra
    fn range_9(&self, limit: u64, extra: u64, roles_size: bool) -> impl Iterator<Item = u64> {
        if roles_size {
            let (_, _, _, number_roles) = self.from_to_offset_exclusion();
            ((limit + extra + 1)..(number_roles + extra)).rev()
        } else {
            let (_, to, _, _) = self.from_to_offset_exclusion();
            ((limit + extra + 1)..(to + extra)).rev()
        }
    }

    /// 3 * to - 2 + extra → 3 * to + limit + extra
    fn range_10(&self, limit: u64, extra: u64, roles_size: bool) -> impl Iterator<Item = u64> {
        if roles_size {
            let (_, _, _, number_roles) = self.from_to_offset_exclusion();
            (3 * number_roles - 2 + extra)..(3 * number_roles + limit + extra)
        } else {
            let (_, to, _, _) = self.from_to_offset_exclusion();
            (3 * to - 2 + extra)..(3 * to + limit + extra)
        }
    }

    /// 1 + extra → 4 + extra + limit
    fn range_11(&self, limit: u64, extra: u64) -> impl Iterator<Item = u64> {
        (1 + extra)..(4 + extra + limit)
    }

    /// from + extra → diff * (diff + 1) / 2 + limit + extra
    fn range_12(&self, limit: u64, extra: u64, roles_size: bool) -> impl Iterator<Item = u64> {
        if roles_size {
            let (from, _, _, number_roles) = self.from_to_offset_exclusion();
            (from + extra)..((number_roles - from) * (number_roles - from + 1) / 2 + extra + limit)
        } else {
            let (from, to, _, _) = self.from_to_offset_exclusion();
            (from + extra)..((to - from) * (to - from + 1) / 2 + extra + limit)
        }
    }

    /// from + extra → to + include + extra + limit - 1
    fn range_13(
        &self,
        limit: u64,
        extra: u64,
        include: u64,
        roles_size: bool,
    ) -> impl Iterator<Item = u64> {
        if roles_size {
            let (from, _, _, number_roles) = self.from_to_offset_exclusion();
            (from + extra)..(number_roles + include + extra + limit - 1)
        } else {
            let (from, to, _, _) = self.from_to_offset_exclusion();
            (from + extra)..(to + include + extra + limit - 1)
        }
    }

    /// from + extra → diff * (diff + 1) / 2 + limit + extra (- offset)
    fn range_14(&self, limit: u64, extra: u64, roles_size: bool) -> impl Iterator<Item = u64> {
        if roles_size {
            let (from, _, _, number_roles) = self.from_to_offset_exclusion();
            let number_roles = number_roles - if self.inclusive { 1 } else { 0 };
            (from + extra)..((number_roles - from) * (number_roles - from + 1) / 2 + extra + limit)
        } else {
            let (from, to, _, _) = self.from_to_offset_exclusion();
            let to = to - if self.inclusive { 1 } else { 0 };
            (from + extra)..((to - from) * (to - from + 1) / 2 + extra + limit)
        }
    }

    /// from + extra → to + include + extra + limit - offset
    fn range_15(
        &self,
        limit: u64,
        extra: u64,
        include: u64,
        roles_size: bool,
    ) -> impl Iterator<Item = u64> {
        if roles_size {
            let (from, _, _, number_roles) = self.from_to_offset_exclusion();
            let number_roles = number_roles - if self.inclusive { 1 } else { 0 };
            (from + extra)..(number_roles + include + extra + limit)
        } else {
            let (from, to, _, _) = self.from_to_offset_exclusion();
            let to = to - if self.inclusive { 1 } else { 0 };
            (from + extra)..(to + include + extra + limit)
        }
    }

    /// from + extra → to + include + extra + limit + 1
    fn range_16(
        &self,
        limit: u64,
        extra: u64,
        include: u64,
        roles_size: bool,
    ) -> impl Iterator<Item = u64> {
        if roles_size {
            let (from, _, _, number_roles) = self.from_to_offset_exclusion();
            (from + extra)..(number_roles + include + extra + limit + 1)
        } else {
            let (from, to, _, _) = self.from_to_offset_exclusion();
            (from + extra)..(to + include + extra + limit + 1)
        }
    }

    /// lower_limit → upper_limit + limit + to
    fn range_17(
        &self,
        limit: u64,
        lower_limit: u64,
        upper_limit: u64,
        roles_size: bool,
    ) -> impl Iterator<Item = u64> {
        if roles_size {
            let (_, _, _, number_roles) = self.from_to_offset_exclusion();
            (lower_limit)..(upper_limit + limit + number_roles)
        } else {
            let (_, to, _, _) = self.from_to_offset_exclusion();
            (lower_limit)..(upper_limit + limit + to)
        }
    }

    /// from + extra + lower → diff * (diff + 1) / 2 + limit + extra
    fn range_18(
        &self,
        limit: u64,
        extra: u64,
        lower: u64,
        roles_size: bool,
    ) -> impl Iterator<Item = u64> {
        if roles_size {
            let (from, _, _, number_roles) = self.from_to_offset_exclusion();
            (from + extra + lower)
                ..((number_roles - from) * (number_roles - from + 1) / 2 + extra + limit)
        } else {
            let (from, to, _, _) = self.from_to_offset_exclusion();
            (from + extra + lower)..((to - from) * (to - from + 1) / 2 + extra + limit)
        }
    }

    /// from + extra + lower → to + include + extra + limit + 1
    fn range_19(
        &self,
        limit: u64,
        extra: u64,
        include: u64,
        lower: u64,
        roles_size: bool,
    ) -> impl Iterator<Item = u64> {
        if roles_size {
            let (from, _, _, number_roles) = self.from_to_offset_exclusion();
            (from + extra + lower)..(number_roles + include + extra + limit + 1)
        } else {
            let (from, to, _, _) = self.from_to_offset_exclusion();
            (from + extra + lower)..(to + include + extra + limit + 1)
        }
    }

    /// from + lower → to + limit + extra
    fn range_20(
        &self,
        limit: u64,
        extra: u64,
        include: u64,
        lower: u64,
        roles_size: bool,
    ) -> impl Iterator<Item = u64> {
        if roles_size {
            let (from, _, _, number_roles) = self.from_to_offset_exclusion();
            (from + extra + lower)..(number_roles + include + extra + limit)
        } else {
            let (from, to, _, _) = self.from_to_offset_exclusion();
            (from + extra + lower)..(to + include + extra + limit)
        }
    }

    /// from → 2 * to + limit + extra
    fn range_23(&self, limit: u64, extra: u64, roles_size: bool) -> impl Iterator<Item = u64> {
        if roles_size {
            let (from, _, _, number_roles) = self.from_to_offset_exclusion();
            (from + extra)..(2 * number_roles + extra + limit)
        } else {
            let (from, to, _, _) = self.from_to_offset_exclusion();
            (from + extra)..(2 * to + extra + limit)
        }
    }

    /// 2 * to - 1 + extra → 2 * to + + 1 limit + extra
    fn range_24(&self, limit: u64, extra: u64, roles_size: bool) -> impl Iterator<Item = u64> {
        if roles_size {
            let (_, _, _, number_roles) = self.from_to_offset_exclusion();
            (2 * number_roles - 1 + extra)..(2 * number_roles + 1 + limit + extra)
        } else {
            let (_, to, _, _) = self.from_to_offset_exclusion();
            (2 * to - 1 + extra)..(2 * to + 1 + limit + extra)
        }
    }

    /// Create the whole matrix of index according to line and column
    fn diag(&self, roles_size: bool) -> Vec<(u64, u64, u64)> {
        let (from, to) = if roles_size {
            let (from, _, _, mut to) = self.from_to_offset_exclusion();
            if to <= from {
                let result = self.from_to_offset_exclusion();
                to = result.1;
            }
            (from, to)
        } else {
            let (from, to, _, _) = self.from_to_offset_exclusion();
            (from, to)
        };
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

    /// Create the whole matrix of index according to line and column
    fn matrix(&self, roles_size: bool) -> Vec<Vec<(u64, u64, u64)>> {
        let diag = self.diag(roles_size);
        let (from, _, _, number_roles) = self.from_to_offset_exclusion();

        // Create the whole matrix
        self.range_0(0, 0, 1, roles_size && (number_roles > from))
            .map(|i| {
                let temp = diag
                    .iter()
                    .filter_map(|(line, column, index)| {
                        if i == *line || i == *column {
                            std::option::Option::Some((*line, *column, *index))
                        } else {
                            std::option::Option::None
                        }
                    })
                    .collect();

                temp
            })
            .collect()
    }

    /// Create the whole matrix of index according to line and column
    fn diag_w_offset(&self, roles_size: bool) -> Vec<(u64, u64, u64)> {
        let (from, to) = if roles_size {
            let (from, _, _, mut to) = self.from_to_offset_exclusion();
            if to <= from {
                let result = self.from_to_offset_exclusion();
                to = result.1;
            }
            (from, to)
        } else {
            let (from, to, _, _) = self.from_to_offset_exclusion();
            (from, to)
        };
        let to = to - if self.inclusive { 1 } else { 0 };
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

    /// Create the whole matrix of index according to line and column
    fn matrix_w_offset(&self, roles_size: bool) -> Vec<Vec<(u64, u64, u64)>> {
        let diag = self.diag_w_offset(roles_size);
        let (from, _, _, number_roles) = self.from_to_offset_exclusion();

        // Create the whole matrix
        self.range_15(0, 0, 1, roles_size && (number_roles > from))
            .map(|i| {
                let temp = diag
                    .iter()
                    .filter_map(|(line, column, index)| {
                        if i == *line || i == *column {
                            std::option::Option::Some((*line, *column, *index))
                        } else {
                            std::option::Option::None
                        }
                    })
                    .collect();

                temp
            })
            .collect()
    }

    /// Return (line, column, index) of diag
    fn get_tuple_diag(&self, diag: &[(u64, u64, u64)], i: u64) -> (u64, u64, u64) {
        if let Some((line, column, index)) = diag.get(usize::try_from(i - 1).unwrap()) {
            (*line, *column, *index)
        } else {
            (0, 0, 0)
        }
    }

    /// Return (line, column, index) of matrix
    fn get_tuple_matrix(&self, matrix: &[Vec<(u64, u64, u64)>], i: u64, j: u64) -> (u64, u64, u64) {
        let list: Vec<(u64, u64, u64)> =
            if let Some(temp) = matrix.get(usize::try_from(i - 1).unwrap()) {
                temp.to_vec()
            } else {
                Vec::new()
            };

        if let Some((line, column, index)) = list.get(usize::try_from(j - 1).unwrap()) {
            (*line, *column, *index)
        } else {
            (0, 0, 0)
        }
    }

    /// Return (line, column) of diag from index
    fn get_line_column_from_diag(&self, diag: &[(u64, u64, u64)], index: u64) -> (u64, u64) {
        for i in diag {
            if i.2 == index {
                return (i.0, i.1);
            }
        }
        return (0, 0);
    }

    // /// Return (line, column) of matrix from index
    // fn get_line_column_from_matrix(&self, matrix: &[(u64, u64, u64)], index: u64) -> (u64, u64) {
    //     for i in matrix {
    //         if i.2 == index {
    //             return (i.0, i.1);
    //         }
    //     }
    //     return (0, 0);
    // }

    /// TODO
    ///i.to_string().parse::<u64>().unwrap() → something better
    fn unwrap_literal(&self, i: &proc_macro2::Literal) -> u64 {
        i.to_string().parse::<u64>().unwrap()
    }

    fn expand_options(
        &self,
        tt: proc_macro2::TokenTree,
        _rest: &mut proc_macro2::token_stream::IntoIter,
    ) -> std::option::Option<proc_macro2::TokenStream> {
        match tt {
            proc_macro2::TokenTree::Group(g) => Some(g.stream()),
            _ => None,
        }
    }

    fn expand2(
        &self,
        tt: proc_macro2::TokenTree,
        rest: &mut proc_macro2::token_stream::IntoIter,
        mutated: &mut bool,
        modes: (Mode, Mode, Mode),
        extracted: (
            &Vec<proc_macro2::TokenStream>, // labels
            &Vec<proc_macro2::TokenStream>, // receivers
            &Vec<proc_macro2::TokenStream>, // role_name
        ),
        diag_and_matrix: (
            &Diag,   // diag,
            &Matrix, // matrix,
            &Diag,   // diag_w_offset,
            &Matrix, // matrix_w_offset,
            &Diag,   // diag with true,
            &Matrix, // matrix with true,
            &Diag,   // diag_w_offset with true,
            &Matrix, // matrix_w_offset with true,
        ),
    ) -> proc_macro2::TokenStream {
        let mut extraction = false;
        let mut new_ident = proc_macro2::TokenStream::new();

        let tt = match tt {
            proc_macro2::TokenTree::Group(g) => {
                let (expanded, g_mutated) =
                    self.expand_pass(g.stream(), modes, extracted, diag_and_matrix);
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
                        _,
                        _,
                        _,
                        Some(proc_macro2::TokenTree::Punct(ref punct)),
                        Some(proc_macro2::TokenTree::Ident(ref ident2)),
                        Some(proc_macro2::TokenTree::Punct(ref sign)),
                        Some(proc_macro2::TokenTree::Literal(ref select_mode)),
                    ) if punct.as_char() == '^'
                        && ident2 == &self.ident
                        && sign.as_char() == ':' =>
                    {
                        // have seen ident ^ N : n
                        ident = match self.unwrap_literal(select_mode) {
                            0 => {
                                // Left branch (3K-2)
                                let (_, to, _, _) = self.from_to_offset_exclusion();
                                proc_macro2::Ident::new(
                                    &format!("{}{}", ident, 3 * to - 2),
                                    ident.span(),
                                )
                            }
                            1 => {
                                // Right branch (3K-1)
                                let (_, to, _, _) = self.from_to_offset_exclusion();
                                proc_macro2::Ident::new(
                                    &format!("{}{}", ident, 3 * to - 1),
                                    ident.span(),
                                )
                            }
                            2 => {
                                let (_, to, _, _) = self.from_to_offset_exclusion();
                                proc_macro2::Ident::new(&format!("{}{}", ident, to), ident.span())
                            }
                            3 => proc_macro2::Ident::new(
                                &format!("{}{}", ident, self.exclusion),
                                ident.span(),
                            ),
                            4 => {
                                let role_names = extracted.2;

                                if let Some(elt) = role_names.get(usize::try_from(0).unwrap()) {
                                    proc_macro2::Ident::new(&format!("Role{}", elt), ident.span())
                                } else {
                                    ident
                                }
                            }
                            5 => {
                                let role_names = extracted.2;

                                if let Some(elt) = role_names.get(usize::try_from(0).unwrap()) {
                                    let temp = &format!("next_{}", elt);
                                    let mut result = String::from("");
                                    for c in temp.chars() {
                                        result.push_str(&c.to_lowercase().to_string());
                                    }
                                    proc_macro2::Ident::new(&result, ident.span())
                                } else {
                                    ident
                                }
                            }
                            6 => {
                                let role_names = extracted.2;

                                if let Some(elt) = role_names.get(usize::try_from(0).unwrap()) {
                                    proc_macro2::Ident::new(
                                        &format!("Role{}Dual", elt),
                                        ident.span(),
                                    )
                                } else {
                                    ident
                                }
                            }
                            7 => {
                                let role_names = extracted.2;

                                if let Some(elt) = role_names.get(usize::try_from(0).unwrap()) {
                                    let temp = &format!("next_{}_dual", elt);
                                    let mut result = String::from("");
                                    for c in temp.chars() {
                                        result.push_str(&c.to_lowercase().to_string());
                                    }
                                    proc_macro2::Ident::new(&result, ident.span())
                                } else {
                                    ident
                                }
                            }
                            8 => {
                                let role_names = extracted.2;

                                if let Some(elt) = role_names.get(usize::try_from(0).unwrap()) {
                                    proc_macro2::Ident::new(
                                        &format!("Role{}toAll", elt),
                                        ident.span(),
                                    )
                                } else {
                                    ident
                                }
                            }
                            9 => {
                                let role_names = extracted.2;

                                if let Some(elt) = role_names.get(usize::try_from(0).unwrap()) {
                                    let temp = &format!("next_{}_to_all", elt);
                                    let mut result = String::from("");
                                    for c in temp.chars() {
                                        result.push_str(&c.to_lowercase().to_string());
                                    }
                                    proc_macro2::Ident::new(&result, ident.span())
                                } else {
                                    ident
                                }
                            }
                            10 => {
                                let role_names = extracted.2;

                                if let Some(elt) = role_names.get(usize::try_from(0).unwrap()) {
                                    proc_macro2::Ident::new(
                                        &format!("RoleAllto{}", elt),
                                        ident.span(),
                                    )
                                } else {
                                    ident
                                }
                            }
                            11 => {
                                let role_names = extracted.2;

                                if let Some(elt) = role_names.get(usize::try_from(0).unwrap()) {
                                    let temp = &format!("next_all_to_{}", elt);
                                    let mut result = String::from("");
                                    for c in temp.chars() {
                                        result.push_str(&c.to_lowercase().to_string());
                                    }
                                    proc_macro2::Ident::new(&result, ident.span())
                                } else {
                                    ident
                                }
                            }
                            12 => {
                                // ~0 with true
                                // Left branch (2K-1)
                                let (_, _, _, number_roles) = self.from_to_offset_exclusion();
                                proc_macro2::Ident::new(
                                    &format!("{}{}", ident, 2 * number_roles - 1),
                                    ident.span(),
                                )
                            }
                            13 => {
                                // ~1 with true
                                // Right branch (2K)
                                let (_, _, _, number_roles) = self.from_to_offset_exclusion();
                                proc_macro2::Ident::new(
                                    &format!("{}{}", ident, 2 * number_roles),
                                    ident.span(),
                                )
                            }
                            14 => {
                                // ~2 with true
                                let (_, _, _, number_roles) = self.from_to_offset_exclusion();
                                proc_macro2::Ident::new(
                                    &format!("{}{}", ident, number_roles),
                                    ident.span(),
                                )
                            }
                            _ => ident,
                        };

                        *rest = peek.clone();
                        *mutated = true;

                        // we may need to also consume another ~
                        match peek.next() {
                            Some(proc_macro2::TokenTree::Punct(ref punct))
                                if punct.as_char() == '^' =>
                            {
                                *rest = peek;
                            }
                            _ => {}
                        }
                    }
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
                                let (line, column, _) = self.get_tuple_diag(diag_and_matrix.0, i);
                                proc_macro2::Ident::new(
                                    &format!("{}{}_{}", ident, line, column),
                                    ident.span(),
                                )
                            }
                            4 => {
                                let (line, column, _) = self.get_tuple_diag(diag_and_matrix.0, i);
                                proc_macro2::Ident::new(
                                    &format!("{}{}_{}", ident, column, line),
                                    ident.span(),
                                )
                            }
                            5 => proc_macro2::Ident::new(
                                &format!("{}{}", ident, 3 * (i - 1) + 1),
                                ident.span(),
                            ),
                            6 => proc_macro2::Ident::new(
                                &format!("{}{}", ident, 3 * (i - 1) + 3),
                                ident.span(),
                            ),
                            7 => {
                                let (_, to, _, _) = self.from_to_offset_exclusion();
                                let (line, column, _) =
                                    self.get_tuple_matrix(diag_and_matrix.1, to, i);
                                match line {
                                    m if m == i => proc_macro2::Ident::new(
                                        &format!("{}{}_{}", ident, column, line),
                                        ident.span(),
                                    ),
                                    _ => proc_macro2::Ident::new(
                                        &format!("{}{}_{}", ident, line, column),
                                        ident.span(),
                                    ),
                                }
                            }
                            8 => {
                                let (from, to, _, _) = self.from_to_offset_exclusion();
                                let sum: u64 = (0..i).map(|j| to - from - j).sum();

                                proc_macro2::Ident::new(&format!("{}{}", ident, sum), ident.span())
                            }
                            9 => {
                                let (from, to, _, _) = self.from_to_offset_exclusion();
                                let diff = to - from;
                                let sum: u64 = (0..i).map(|j| to - from - j).sum();

                                proc_macro2::Ident::new(
                                    &format!("{}{}", ident, diff * (diff + 1) / 2 + sum),
                                    ident.span(),
                                )
                            }
                            10 => {
                                let (from, to, _, _) = self.from_to_offset_exclusion();
                                let diff = to - from;

                                proc_macro2::Ident::new(
                                    &format!("{}{}", ident, diff * (diff + 1) / 2 + i),
                                    ident.span(),
                                )
                            }
                            11 => proc_macro2::Ident::new(
                                &format!("{}{}", ident, 3 * (i - 1) + 2),
                                ident.span(),
                            ),
                            12 => {
                                let (line, column, _) = self.get_tuple_diag(diag_and_matrix.2, i);
                                proc_macro2::Ident::new(
                                    &format!("{}{}_{}", ident, line, column),
                                    ident.span(),
                                )
                            }
                            13 => {
                                let (line, column, _) = self.get_tuple_diag(diag_and_matrix.2, i);
                                proc_macro2::Ident::new(
                                    &format!("{}{}_{}", ident, column, line),
                                    ident.span(),
                                )
                            }
                            14 => proc_macro2::Ident::new(
                                &format!("{}{}", ident, i + 1),
                                ident.span(),
                            ),
                            15 => {
                                let labels = extracted.0;

                                if let Some(elt) = labels.get(usize::try_from(i - 1).unwrap()) {
                                    // ident will be unused in the end
                                    new_ident = elt.clone();
                                    extraction = true;
                                    ident
                                } else {
                                    ident
                                }
                            }
                            16 => {
                                let receivers = extracted.1;

                                if let Some(elt) = receivers.get(usize::try_from(i - 1).unwrap()) {
                                    // ident will be unused in the end
                                    new_ident = elt.clone();
                                    extraction = true;
                                    ident
                                } else {
                                    ident
                                }
                            }
                            17 => {
                                if i < self.exclusion.base10_parse::<u64>().unwrap() {
                                    proc_macro2::Ident::new(
                                        &format!("{}{}_{}", ident, self.exclusion, i),
                                        ident.span(),
                                    )
                                } else {
                                    proc_macro2::Ident::new(
                                        &format!("{}{}_{}", ident, self.exclusion, i + 1),
                                        ident.span(),
                                    )
                                }
                            }
                            18 => {
                                let temp = if i >= self.exclusion.base10_parse::<u64>().unwrap() {
                                    i + 1
                                } else {
                                    i
                                };
                                proc_macro2::Ident::new(&format!("{}{}", ident, temp), ident.span())
                            }
                            19 => proc_macro2::Ident::new(&format!("{}", i), ident.span()),
                            20 => {
                                let fn_send = extracted.0;

                                if let Some(elt) = fn_send.get(usize::try_from(i - 2).unwrap()) {
                                    // ident will be unused in the end
                                    new_ident = elt.clone();
                                    extraction = true;
                                    ident
                                } else {
                                    ident
                                }
                            }
                            21 => {
                                let labels = extracted.0;

                                if let Some(elt) = labels.get(usize::try_from(i - 2).unwrap()) {
                                    // ident will be unused in the end
                                    new_ident = elt.clone();
                                    extraction = true;
                                    ident
                                } else {
                                    ident
                                }
                            }
                            22 => {
                                let receivers = extracted.1;

                                if let Some(elt) = receivers.get(usize::try_from(i - 2).unwrap()) {
                                    // ident will be unused in the end
                                    new_ident = elt.clone();
                                    extraction = true;
                                    ident
                                } else {
                                    ident
                                }
                            }
                            23 => {
                                let role_names = extracted.2;

                                if let Some(elt) = role_names.get(usize::try_from(i - 1).unwrap()) {
                                    proc_macro2::Ident::new(&format!("Role{}", elt), ident.span())
                                } else {
                                    ident
                                }
                            }
                            24 => {
                                let role_names = extracted.2;

                                if let Some(elt) = role_names.get(usize::try_from(i - 1).unwrap()) {
                                    let temp = &format!("next_{}", elt);
                                    let mut result = String::from("");
                                    for c in temp.chars() {
                                        result.push_str(&c.to_lowercase().to_string());
                                    }
                                    proc_macro2::Ident::new(&result, ident.span())
                                } else {
                                    ident
                                }
                            }
                            25 => {
                                let role_names = extracted.2;

                                if let Some(elt) = role_names.get(usize::try_from(i - 1).unwrap()) {
                                    proc_macro2::Ident::new(
                                        &format!("Role{}Dual", elt),
                                        ident.span(),
                                    )
                                } else {
                                    ident
                                }
                            }
                            26 => {
                                let role_names = extracted.2;

                                if let Some(elt) = role_names.get(usize::try_from(i - 1).unwrap()) {
                                    let temp = &format!("next_{}_dual", elt);
                                    let mut result = String::from("");
                                    for c in temp.chars() {
                                        result.push_str(&c.to_lowercase().to_string());
                                    }
                                    proc_macro2::Ident::new(&result, ident.span())
                                } else {
                                    ident
                                }
                            }
                            27 => {
                                let role_names = extracted.2;

                                if let Some(elt) = role_names.get(usize::try_from(i - 1).unwrap()) {
                                    proc_macro2::Ident::new(
                                        &format!("Role{}toAll", elt),
                                        ident.span(),
                                    )
                                } else {
                                    ident
                                }
                            }
                            28 => {
                                let role_names = extracted.2;

                                if let Some(elt) = role_names.get(usize::try_from(i - 1).unwrap()) {
                                    let temp = &format!("next_{}_to_all", elt);
                                    let mut result = String::from("");
                                    for c in temp.chars() {
                                        result.push_str(&c.to_lowercase().to_string());
                                    }
                                    proc_macro2::Ident::new(&result, ident.span())
                                } else {
                                    ident
                                }
                            }
                            29 => {
                                let role_names = extracted.2;

                                if let Some(elt) = role_names.get(usize::try_from(i - 1).unwrap()) {
                                    proc_macro2::Ident::new(
                                        &format!("RoleAllto{}", elt),
                                        ident.span(),
                                    )
                                } else {
                                    ident
                                }
                            }
                            30 => {
                                let role_names = extracted.2;

                                if let Some(elt) = role_names.get(usize::try_from(i - 1).unwrap()) {
                                    let temp = &format!("next_all_to_{}", elt);
                                    let mut result = String::from("");
                                    for c in temp.chars() {
                                        result.push_str(&c.to_lowercase().to_string());
                                    }
                                    proc_macro2::Ident::new(&result, ident.span())
                                } else {
                                    ident
                                }
                            }
                            31 => {
                                // ~9 with true
                                let (from, _, _, number_roles) = self.from_to_offset_exclusion();
                                let diff = number_roles - from;
                                let sum: u64 = (0..i).map(|j| number_roles - 1 - j).sum();

                                proc_macro2::Ident::new(
                                    &format!("{}{}", ident, diff * (diff + 1) / 2 + sum),
                                    ident.span(),
                                )
                            }
                            32 => {
                                // ~3 with true
                                let (line, column, _) = self.get_tuple_diag(diag_and_matrix.4, i);
                                proc_macro2::Ident::new(
                                    &format!("{}{}_{}", ident, line, column),
                                    ident.span(),
                                )
                            }
                            33 => {
                                // ~4 with true
                                let (line, column, _) = self.get_tuple_diag(diag_and_matrix.4, i);
                                proc_macro2::Ident::new(
                                    &format!("{}{}_{}", ident, column, line),
                                    ident.span(),
                                )
                            }
                            34 => {
                                // ~7 with true
                                let (_, _, _, number_roles) = self.from_to_offset_exclusion();
                                let (line, column, _) =
                                    self.get_tuple_matrix(diag_and_matrix.5, number_roles, i);
                                match line {
                                    m if m == i => proc_macro2::Ident::new(
                                        &format!("{}{}_{}", ident, column, line),
                                        ident.span(),
                                    ),
                                    _ => proc_macro2::Ident::new(
                                        &format!("{}{}_{}", ident, line, column),
                                        ident.span(),
                                    ),
                                }
                            }
                            35 => proc_macro2::Ident::new(
                                // ~5 with true and 2* instead of 3*
                                &format!("{}{}", ident, 2 * (i - 1) + 1),
                                ident.span(),
                            ),
                            36 => proc_macro2::Ident::new(
                                // ~6 with true and 2* instead of 3*
                                &format!("{}{}", ident, 2 * (i - 1) + 2),
                                ident.span(),
                            ),
                            37 => {
                                // ~8 with true
                                let (_, _, _, number_roles) = self.from_to_offset_exclusion();
                                let sum: u64 = (0..i).map(|j| number_roles - 1 - j).sum();

                                proc_macro2::Ident::new(&format!("{}{}", ident, sum), ident.span())
                            }
                            38 => {
                                // ~9 with true
                                let (from, _, _, number_roles) = self.from_to_offset_exclusion();
                                let diff = number_roles - from;
                                let sum: u64 = (0..i).map(|j| number_roles - 1 - j).sum();

                                proc_macro2::Ident::new(
                                    &format!("{}{}", ident, diff * (diff + 1) / 2 + sum),
                                    ident.span(),
                                )
                            }
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
                                let (line, column, _) =
                                    self.get_tuple_matrix(diag_and_matrix.1, i, j);
                                proc_macro2::Ident::new(
                                    &format!("{}{}_{}", ident, line, column),
                                    ident.span(),
                                )
                            }
                            1 => {
                                let (line, column, _) =
                                    self.get_tuple_matrix(diag_and_matrix.1, i, j);
                                proc_macro2::Ident::new(
                                    &format!("{}{}_{}", ident, column, line),
                                    ident.span(),
                                )
                            }
                            2 => {
                                let (_, _, m) = self.get_tuple_matrix(diag_and_matrix.1, i, j);
                                proc_macro2::Ident::new(&format!("{}{}", ident, m), ident.span())
                            }
                            3 => {
                                let (from, to, _, _) = self.from_to_offset_exclusion();
                                let (_, _, m) = self.get_tuple_matrix(diag_and_matrix.1, i, j);
                                let diff = to - from;

                                proc_macro2::Ident::new(
                                    &format!("{}{}", ident, diff * (diff + 1) / 2 + m),
                                    ident.span(),
                                )
                            }
                            4 => proc_macro2::Ident::new(
                                &format!("{}{}", ident, 3 * (i - 1) + j),
                                ident.span(),
                            ),
                            5 => {
                                let (line, column, _) =
                                    self.get_tuple_matrix(diag_and_matrix.1, i, j);
                                match line {
                                    m if m == i => proc_macro2::Ident::new(
                                        &format!("{}{}_{}", ident, line, column),
                                        ident.span(),
                                    ),
                                    _ => proc_macro2::Ident::new(
                                        &format!("{}{}_{}", ident, column, line),
                                        ident.span(),
                                    ),
                                }
                            }
                            6 => {
                                let (_, _, m) = self.get_tuple_matrix(diag_and_matrix.3, i, j);
                                proc_macro2::Ident::new(&format!("{}{}", ident, m), ident.span())
                            }
                            7 => {
                                let temp = if i >= self.exclusion.base10_parse::<u64>().unwrap() {
                                    i + 1
                                } else {
                                    i
                                };
                                if j < temp {
                                    proc_macro2::Ident::new(
                                        &format!("{}{}_{}", ident, temp, j),
                                        ident.span(),
                                    )
                                } else {
                                    proc_macro2::Ident::new(
                                        &format!("{}{}_{}", ident, temp, j + 1),
                                        ident.span(),
                                    )
                                }
                            }
                            8 => proc_macro2::Ident::new(&format!("{}{}", ident, j), ident.span()),
                            9 => {
                                let role_names = extracted.2;

                                if let Some(elt) = role_names.get(usize::try_from(j - 1).unwrap()) {
                                    proc_macro2::Ident::new(&format!("Role{}", elt), ident.span())
                                } else {
                                    ident
                                }
                            }
                            10 => {
                                let role_names = extracted.2;

                                if let Some(elt) = role_names.get(usize::try_from(j - 1).unwrap()) {
                                    let temp = &format!("next_{}", elt);
                                    let mut result = String::from("");
                                    for c in temp.chars() {
                                        result.push_str(&c.to_lowercase().to_string());
                                    }
                                    proc_macro2::Ident::new(&result, ident.span())
                                } else {
                                    ident
                                }
                            }
                            11 => {
                                let role_names = extracted.2;
                                if let Some(elt) = role_names.get(usize::try_from(j - 1).unwrap()) {
                                    proc_macro2::Ident::new(
                                        &format!("Role{}Dual", elt),
                                        ident.span(),
                                    )
                                } else {
                                    ident
                                }
                            }
                            12 => {
                                let role_names = extracted.2;
                                if let Some(elt) = role_names.get(usize::try_from(j - 1).unwrap()) {
                                    let temp = &format!("next_{}_dual", elt);
                                    let mut result = String::from("");
                                    for c in temp.chars() {
                                        result.push_str(&c.to_lowercase().to_string());
                                    }
                                    proc_macro2::Ident::new(&result, ident.span())
                                } else {
                                    ident
                                }
                            }
                            13 => {
                                let role_names = extracted.2;
                                if let Some(elt) = role_names.get(usize::try_from(j - 1).unwrap()) {
                                    proc_macro2::Ident::new(
                                        &format!("Role{}toAll", elt),
                                        ident.span(),
                                    )
                                } else {
                                    ident
                                }
                            }
                            14 => {
                                let role_names = extracted.2;
                                if let Some(elt) = role_names.get(usize::try_from(j - 1).unwrap()) {
                                    let temp = &format!("next_{}_to_all", elt);
                                    let mut result = String::from("");
                                    for c in temp.chars() {
                                        result.push_str(&c.to_lowercase().to_string());
                                    }
                                    proc_macro2::Ident::new(&result, ident.span())
                                } else {
                                    ident
                                }
                            }
                            15 => {
                                let role_names = extracted.2;
                                if let Some(elt) = role_names.get(usize::try_from(j - 1).unwrap()) {
                                    proc_macro2::Ident::new(
                                        &format!("RoleAllto{}", elt),
                                        ident.span(),
                                    )
                                } else {
                                    ident
                                }
                            }
                            16 => {
                                let role_names = extracted.2;
                                if let Some(elt) = role_names.get(usize::try_from(j - 1).unwrap()) {
                                    let temp = &format!("next_all_to_{}", elt);
                                    let mut result = String::from("");
                                    for c in temp.chars() {
                                        result.push_str(&c.to_lowercase().to_string());
                                    }
                                    proc_macro2::Ident::new(&result, ident.span())
                                } else {
                                    ident
                                }
                            }
                            17 => {
                                let index = if j >= i { j - 1 } else { j };

                                proc_macro2::Ident::new(
                                    &format!("{}{}", ident, index),
                                    ident.span(),
                                )
                            }
                            18 => {
                                // ~5 with true
                                let (line, column, _) =
                                    self.get_tuple_matrix(diag_and_matrix.5, i, j);
                                match line {
                                    m if m == i => proc_macro2::Ident::new(
                                        &format!("{}{}_{}", ident, line, column),
                                        ident.span(),
                                    ),
                                    _ => proc_macro2::Ident::new(
                                        &format!("{}{}_{}", ident, column, line),
                                        ident.span(),
                                    ),
                                }
                            }
                            19 => proc_macro2::Ident::new(
                                &format!("{}{}", ident, 2 * (i - 1) + j),
                                ident.span(),
                            ),
                            20 => {
                                // ~2 with true matrix
                                let (_, _, m) = self.get_tuple_matrix(diag_and_matrix.5, i, j);
                                proc_macro2::Ident::new(&format!("{}{}", ident, m), ident.span())
                            }
                            21 => {
                                let (from, _, _, number_roles) = self.from_to_offset_exclusion();
                                let (_, _, m) = self.get_tuple_matrix(diag_and_matrix.5, i, j);
                                let diff = number_roles - from;

                                proc_macro2::Ident::new(
                                    &format!("{}{}", ident, diff * (diff + 1) / 2 + m),
                                    ident.span(),
                                )
                            }
                            22 => {
                                if j < i {
                                    proc_macro2::Ident::new(
                                        &format!("{}{}", ident, j),
                                        ident.span(),
                                    )
                                } else {
                                    proc_macro2::Ident::new(
                                        &format!("{}{}", ident, j - 1),
                                        ident.span(),
                                    )
                                }
                            }
                            23 => {
                                if j > i {
                                    proc_macro2::Ident::new(
                                        &format!("{}{}", ident, 2 * (j - 1) - 1),
                                        ident.span(),
                                    )
                                } else {
                                    proc_macro2::Ident::new(
                                        &format!("{}{}", ident, 2 * (j - 1) + 1),
                                        ident.span(),
                                    )
                                }
                            }
                            24 => {
                                if j > i {
                                    proc_macro2::Ident::new(
                                        &format!("{}{}", ident, 2 * (j - 1)),
                                        ident.span(),
                                    )
                                } else {
                                    proc_macro2::Ident::new(
                                        &format!("{}{}", ident, 2 * (j - 1) + 2),
                                        ident.span(),
                                    )
                                }
                            }
                            25 => {
                                // ~(~8) with true and offset
                                let (_, _, m) = if j > i {
                                    self.get_tuple_matrix(diag_and_matrix.5, i, j - 1)
                                } else {
                                    self.get_tuple_matrix(diag_and_matrix.5, i, j)
                                };
                                proc_macro2::Ident::new(&format!("{}{}", ident, m), ident.span())
                            }
                            26 => {
                                // ~(~9) with true and offset
                                let (from, _, _, number_roles) = self.from_to_offset_exclusion();
                                let (_, _, m) = if j > i {
                                    self.get_tuple_matrix(diag_and_matrix.5, i, j - 1)
                                } else {
                                    self.get_tuple_matrix(diag_and_matrix.5, i, j)
                                };
                                let diff = number_roles - from;

                                proc_macro2::Ident::new(
                                    &format!("{}{}", ident, diff * (diff + 1) / 2 + m),
                                    ident.span(),
                                )
                            }
                            27 => {
                                let (line, column) =
                                    self.get_line_column_from_diag(diag_and_matrix.4, j);
                                proc_macro2::Ident::new(
                                    &format!("{}{}_{}", ident, line, column),
                                    ident.span(),
                                )
                            }
                            28 => {
                                let (line, column) =
                                    self.get_line_column_from_diag(diag_and_matrix.4, j);
                                proc_macro2::Ident::new(
                                    &format!("{}{}_{}", ident, column, line),
                                    ident.span(),
                                )
                            }
                            29 => proc_macro2::Ident::new(&format!("{}{}", ident, j), ident.span()),
                            30 => {
                                let (from, _, _, number_roles) = self.from_to_offset_exclusion();
                                let diff = number_roles - from;
                                proc_macro2::Ident::new(
                                    &format!("{}{}", ident, diff * (diff + 1) / 2 + j),
                                    ident.span(),
                                )
                            }
                            31 => proc_macro2::Ident::new(
                                &format!("{}{}_{}", ident, i, j),
                                ident.span(),
                            ),
                            _ => ident,
                        };

                        *rest = peek.clone();
                        *mutated = true;

                        // we may need to also consume another ~
                        match peek.next() {
                            Some(proc_macro2::TokenTree::Punct(ref punct))
                                if punct.as_char() == '~' =>
                            {
                                *rest = peek;
                            }
                            _ => {}
                        }
                    }

                    (
                        Mode::ReplaceIdent(_i),
                        Mode::ReplaceIdent(j),
                        Mode::ReplaceIdent(k),
                        Some(proc_macro2::TokenTree::Punct(ref punct)),
                        Some(proc_macro2::TokenTree::Ident(ref ident2)),
                        Some(proc_macro2::TokenTree::Punct(ref sign)),
                        Some(proc_macro2::TokenTree::Literal(ref select_mode)),
                    ) if punct.as_char() == '|'
                        && ident2 == &self.ident
                        && sign.as_char() == ':' =>
                    {
                        // have seen ident | N : n
                        ident = match self.unwrap_literal(select_mode) {
                            0 => proc_macro2::Ident::new(&format!("{}{}", ident, k), ident.span()),
                            1 => {
                                // ~2 with true matrix
                                let (_, _, m) = self.get_tuple_matrix(diag_and_matrix.5, j, k);
                                proc_macro2::Ident::new(&format!("{}{}", ident, m), ident.span())
                            }
                            2 => {
                                // ~3 with true matrix
                                let (from, _, _, number_roles) = self.from_to_offset_exclusion();
                                let (_, _, m) = self.get_tuple_matrix(diag_and_matrix.5, j, k);
                                let diff = number_roles - from;

                                proc_macro2::Ident::new(
                                    &format!("{}{}", ident, diff * (diff + 1) / 2 + m),
                                    ident.span(),
                                )
                            }
                            3 => {
                                if j > k {
                                    proc_macro2::Ident::new(
                                        &format!("{}{}_{}", ident, j, k),
                                        ident.span(),
                                    )
                                } else {
                                    proc_macro2::Ident::new(
                                        &format!("{}{}_{}", ident, j, k + 1),
                                        ident.span(),
                                    )
                                }
                            }
                            _ => ident,
                        };

                        *rest = peek.clone();
                        *mutated = true;

                        // we may need to also consume another ~
                        match peek.next() {
                            Some(proc_macro2::TokenTree::Punct(ref punct))
                                if punct.as_char() == '|' =>
                            {
                                *rest = peek;
                            }
                            _ => {}
                        }
                    }

                    // available signs
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
                                    .range_0(0, self.unwrap_literal(extra), 0, false)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            1 => {
                                return self
                                    .range_1(0, self.unwrap_literal(extra), false)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            2 => {
                                return self
                                    .range_2(0, self.unwrap_literal(extra), false)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            3 => {
                                return self
                                    .range_3(0, self.unwrap_literal(extra), false)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            4 => {
                                return self
                                    .range_12(1, self.unwrap_literal(extra), false)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            5 => {
                                return self
                                    .range_5(0, self.unwrap_literal(extra), false)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
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
                                            extracted,
                                            diag_and_matrix,
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
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            8 => {
                                let (from, to, _, _) = self.from_to_offset_exclusion();
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
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            9 => {
                                return self
                                    .range_7(0, self.unwrap_literal(extra), false)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            10 => {
                                return self
                                    .range_4(1, self.unwrap_literal(extra), false)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            11 => {
                                return self
                                    .range_7(1, self.unwrap_literal(extra), false)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            12 => {
                                return self
                                    .range_9(0, self.unwrap_literal(extra), false)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            13 => {
                                return self
                                    .range_10(0, self.unwrap_literal(extra), false)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            14 => {
                                return self
                                    .range_14(1, self.unwrap_literal(extra), false)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            15 => {
                                return self
                                    .range_16(0, self.unwrap_literal(extra), 0, false)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            16 => {
                                return self
                                    .range_17(0, 2, 0, false)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            17 => {
                                return self
                                    .range_18(1, self.unwrap_literal(extra), 1, false)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            18 => {
                                return self
                                    .range_19(0, self.unwrap_literal(extra), 0, 1, false)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            19 => {
                                return self
                                    .range_20(0, self.unwrap_literal(extra), 0, 1, false)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            20 => {
                                // ~0 with true
                                return self
                                    .range_0(0, self.unwrap_literal(extra), 0, true)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            21 => {
                                // ~0 with true + 1
                                return self
                                    .range_0(0, self.unwrap_literal(extra), 1, true)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            22 => {
                                // ~2 with true
                                return self
                                    .range_2(0, self.unwrap_literal(extra), true)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            23 => {
                                // ~3 with true
                                return self
                                    .range_3(0, self.unwrap_literal(extra), true)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            24 => {
                                // ~4 with true
                                return self
                                    .range_12(1, self.unwrap_literal(extra), true)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            25 => {
                                // ~10 with true
                                return self
                                    .range_4(1, self.unwrap_literal(extra), true)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            26 => {
                                // ~11 with true and to 2* instead of 3*
                                return self
                                    .range_23(1, self.unwrap_literal(extra), true)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            27 => {
                                // ~13 with true and 2* instead of 3*
                                return self
                                    .range_24(0, self.unwrap_literal(extra), true)
                                    .map(|i| {
                                        self.expand_pass(
                                            rep.stream(),
                                            (Mode::ReplaceIdent(i), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
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
                                    .range_0(0, 0, 0, false)
                                    .filter_map(|i| {
                                        if i == self.exclusion.base10_parse::<u64>().unwrap()
                                            && self.objection
                                        {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (Mode::ReplaceIdent(i), modes.1, modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (Mode::ReplaceIdent(i), modes.1, modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        }
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            1 => {
                                return self
                                    .range_0(0, 0, 1, false)
                                    .map(|j| {
                                        self.expand_pass(
                                            rep_if.stream(),
                                            (Mode::ReplaceIdent(j), modes.1, modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            2 => {
                                return self
                                    .range_0(0, 0, 0, false)
                                    .filter_map(|i| {
                                        if i == 1 && self.objection {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (Mode::ReplaceIdent(i), modes.1, modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (Mode::ReplaceIdent(i), modes.1, modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        }
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            3 => {
                                return self
                                    .range_17(0, 2, 0, false)
                                    .filter_map(|i| {
                                        if i == self.exclusion.base10_parse::<u64>().unwrap()
                                            && self.objection
                                        {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (Mode::ReplaceIdent(i), modes.1, modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (Mode::ReplaceIdent(i), modes.1, modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        }
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            4 => {
                                return self
                                    .range_20(0, 0, 0, 1, false)
                                    .filter_map(|i| {
                                        if i == 2 && self.objection {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (Mode::ReplaceIdent(i), modes.1, modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (Mode::ReplaceIdent(i), modes.1, modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        }
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            5 => {
                                return self
                                    .range_12(1, 0, false)
                                    .filter_map(|i| {
                                        if i < (self.to).base10_parse::<u64>().unwrap()
                                            && self.objection
                                        {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (Mode::ReplaceIdent(i), modes.1, modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (Mode::ReplaceIdent(i), modes.1, modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        }
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            6 => {
                                return self
                                    .one()
                                    .filter_map(|i| {
                                        if i == 1 && self.objection {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (Mode::ReplaceIdent(i), modes.1, modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (Mode::ReplaceIdent(i), modes.1, modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        }
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
                                    .range_0(0, 0, 0, false)
                                    .filter_map(|j| {
                                        let (k, _, _) =
                                            self.get_tuple_matrix(diag_and_matrix.1, i, j);

                                        if k != i {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        }
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            1 => {
                                return self
                                    .range_8(0, 0, false)
                                    .filter_map(|j| {
                                        let (k, _, _) =
                                            self.get_tuple_matrix(diag_and_matrix.1, i, j);

                                        if k != i {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        }
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            2 => {
                                return self
                                    .range_7(1, 0, false)
                                    .map(|j| {
                                        self.expand_pass(
                                            rep_if.stream(),
                                            (modes.0, Mode::ReplaceIdent(j), modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            3 => {
                                return self
                                    .range_11(0, 0)
                                    .map(|j| {
                                        self.expand_pass(
                                            rep_if.stream(),
                                            (modes.0, Mode::ReplaceIdent(j), modes.2),
                                            extracted,
                                            diag_and_matrix,
                                        )
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            4 => {
                                return self
                                    .range_13(0, 0, 0, false)
                                    .filter_map(|j| {
                                        let (k, _, _) =
                                            self.get_tuple_matrix(diag_and_matrix.3, i, j);

                                        if k != i {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        }
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            5 => {
                                return self
                                    .range_0(0, 0, 0, false)
                                    .filter_map(|j| {
                                        if i != j {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        }
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            6 => {
                                return self
                                    .range_0(0, 0, 0, true)
                                    .filter_map(|j| {
                                        if i != j {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        }
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            7 => {
                                return self
                                    .range_0(0, 0, 1, true)
                                    .filter_map(|j| {
                                        if i != j {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        }
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            8 => {
                                // ~(~()()0*) with true
                                return self
                                    .range_0(0, 0, 0, true)
                                    .filter_map(|j| {
                                        let (k, _, _) =
                                            self.get_tuple_matrix(diag_and_matrix.5, i, j);

                                        if k != i {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        }
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            9 => {
                                // ~(#4) with true
                                return self
                                    .range_12(1, 0, true)
                                    .filter_map(|j| {
                                        let (line, _column) =
                                            self.get_line_column_from_diag(diag_and_matrix.4, j);
                                        if i == line {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        }
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            10 => {
                                return self
                                    .range_0(0, 0, 1, true)
                                    .filter_map(|j| {
                                        if i > j {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        } else if i < j {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (modes.0, Mode::ReplaceIdent(j), modes.2),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        } else {
                                            std::option::Option::None
                                        }
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

            proc_macro2::TokenTree::Punct(ref p) if p.as_char() == '|' => {
                // is this |(...)(...)n* ?
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
                        Mode::ReplaceIdent(j),
                        _,
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
                                    .range_0(0, 0, 0, true)
                                    .filter_map(|k| {
                                        let cond = if k >= i { j - 1 } else { j };
                                        if k == cond {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (modes.0, modes.1, Mode::ReplaceIdent(k)),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (modes.0, modes.1, Mode::ReplaceIdent(k)),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        }
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            1 => {
                                return self
                                    .range_0(0, 0, 1, true)
                                    .filter_map(|k| {
                                        let cond = if k >= i { j - 1 } else { j };
                                        if k == cond {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (modes.0, modes.1, Mode::ReplaceIdent(k)),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (modes.0, modes.1, Mode::ReplaceIdent(k)),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        }
                                    })
                                    .map(|(ts, _)| ts)
                                    .collect();
                            }
                            2 => {
                                // ~(~()()0*) with true
                                return self
                                    .range_0(0, 0, 0, true)
                                    .filter_map(|k| {
                                        let (l, _, _) =
                                            self.get_tuple_matrix(diag_and_matrix.5, j, k);

                                        let (_, _, m1) = if j > i {
                                            self.get_tuple_matrix(diag_and_matrix.5, i, j - 1)
                                        } else {
                                            self.get_tuple_matrix(diag_and_matrix.5, i, j)
                                        };
                                        let (_, _, m2) =
                                            self.get_tuple_matrix(diag_and_matrix.5, j, k);

                                        if l == j || m1 == m2 {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_if.stream(),
                                                (modes.0, modes.1, Mode::ReplaceIdent(k)),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        } else {
                                            std::option::Option::Some(self.expand_pass(
                                                rep_else.stream(),
                                                (modes.0, modes.1, Mode::ReplaceIdent(k)),
                                                extracted,
                                                diag_and_matrix,
                                            ))
                                        }
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

        if extraction {
            new_ident
        } else {
            std::iter::once(tt).collect()
        }
    }

    fn expand_pass(
        &self,
        stream: proc_macro2::TokenStream,
        modes: (Mode, Mode, Mode),
        extracted: (
            &Vec<proc_macro2::TokenStream>, // labels
            &Vec<proc_macro2::TokenStream>, // receivers
            &Vec<proc_macro2::TokenStream>, // role_name
        ),
        diag_and_matrix: (
            &Diag,   // diag,
            &Matrix, // matrix,
            &Diag,   // diag_w_offset,
            &Matrix, // matrix_w_offset,
            &Diag,   // diag with true,
            &Matrix, // matrix with true,
            &Diag,   // diag_w_offset with true,
            &Matrix, // matrix_w_offset with true,
        ),
    ) -> (proc_macro2::TokenStream, bool) {
        let mut out = proc_macro2::TokenStream::new();
        let mut mutated = false;
        let mut tts = stream.into_iter();
        while let Some(tt) = tts.next() {
            out.extend(self.expand2(
                tt,
                &mut tts,
                &mut mutated,
                modes,
                extracted,
                diag_and_matrix,
            ));
        }
        (out, mutated)
    }

    fn expand_pass_options(
        &self,
        stream: proc_macro2::TokenStream,
    ) -> Vec<proc_macro2::TokenStream> {
        let mut out: Vec<proc_macro2::TokenStream> = Vec::new();
        let mut tts = stream.into_iter();
        while let Some(tt) = tts.next() {
            let elt = self.expand_options(tt, &mut tts);
            if let Some(elt_tt) = elt {
                out.push(elt_tt)
            }
        }
        out
    }

    fn expand(
        &self,
        stream: proc_macro2::TokenStream,
        labels: proc_macro2::TokenStream,
        receivers: proc_macro2::TokenStream,
        role_name: proc_macro2::TokenStream,
        diag_and_matrix: (
            &Diag,   // diag,
            &Matrix, // matrix,
            &Diag,   // diag_w_offset,
            &Matrix, // matrix_w_offset,
            &Diag,   // diag with true,
            &Matrix, // matrix with true,
            &Diag,   // diag_w_offset with true,
            &Matrix, // matrix_w_offset with true,
        ),
    ) -> proc_macro2::TokenStream {
        let extracted_labels = self.expand_pass_options(labels);
        let extracted_receivers = self.expand_pass_options(receivers);
        let extracted_role_name = self.expand_pass_options(role_name);

        let (out, mutated) = self.expand_pass(
            stream.clone(),
            (
                Mode::ReplaceSequence,
                Mode::ReplaceSequence,
                Mode::ReplaceSequence,
            ),
            (
                &extracted_labels,
                &extracted_receivers,
                &extracted_role_name,
            ),
            diag_and_matrix,
        );
        if mutated {
            return out;
        }

        self.range_0(0, 0, 0, false)
            .map(|i| {
                self.expand_pass(
                    stream.clone(),
                    (
                        Mode::ReplaceIdent(i),
                        Mode::ReplaceSequence,
                        Mode::ReplaceSequence,
                    ),
                    (
                        &extracted_labels,
                        &extracted_receivers,
                        &extracted_role_name,
                    ),
                    diag_and_matrix,
                )
            })
            .map(|(ts, _)| ts)
            .collect()
    }
}

//////////////////////////////////////

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SeqMacroInput);
    let output: proc_macro2::TokenStream = input.into();
    output.into()
}

#[proc_macro_hack]
pub fn e_seq(input: TokenStream) -> TokenStream {
    seq(input)
}

#[proc_macro]
pub fn recv_aux_simple(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as RecvAuxSimpleMacroInput);
    let output: proc_macro2::TokenStream = input.into();
    output.into()
}

#[proc_macro_hack]
pub fn e_recv_aux_simple(input: TokenStream) -> TokenStream {
    recv_aux_simple(input)
}
