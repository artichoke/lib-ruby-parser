#[cfg(feature = "onig")]
use onig::{Regex, RegexOptions};

use bumpalo::Bump;

use std::collections::HashMap;
use std::convert::TryInto;

crate::use_native_or_external!(String);
crate::use_native_or_external!(Vec);
crate::use_native_or_external!(Maybe);

use crate::error::Diagnostics;
use crate::nodes::internal;
#[allow(unused_imports)]
use crate::nodes::*;
use crate::LexState;
use crate::Loc;
use crate::{
    Bytes, Context, CurrentArgStack, /*Lexer,*/ MaxNumparamStack, Node, StaticEnvironment,
    Token, VariablesStack,
};
use crate::{Diagnostic, DiagnosticMessage, ErrorLevel};

#[derive(Debug, PartialEq)]
pub(crate) enum LoopType {
    While,
    Until,
}

#[derive(Debug, PartialEq)]
pub(crate) enum KeywordCmd {
    Break,
    Defined,
    Next,
    Redo,
    Retry,
    Return,
    Super,
    Yield,
    Zsuper,
}

enum MethodCallType {
    Send,
    CSend,
}

#[derive(Debug, PartialEq)]
pub(crate) enum LogicalOp {
    And,
    Or,
}

#[derive(Debug)]
pub(crate) enum PKwLabel<'a> {
    PlainLabel(&'a Token<'a>),
    QuotedLabel((&'a Token<'a>, Vec<'a, &'a Node<'a>>, &'a Token<'a>)),
}

#[derive(Debug)]
pub(crate) enum ArgsType<'a> {
    Args(Maybe<&'a Node<'a>>),
    Numargs(u8),
}

#[derive(Debug)]
pub(crate) struct Builder<'a> {
    bump: &'a Bump,
    static_env: StaticEnvironment,
    context: Context,
    current_arg_stack: CurrentArgStack,
    max_numparam_stack: MaxNumparamStack,
    pattern_variables: VariablesStack,
    pattern_hash_keys: VariablesStack,
    diagnostics: Diagnostics<'a>,
}

#[allow(mutable_transmutes)]
impl<'a> Builder<'a> {
    pub(crate) fn new(
        bump: &'a Bump,
        static_env: StaticEnvironment,
        context: Context,
        current_arg_stack: CurrentArgStack,
        max_numparam_stack: MaxNumparamStack,
        pattern_variables: VariablesStack,
        pattern_hash_keys: VariablesStack,
        diagnostics: Diagnostics<'a>,
    ) -> Self {
        Self {
            static_env,
            context,
            current_arg_stack,
            max_numparam_stack,
            pattern_variables,
            pattern_hash_keys,
            diagnostics,
            bump,
        }
    }

    //
    // Literals
    //

    // Singletons

    pub(crate) fn nil(&self, nil_t: &'a Token<'a>) -> &'a Node<'a> {
        Node::new_nil(self.bump, loc(nil_t))
    }

    pub(crate) fn true_(&self, true_t: &'a Token<'a>) -> &'a Node<'a> {
        Node::new_true(self.bump, loc(true_t))
    }

    pub(crate) fn false_(&self, false_t: &'a Token<'a>) -> &'a Node<'a> {
        Node::new_false(self.bump, loc(false_t))
    }

    // Numerics

    pub(crate) fn integer(&self, integer_t: &'a Token<'a>) -> &'a Node<'a> {
        let expression_l = loc(integer_t);
        Node::new_int(self.bump, value(integer_t), None, expression_l)
    }

    pub(crate) fn float(&self, float_t: &'a Token<'a>) -> &'a Node<'a> {
        let expression_l = loc(float_t);
        Node::new_float(self.bump, value(float_t), None, expression_l)
    }

    pub(crate) fn rational(&self, rational_t: &'a Token<'a>) -> &'a Node<'a> {
        let expression_l = loc(rational_t);
        Node::new_rational(self.bump, value(rational_t), None, expression_l)
    }

    pub(crate) fn complex(&self, complex_t: &'a Token<'a>) -> &'a Node<'a> {
        let expression_l = loc(complex_t);
        Node::new_complex(self.bump, value(complex_t), None, expression_l)
    }

    pub(crate) fn unary_num(&self, unary_t: &'a Token<'a>, numeric: &'a Node<'a>) -> &'a Node<'a> {
        let new_operator_l = loc(unary_t);
        let sign = String::from(value(unary_t));

        let numeric: &'a mut Node<'a> = unsafe { std::mem::transmute(numeric) };
        match numeric {
            Node::Int(int) => {
                let new_value = String::from_str_in(&(sign + int.get_value()), self.bump);
                int.set_value(new_value);

                let new_expression_l = new_operator_l.join(int.get_expression_l());
                int.set_expression_l(new_expression_l);

                int.set_operator_l(Some(new_operator_l));
            }
            Node::Float(float) => {
                let new_value = String::from_str_in(&(sign + float.get_value()), self.bump);
                float.set_value(new_value);

                let new_expression_l = new_operator_l.join(float.get_expression_l());
                float.set_expression_l(new_expression_l);

                float.set_operator_l(Some(new_operator_l));
            }
            Node::Rational(rational) => {
                let new_value = String::from_str_in(&(sign + rational.get_value()), self.bump);
                rational.set_value(new_value);

                let new_expression_l = new_operator_l.join(rational.get_expression_l());
                rational.set_expression_l(new_expression_l);

                rational.set_operator_l(Some(new_operator_l));
            }
            Node::Complex(complex) => {
                let new_value = String::from_str_in(&(sign + complex.get_value()), self.bump);
                complex.set_value(new_value);

                let new_expression_l = new_operator_l.join(complex.get_expression_l());
                complex.set_expression_l(new_expression_l);

                complex.set_operator_l(Some(new_operator_l));
            }
            _ => {
                unreachable!()
            }
        }

        numeric
    }

    pub(crate) fn __line__(&self, line_t: &'a Token<'a>) -> &'a Node<'a> {
        Node::new_line(self.bump, loc(line_t))
    }

    // Strings

    pub(crate) fn str_node(
        &self,
        begin_t: Maybe<&'a Token<'a>>,
        value: Bytes<'a>,
        parts: Vec<'a, &'a Node<'a>>,
        end_t: Maybe<&'a Token<'a>>,
    ) -> &'a Node<'a> {
        if self.is_heredoc(&begin_t) {
            let HeredocMap {
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            } = self.heredoc_map(&begin_t, parts.as_slice(), &end_t);

            Node::new_heredoc(
                self.bump,
                parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            )
        } else {
            let CollectionMap {
                begin_l,
                end_l,
                expression_l,
            } = collection_map(&begin_t, &parts, &end_t);

            Node::new_str(self.bump, value, begin_l, end_l, expression_l)
        }
    }

    pub(crate) fn string_internal(&self, string_t: &'a Token<'a>) -> &'a Node<'a> {
        let expression_l = loc(string_t);
        let value = string_t.token_value.take();
        Node::new_str(self.bump, value, None, None, expression_l)
    }

    pub(crate) fn string_compose(
        &self,
        begin_t: Maybe<&'a Token<'a>>,
        parts: Vec<'a, &'a Node<'a>>,
        end_t: Maybe<&'a Token<'a>>,
    ) -> &'a Node<'a> {
        if parts.is_empty() {
            return self.str_node(begin_t, Bytes::empty(self.bump), parts, end_t);
        } else if parts.len() == 1 {
            if (parts[0].is_str() || parts[0].is_dstr() || parts[0].is_heredoc())
                && begin_t.is_none()
                && end_t.is_none()
            {
                return parts
                    .into_iter()
                    .next()
                    .expect("expected at least 1 element");
            }

            match parts.first().unwrap() {
                Node::Str(str) => {
                    let str_value = { str.get_value().clone() };
                    let part = parts
                        .into_iter()
                        .next()
                        .expect("expected at least 1 element");
                    return self.str_node(begin_t, str_value, bump_vec![in self.bump; part], end_t);
                }
                _ => {}
            }

            // if let Some(str) = part.as_str() {
            //     let value = str.get_value().clone();
            //     return self.str_node(begin_t, value, bump_vec![in self.bump; part], end_t);
            // }

            if parts[0].is_dstr() || parts[0].is_heredoc() {
                unreachable!()
            }
        }

        if self.is_heredoc(&begin_t) {
            let HeredocMap {
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            } = self.heredoc_map(&begin_t, &parts, &end_t);

            Node::new_heredoc(
                self.bump,
                parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            )
        } else {
            let CollectionMap {
                begin_l,
                end_l,
                expression_l,
            } = collection_map(&begin_t, &parts, &end_t);

            Node::new_dstr(self.bump, parts, begin_l, end_l, expression_l)
        }
    }

    pub(crate) fn character(&self, char_t: &'a Token<'a>) -> &'a Node<'a> {
        let str_loc = loc(char_t);

        let begin_l = Some(str_loc.with_end(str_loc.begin() + 1));
        let end_l = None;
        let expression_l = str_loc;

        let value = char_t.token_value.take();
        Node::new_str(self.bump, value, begin_l, end_l, expression_l)
    }

    pub(crate) fn __file__(&self, file_t: &'a Token<'a>) -> &'a Node<'a> {
        Node::new_file(self.bump, loc(file_t))
    }

    // Symbols

    fn validate_sym_value(&self, value: &Bytes, loc: &Loc) {
        if !value.is_valid_utf8() {
            self.error(
                DiagnosticMessage::new_invalid_symbol(String::from_str_in("UTF-8", self.bump)),
                loc,
            )
        }
    }

    pub(crate) fn symbol(&self, start_t: &'a Token<'a>, value_t: &'a Token<'a>) -> &'a Node<'a> {
        let expression_l = loc(start_t).join(&loc(value_t));
        let begin_l = Some(loc(start_t));
        let value = value_t.token_value.take();
        self.validate_sym_value(&value, &expression_l);
        Node::new_sym(self.bump, value, begin_l, None, expression_l)
    }

    pub(crate) fn symbol_internal(&self, symbol_t: &'a Token<'a>) -> &'a Node<'a> {
        let expression_l = loc(symbol_t);
        let value = symbol_t.token_value.take();
        self.validate_sym_value(&value, &expression_l);
        Node::new_sym(self.bump, value, None, None, expression_l)
    }

    pub(crate) fn symbol_compose(
        &self,
        begin_t: &'a Token<'a>,
        mut parts: Vec<'a, &'a Node<'a>>,
        end_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        if parts.len() == 1 {
            match parts.first_mut().unwrap() {
                Node::Str(str) => {
                    let value = str.value.take();

                    let CollectionMap {
                        begin_l,
                        end_l,
                        expression_l,
                    } = collection_map(&Some(begin_t), &[], &Some(end_t));

                    self.validate_sym_value(&value, &expression_l);

                    return Node::new_sym(self.bump, value, begin_l, end_l, expression_l);
                }
                _ => {}
            }
        }

        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = collection_map(&Some(begin_t), &parts, &Some(end_t));
        Node::new_dsym(self.bump, parts, begin_l, end_l, expression_l)
    }

    // Executable strings

    pub(crate) fn xstring_compose(
        &self,
        begin_t: &'a Token<'a>,
        parts: Vec<'a, &'a Node<'a>>,
        end_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let begin_l = loc(begin_t);
        if lossy_value(begin_t).as_str().starts_with("<<") {
            let heredoc_body_l = collection_expr(&parts).unwrap_or_else(|| loc(end_t));
            let heredoc_end_l = loc(end_t);
            let expression_l = begin_l;

            Node::new_x_heredoc(
                self.bump,
                parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            )
        } else {
            let end_l = loc(end_t);
            let expression_l = begin_l.join(&end_l);

            Node::new_xstr(self.bump, parts, begin_l, end_l, expression_l)
        }
    }

    // Indented (interpolated, noninterpolated, executable) strings

    pub(crate) fn heredoc_dedent(&self, node: &'a Node<'a>, dedent_level: i32) {
        if dedent_level == 0 {
            return;
        }

        let dedent_level: usize = dedent_level
            .try_into()
            .expect("dedent_level must be positive");

        fn dedent_string<'a>(bump: &'a Bump, s: Bytes<'a>, width: usize) -> Bytes<'a> {
            const TAB_WIDTH: usize = 8;
            let mut col: usize = 0;
            let mut i: usize = 0;

            loop {
                if !(i < s.len() && col < width) {
                    break;
                }

                if s[i] == b' ' {
                    col += 1;
                } else if s[i] == b'\t' {
                    let n = TAB_WIDTH * (col / TAB_WIDTH + 1);
                    if n > TAB_WIDTH {
                        break;
                    }
                    col = n;
                } else {
                    break;
                }

                i += 1;
            }

            Bytes::new(
                bump,
                Vec::from_iter_in(s.as_raw()[i..].iter().cloned(), bump),
            )
        }

        fn dedent_heredoc_parts<'a>(
            bump: &'a Bump,
            parts: &mut Vec<'a, &'a Node<'a>>,
            dedent_level: usize,
        ) {
            let mut idx_to_drop = bump_vec![in bump;];

            for (idx, part) in parts.iter_mut().enumerate() {
                match part {
                    Node::Str(Str {
                        value,
                        begin_l,
                        end_l,
                        expression_l,
                        ..
                    }) => {
                        let value = dedent_string(bump, value.take(), dedent_level);
                        if value.is_empty() {
                            idx_to_drop.push(idx)
                        } else {
                            *part = Node::new_str(
                                bump,
                                value,
                                begin_l.clone(),
                                end_l.clone(),
                                expression_l.clone(),
                            )
                        }
                    }
                    Node::Begin(_)
                    | Node::Gvar(_)
                    | Node::BackRef(_)
                    | Node::NthRef(_)
                    | Node::Ivar(_)
                    | Node::Cvar(_) => { /* skip */ }
                    other => {
                        unreachable!("unsupported heredoc child")
                    }
                }
            }
            idx_to_drop.reverse();
            for idx in idx_to_drop.into_iter() {
                parts.remove(idx);
            }
        };

        match node {
            Node::Heredoc(Heredoc { parts, .. }) | Node::XHeredoc(XHeredoc { parts, .. }) => {
                let parts: &mut Vec<&'a Node> = unsafe { std::mem::transmute(parts) };
                dedent_heredoc_parts(self.bump, parts, dedent_level);
            }
            _ => {
                unreachable!("unsupported heredoc_dedent argument {}", node.str_type())
            }
        }
    }

    // Regular expressions

    pub(crate) fn regexp_options(&self, regexp_end_t: &'a Token<'a>) -> Maybe<&'a Node<'a>> {
        if regexp_end_t.loc().end() - regexp_end_t.loc().begin() == 1 {
            // no regexp options, only trailing "/"
            return Maybe::none();
        }
        let expression_l = loc(regexp_end_t).adjust_begin(1);
        let options = value(regexp_end_t);
        let mut options = options
            .as_str()
            .chars()
            .skip(1)
            .collect::<std::vec::Vec<_>>();
        options.sort_unstable();
        options.dedup();
        let options = if options.is_empty() {
            Maybe::none()
        } else {
            Some(String::from_iter_in(options.into_iter(), self.bump))
        };

        Some(Node::new_reg_opt(self.bump, options, expression_l))
    }

    pub(crate) fn regexp_compose(
        &self,
        begin_t: &'a Token<'a>,
        parts: Vec<'a, &'a Node<'a>>,
        end_t: &'a Token<'a>,
        options: Maybe<&'a Node<'a>>,
    ) -> &'a Node<'a> {
        let begin_l = loc(begin_t);
        let end_l = loc(end_t).resize(1);
        let expression_l =
            begin_l.join(&maybe_node_expr_mut(&options).unwrap_or_else(|| loc(end_t)));

        match &options {
            Some(Node::RegOpt(RegOpt { options, .. })) => {
                self.validate_static_regexp(&parts, options, &expression_l)
            }
            None => self.validate_static_regexp(&parts, &Maybe::none(), &expression_l),
            _ => unreachable!("must be Option<RegOpt>"),
        }
        // if options.is_some() && options.as_ref().unwrap().is_reg_opt() {
        //     let options = options
        //         .as_ref()
        //         .unwrap()
        //         .as_reg_opt()
        //         .unwrap()
        //         .get_options();
        //     self.validate_static_regexp(&parts, options, &expression_l)
        // } else if options.is_none() {
        //     self.validate_static_regexp(&parts, &Maybe::none(), &expression_l)
        // } else {
        //     unreachable!("must be Option<RegOpt>")
        // }

        Node::new_regexp(self.bump, parts, options, begin_l, end_l, expression_l)
    }

    // Arrays

    pub(crate) fn array(
        &self,
        begin_t: Maybe<&'a Token<'a>>,
        elements: Vec<'a, &'a Node<'a>>,
        end_t: Maybe<&'a Token<'a>>,
    ) -> &'a Node<'a> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = collection_map(&begin_t, &elements, &end_t);

        Node::new_array(self.bump, elements, begin_l, end_l, expression_l)
    }

    pub(crate) fn splat(&self, star_t: &'a Token<'a>, value: Maybe<&'a Node<'a>>) -> &'a Node<'a> {
        let operator_l = loc(star_t);
        let expression_l = operator_l.maybe_join(&maybe_node_expr_mut(&value));

        Node::new_splat(self.bump, value, operator_l, expression_l)
    }

    pub(crate) fn word(&self, parts: Vec<'a, &'a Node<'a>>) -> &'a Node<'a> {
        if parts.len() == 1 && (parts[0].is_str() || parts[0].is_dstr()) {
            let part = parts
                .into_iter()
                .next()
                .expect("parts is supposed to have exactly 1 element");
            return part;
        }

        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = collection_map(&Maybe::none(), &parts, &Maybe::none());

        Node::new_dstr(self.bump, parts, begin_l, end_l, expression_l)
    }

    pub(crate) fn words_compose(
        &self,
        begin_t: &'a Token<'a>,
        elements: Vec<'a, &'a Node<'a>>,
        end_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let begin_l = loc(begin_t);
        let end_l = loc(end_t);
        let expression_l = begin_l.join(&end_l);
        Node::new_array(
            self.bump,
            elements,
            Some(begin_l),
            Some(end_l),
            expression_l,
        )
    }

    pub(crate) fn symbols_compose(
        &self,
        begin_t: &'a Token<'a>,
        parts: Vec<'a, &'a Node<'a>>,
        end_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let parts = parts.into_iter().map(|part| match part {
            Node::Str(Str {
                value,
                begin_l,
                end_l,
                expression_l,
            }) => {
                self.validate_sym_value(&value, &expression_l);
                Node::new_sym(
                    self.bump,
                    value.take(),
                    begin_l.clone(),
                    end_l.clone(),
                    expression_l.clone(),
                )
            }
            Node::Dstr(Dstr {
                parts,
                begin_l,
                end_l,
                expression_l,
            }) => Node::new_dsym(
                self.bump,
                take_vec(parts),
                begin_l.clone(),
                end_l.clone(),
                expression_l.clone(),
            ),
            other => other,
        });
        let parts = Vec::from_iter_in(parts, self.bump);

        let begin_l = loc(begin_t);
        let end_l = loc(end_t);
        let expression_l = begin_l.join(&end_l);
        Node::new_array(
            self.bump,
            Vec::from(parts),
            Some(begin_l),
            Some(end_l),
            expression_l,
        )
    }

    // Hashes

    pub(crate) fn pair(
        &self,
        key: &'a Node<'a>,
        assoc_t: &'a Token<'a>,
        value: &'a Node<'a>,
    ) -> &'a Node<'a> {
        let operator_l = loc(assoc_t);
        let expression_l = join_exprs(key, value);

        Node::new_pair(self.bump, key, value, operator_l, expression_l)
    }

    pub(crate) fn pair_keyword(&self, key_t: &'a Token<'a>, value: &'a Node<'a>) -> &'a Node<'a> {
        let key_loc = loc(key_t);
        let key_l = key_loc.adjust_end(-1);
        let colon_l = key_loc.with_begin(key_loc.end() - 1);
        let expression_l = key_loc.join(value.expression());

        let key = key_t.token_value.take();
        self.validate_sym_value(&key, &key_l);

        Node::new_pair(
            self.bump,
            Node::new_sym(self.bump, key, None, None, key_l),
            value,
            colon_l,
            expression_l,
        )
    }

    pub(crate) fn pair_quoted(
        &self,
        begin_t: &'a Token<'a>,
        parts: Vec<'a, &'a Node<'a>>,
        end_t: &'a Token<'a>,
        value: &'a Node<'a>,
    ) -> &'a Node<'a> {
        let end_l = loc(end_t);

        let quote_loc = Loc::new(end_l.end() - 2, end_l.end() - 1);

        let colon_l = end_l.with_begin(end_l.end() - 1);

        let end_t = end_t;
        let end_t: &'a Token = self.bump.alloc(Token::new(
            self.bump,
            end_t.token_type(),
            end_t.token_value.take(),
            quote_loc,
            LexState::default(),
            LexState::default(),
        ));
        let expression_l = loc(begin_t).join(value.expression());

        Node::new_pair(
            self.bump,
            self.symbol_compose(begin_t, parts, end_t),
            value,
            colon_l,
            expression_l,
        )
    }

    pub(crate) fn kwsplat(&self, dstar_t: &'a Token<'a>, value: &'a Node<'a>) -> &'a Node<'a> {
        let operator_l = loc(dstar_t);
        let expression_l = value.expression().join(&operator_l);

        Node::new_kwsplat(self.bump, value, operator_l, expression_l)
    }

    pub(crate) fn associate(
        &self,
        begin_t: Maybe<&'a Token<'a>>,
        pairs: Vec<'a, &'a Node<'a>>,
        end_t: Maybe<&'a Token<'a>>,
    ) -> &'a Node<'a> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = collection_map(&begin_t, &pairs, &end_t);

        Node::new_hash(self.bump, pairs, begin_l, end_l, expression_l)
    }

    // Ranges

    pub(crate) fn range_inclusive(
        &self,
        left: Maybe<&'a Node<'a>>,
        dot2_t: &'a Token<'a>,
        right: Maybe<&'a Node<'a>>,
    ) -> &'a Node<'a> {
        let operator_l = loc(dot2_t);
        let expression_l = operator_l
            .maybe_join(&maybe_node_expr_mut(&left))
            .maybe_join(&maybe_node_expr_mut(&right));

        Node::new_irange(self.bump, left, right, operator_l, expression_l)
    }

    pub(crate) fn range_exclusive(
        &self,
        left: Maybe<&'a Node<'a>>,
        dot3_t: &'a Token<'a>,
        right: Maybe<&'a Node<'a>>,
    ) -> &'a Node<'a> {
        let operator_l = loc(dot3_t);
        let expression_l = operator_l
            .maybe_join(&maybe_node_expr_mut(&left))
            .maybe_join(&maybe_node_expr_mut(&right));

        Node::new_erange(self.bump, left, right, operator_l, expression_l)
    }

    //
    // Access
    //

    pub(crate) fn self_(&self, token: &'a Token<'a>) -> &'a Node<'a> {
        Node::new_self(self.bump, loc(token))
    }

    pub(crate) fn lvar(&self, token: &'a Token<'a>) -> &'a Node<'a> {
        let expression_l = loc(token);
        Node::new_lvar(self.bump, value(token), expression_l)
    }

    pub(crate) fn ivar(&self, token: &'a Token<'a>) -> &'a Node<'a> {
        let expression_l = loc(token);
        Node::new_ivar(self.bump, value(token), expression_l)
    }

    pub(crate) fn gvar(&self, token: &'a Token<'a>) -> &'a Node<'a> {
        let expression_l = loc(token);
        Node::new_gvar(self.bump, value(token), expression_l)
    }

    pub(crate) fn cvar(&self, token: &'a Token<'a>) -> &'a Node<'a> {
        let expression_l = loc(token);
        Node::new_cvar(self.bump, value(token), expression_l)
    }

    pub(crate) fn back_ref(&self, token: &'a Token<'a>) -> &'a Node<'a> {
        let expression_l = loc(token);
        Node::new_back_ref(self.bump, value(token), expression_l)
    }

    const MAX_NTH_REF: usize = 0b111111111111111111111111111111;

    pub(crate) fn nth_ref(&self, token: &'a Token<'a>) -> &'a Node<'a> {
        let expression_l = loc(token);
        let name = value(token);
        let name = &name.as_str()[1..];
        let parsed = name.parse::<usize>();
        let name = String::from_str_in(name, self.bump);

        if parsed.is_err() || parsed.map(|n| n > Self::MAX_NTH_REF) == Ok(true) {
            self.warn(
                DiagnosticMessage::new_nth_ref_is_too_big(name.clone()),
                &expression_l,
            )
        }

        Node::new_nth_ref(self.bump, name, expression_l)
    }
    pub(crate) fn accessible(&self, node: &'a Node<'a>) -> &'a Node<'a> {
        match node {
            Node::Lvar(Lvar { name, expression_l }) => {
                let name_s = name.as_str();
                if self.static_env.is_declared(name_s) {
                    if let Some(current_arg) = self.current_arg_stack.top() {
                        if current_arg == name_s {
                            self.error(
                                DiagnosticMessage::new_circular_argument_reference(name.clone()),
                                &expression_l,
                            );
                        }
                    }

                    Node::new_lvar(self.bump, take_str(name), expression_l.clone())
                } else {
                    Node::new_send(
                        self.bump,
                        Maybe::none(),
                        take_str(name),
                        bump_vec![in self.bump; ],
                        None,
                        Some(expression_l.clone()),
                        None,
                        None,
                        None,
                        expression_l.clone(),
                    )
                }
            }
            other => other,
        }
    }

    pub(crate) fn const_(&self, name_t: &'a Token<'a>) -> &'a Node<'a> {
        let name_l = loc(name_t);
        let expression_l = name_l;

        Node::new_const(
            self.bump,
            Maybe::none(),
            value(name_t),
            None,
            name_l,
            expression_l,
        )
    }

    pub(crate) fn const_global(
        &self,
        t_colon3: &'a Token<'a>,
        name_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let scope = Node::new_cbase(self.bump, loc(t_colon3));

        let name_l = loc(name_t);
        let expression_l = scope.expression().join(&name_l);
        let double_colon_l = loc(t_colon3);

        Node::new_const(
            self.bump,
            Some(scope),
            value(name_t),
            Some(double_colon_l),
            name_l,
            expression_l,
        )
    }

    pub(crate) fn const_fetch(
        &self,
        scope: &'a Node<'a>,
        t_colon2: &'a Token<'a>,
        name_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let scope: &'a Node = scope;
        let name_l = loc(name_t);
        let expression_l = scope.expression().join(&name_l);
        let double_colon_l = loc(t_colon2);

        Node::new_const(
            self.bump,
            Some(scope),
            value(name_t),
            Some(double_colon_l),
            name_l,
            expression_l,
        )
    }

    pub(crate) fn __encoding__(&self, encoding_t: &'a Token<'a>) -> &'a Node<'a> {
        Node::new_encoding(self.bump, loc(encoding_t))
    }

    //
    // Assignments
    //

    pub(crate) fn assignable(&self, node: &'a Node<'a>) -> Result<&'a Node<'a>, ()> {
        let node = match node {
            Node::Cvar(Cvar { name, expression_l }) => Node::new_cvasgn(
                self.bump,
                take_str(name),
                Maybe::none(),
                expression_l.clone(),
                None,
                expression_l.clone(),
            ),
            Node::Ivar(Ivar { name, expression_l }) => Node::new_ivasgn(
                self.bump,
                take_str(name),
                Maybe::none(),
                expression_l.clone(),
                None,
                expression_l.clone(),
            ),
            Node::Gvar(Gvar { name, expression_l }) => Node::new_gvasgn(
                self.bump,
                take_str(name),
                Maybe::none(),
                expression_l.clone(),
                None,
                expression_l.clone(),
            ),
            Node::Const(Const {
                scope,
                name,
                double_colon_l,
                name_l,
                expression_l,
            }) => {
                if !self.context.is_dynamic_const_definition_allowed() {
                    self.error(
                        DiagnosticMessage::new_dynamic_constant_assignment(),
                        &expression_l,
                    );
                    return Err(());
                }
                Node::new_casgn(
                    self.bump,
                    take_maybe_node(scope),
                    take_str(name),
                    Maybe::none(),
                    double_colon_l.clone(),
                    name_l.clone(),
                    None,
                    expression_l.clone(),
                )
            }
            Node::Lvar(Lvar { name, expression_l }) => {
                let name_s = name.as_str();
                self.check_assignment_to_numparam(name_s, expression_l)?;
                self.check_reserved_for_numparam(name_s, expression_l)?;

                self.static_env.declare(name_s);

                Node::new_lvasgn(
                    self.bump,
                    take_str(name),
                    Maybe::none(),
                    expression_l.clone(),
                    None,
                    expression_l.clone(),
                )
            }
            Node::Self_(Self_ { expression_l }) => {
                self.error(DiagnosticMessage::new_cant_assign_to_self(), expression_l);
                return Err(());
            }
            Node::Nil(Nil { expression_l }) => {
                self.error(DiagnosticMessage::new_cant_assign_to_nil(), expression_l);
                return Err(());
            }
            Node::True(True { expression_l }) => {
                self.error(DiagnosticMessage::new_cant_assign_to_true(), expression_l);
                return Err(());
            }
            Node::False(False { expression_l }) => {
                self.error(DiagnosticMessage::new_cant_assign_to_false(), expression_l);
                return Err(());
            }
            Node::File(File { expression_l }) => {
                self.error(DiagnosticMessage::new_cant_assign_to_file(), expression_l);
                return Err(());
            }
            Node::Line(Line { expression_l }) => {
                self.error(DiagnosticMessage::new_cant_assign_to_line(), expression_l);
                return Err(());
            }
            Node::Encoding(Encoding { expression_l }) => {
                self.error(
                    DiagnosticMessage::new_cant_assign_to_encoding(),
                    expression_l,
                );
                return Err(());
            }
            Node::BackRef(BackRef { name, expression_l }) => {
                self.error(
                    DiagnosticMessage::new_cant_set_variable(take_str(name)),
                    expression_l,
                );
                return Err(());
            }
            Node::NthRef(NthRef { name, expression_l }) => {
                self.error(
                    DiagnosticMessage::new_cant_set_variable(String::from_str_in(
                        &format!("${}", name),
                        self.bump,
                    )),
                    expression_l,
                );
                return Err(());
            }
            other => {
                unreachable!("{:?} can't be used in assignment", other)
            }
        };

        Ok(node)
    }

    pub(crate) fn const_op_assignable(&self, node: &'a Node<'a>) -> &'a Node<'a> {
        match node {
            Node::Const(Const {
                scope,
                name,
                double_colon_l,
                name_l,
                expression_l,
            }) => Node::new_casgn(
                self.bump,
                take_maybe_node(scope),
                take_str(name),
                Maybe::none(),
                double_colon_l.clone(),
                name_l.clone(),
                None,
                expression_l.clone(),
            ),
            other => {
                unreachable!("unsupported const_op_assignable arument: {:?}", other)
            }
        }
    }

    pub(crate) fn assign(
        &self,
        lhs: &'a Node<'a>,
        eql_t: &'a Token<'a>,
        new_rhs: &'a Node<'a>,
    ) -> &'a Node<'a> {
        let op_l = Some(loc(eql_t));
        let expr_l = join_exprs(lhs, new_rhs);

        let lhs: &'a mut Node<'a> = unsafe { std::mem::transmute(lhs) };
        match lhs {
            Node::Cvasgn(cvasgn) => {
                cvasgn.set_expression_l(expr_l);
                cvasgn.set_operator_l(op_l);
                cvasgn.set_value(Some(new_rhs));
            }
            Node::Ivasgn(ivasgn) => {
                ivasgn.set_expression_l(expr_l);
                ivasgn.set_operator_l(op_l);
                ivasgn.set_value(Some(new_rhs));
            }
            Node::Gvasgn(gvasgn) => {
                gvasgn.set_expression_l(expr_l);
                gvasgn.set_operator_l(op_l);
                gvasgn.set_value(Some(new_rhs));
            }
            Node::Lvasgn(lvasgn) => {
                lvasgn.set_expression_l(expr_l);
                lvasgn.set_operator_l(op_l);
                lvasgn.set_value(Some(new_rhs));
            }
            Node::Casgn(casgn) => {
                casgn.set_expression_l(expr_l);
                casgn.set_operator_l(op_l);
                casgn.set_value(Some(new_rhs));
            }
            Node::IndexAsgn(index_asgn) => {
                index_asgn.set_expression_l(expr_l);
                index_asgn.set_operator_l(op_l);
                index_asgn.set_value(Some(new_rhs));
            }
            Node::Send(send) => {
                send.set_expression_l(expr_l);
                send.set_operator_l(op_l);
                if send.get_args().is_empty() {
                    send.set_args(bump_vec![in self.bump; new_rhs]);
                } else {
                    unreachable!("can't assign to method call with args")
                }
            }
            Node::CSend(c_send) => {
                c_send.set_expression_l(expr_l);
                c_send.set_operator_l(op_l);
                if c_send.get_args().is_empty() {
                    c_send.set_args(bump_vec![in self.bump; new_rhs]);
                } else {
                    unreachable!("can't assign to method call with args")
                }
            }
            _ => {
                unreachable!("{:?} can't be used in assignment", lhs)
            }
        }

        lhs
    }

    pub(crate) fn op_assign(
        &self,
        mut lhs: &'a Node<'a>,
        op_t: &'a Token<'a>,
        rhs: &'a Node<'a>,
    ) -> Result<&'a Node<'a>, ()> {
        let operator_l = loc(op_t);
        let mut operator = String::from(value(op_t));
        operator.pop();
        let expression_l = join_exprs(lhs, rhs);

        match lhs {
            Node::Gvasgn(_)
            | Node::Ivasgn(_)
            | Node::Lvasgn(_)
            | Node::Cvasgn(_)
            | Node::Casgn(_)
            | Node::Send(_)
            | Node::CSend(_) => {
                // ignore
            }
            Node::Index(Index {
                recv,
                indexes,
                begin_l,
                end_l,
                expression_l,
            }) => {
                lhs = Node::new_index_asgn(
                    self.bump,
                    recv,
                    take_vec(indexes),
                    Maybe::none(),
                    begin_l.clone(),
                    end_l.clone(),
                    None,
                    expression_l.clone(),
                );
            }
            Node::BackRef(BackRef { name, expression_l }) => {
                self.error(
                    DiagnosticMessage::new_cant_set_variable(take_str(name)),
                    &expression_l,
                );
                return Err(());
            }
            Node::NthRef(NthRef { name, expression_l }) => {
                self.error(
                    DiagnosticMessage::new_cant_set_variable(String::from_str_in(
                        &format!("${}", name),
                        self.bump,
                    )),
                    expression_l,
                );
                return Err(());
            }
            _ => {
                unreachable!("unsupported op_assign lhs {:?}", lhs)
            }
        };

        let recv: &'a Node = lhs;
        let value: &'a Node = rhs;

        let result = match &operator[..] {
            "&&" => Node::new_and_asgn(self.bump, recv, value, operator_l, expression_l),
            "||" => Node::new_or_asgn(self.bump, recv, value, operator_l, expression_l),
            _ => Node::new_op_asgn(
                self.bump,
                recv,
                String::from_str_in(&operator, self.bump),
                value,
                operator_l,
                expression_l,
            ),
        };

        Ok(result)
    }

    pub(crate) fn multi_lhs(
        &self,
        begin_t: Maybe<&'a Token<'a>>,
        items: Vec<'a, &'a Node<'a>>,
        end_t: Maybe<&'a Token<'a>>,
    ) -> &'a Node<'a> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = collection_map(&begin_t, &items, &end_t);

        Node::new_mlhs(self.bump, items, begin_l, end_l, expression_l)
    }

    pub(crate) fn multi_assign(
        &self,
        lhs: &'a Node<'a>,
        eql_t: &'a Token<'a>,
        rhs: &'a Node<'a>,
    ) -> &'a Node<'a> {
        let operator_l = loc(eql_t);
        let expression_l = join_exprs(lhs, rhs);

        Node::new_masgn(self.bump, lhs, rhs, operator_l, expression_l)
    }

    //
    // Class and module definition
    //

    pub(crate) fn def_class(
        &self,
        class_t: &'a Token<'a>,
        name: &'a Node<'a>,
        lt_t: Maybe<&'a Token<'a>>,
        superclass: Maybe<&'a Node<'a>>,
        body: Maybe<&'a Node<'a>>,
        end_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let keyword_l = loc(class_t);
        let end_l = loc(end_t);
        let operator_l = maybe_loc(&lt_t);
        let expression_l = keyword_l.join(&end_l);

        Node::new_class(
            self.bump,
            name,
            superclass,
            body,
            keyword_l,
            operator_l,
            end_l,
            expression_l,
        )
    }

    pub(crate) fn def_sclass(
        &self,
        class_t: &'a Token<'a>,
        lshift_t: &'a Token<'a>,
        expr: &'a Node<'a>,
        body: Maybe<&'a Node<'a>>,
        end_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let keyword_l = loc(class_t);
        let end_l = loc(end_t);
        let operator_l = loc(lshift_t);
        let expression_l = keyword_l.join(&end_l);

        Node::new_s_class(
            self.bump,
            expr,
            body,
            keyword_l,
            operator_l,
            end_l,
            expression_l,
        )
    }

    pub(crate) fn def_module(
        &self,
        module_t: &'a Token<'a>,
        name: &'a Node<'a>,
        body: Maybe<&'a Node<'a>>,
        end_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let keyword_l = loc(module_t);
        let end_l = loc(end_t);
        let expression_l = keyword_l.join(&end_l);

        Node::new_module(self.bump, name, body, keyword_l, end_l, expression_l)
    }

    //
    // Method (un)definition
    //

    pub(crate) fn def_method(
        &self,
        def_t: &'a Token<'a>,
        name_t: &'a Token<'a>,
        args: Maybe<&'a Node<'a>>,
        body: Maybe<&'a Node<'a>>,
        end_t: &'a Token<'a>,
    ) -> Result<&'a Node<'a>, ()> {
        let name_l = loc(name_t);
        let keyword_l = loc(def_t);
        let end_l = loc(end_t);
        let expression_l = keyword_l.join(&end_l);

        let name = value(name_t);
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Ok(Node::new_def(
            self.bump,
            name,
            args,
            body,
            keyword_l,
            name_l,
            Some(end_l),
            None,
            expression_l,
        ))
    }

    pub(crate) fn def_endless_method(
        &self,
        def_t: &'a Token<'a>,
        name_t: &'a Token<'a>,
        args: Maybe<&'a Node<'a>>,
        assignment_t: &'a Token<'a>,
        body: Maybe<&'a Node<'a>>,
    ) -> Result<&'a Node<'a>, ()> {
        let body_l = maybe_node_expr_mut(&body)
            .unwrap_or_else(|| unreachable!("endless method always has a body"));

        let keyword_l = loc(def_t);
        let expression_l = keyword_l.join(&body_l);
        let name_l = loc(name_t);
        let assignment_l = loc(assignment_t);

        let name = value(name_t);
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Ok(Node::new_def(
            self.bump,
            name,
            args,
            body,
            keyword_l,
            name_l,
            None,
            Some(assignment_l),
            expression_l,
        ))
    }

    pub(crate) fn def_singleton(
        &self,
        def_t: &'a Token<'a>,
        definee: &'a Node<'a>,
        dot_t: &'a Token<'a>,
        name_t: &'a Token<'a>,
        args: Maybe<&'a Node<'a>>,
        body: Maybe<&'a Node<'a>>,
        end_t: &'a Token<'a>,
    ) -> Result<&'a Node<'a>, ()> {
        let keyword_l = loc(def_t);
        let operator_l = loc(dot_t);
        let name_l = loc(name_t);
        let end_l = loc(end_t);
        let expression_l = keyword_l.join(&end_l);

        let name = value(name_t);
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Ok(Node::new_defs(
            self.bump,
            definee,
            name,
            args,
            body,
            keyword_l,
            operator_l,
            name_l,
            None,
            Some(end_l),
            expression_l,
        ))
    }

    pub(crate) fn def_endless_singleton(
        &self,
        def_t: &'a Token<'a>,
        definee: &'a Node<'a>,
        dot_t: &'a Token<'a>,
        name_t: &'a Token<'a>,
        args: Maybe<&'a Node<'a>>,
        assignment_t: &'a Token<'a>,
        body: Maybe<&'a Node<'a>>,
    ) -> Result<&'a Node<'a>, ()> {
        let body_l = maybe_node_expr_mut(&body)
            .unwrap_or_else(|| unreachable!("endless method always has body"));

        let keyword_l = loc(def_t);
        let operator_l = loc(dot_t);
        let name_l = loc(name_t);
        let assignment_l = loc(assignment_t);
        let expression_l = keyword_l.join(&body_l);

        let name = value(name_t);
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Ok(Node::new_defs(
            self.bump,
            definee,
            name,
            args,
            body,
            keyword_l,
            operator_l,
            name_l,
            Some(assignment_l),
            None,
            expression_l,
        ))
    }

    pub(crate) fn undef_method(
        &self,
        undef_t: &'a Token<'a>,
        names: Vec<'a, &'a Node<'a>>,
    ) -> &'a Node<'a> {
        let keyword_l = loc(undef_t);
        let expression_l = keyword_l.maybe_join(&collection_expr(&names));
        Node::new_undef(self.bump, names, keyword_l, expression_l)
    }

    pub(crate) fn alias(
        &self,
        alias_t: &'a Token<'a>,
        to: &'a Node<'a>,
        from: &'a Node<'a>,
    ) -> &'a Node<'a> {
        let keyword_l = loc(alias_t);
        let expression_l = keyword_l.join(from.expression());
        Node::new_alias(self.bump, to, from, keyword_l, expression_l)
    }

    //
    // Formal arguments
    //

    pub(crate) fn args(
        &self,
        begin_t: Maybe<&'a Token<'a>>,
        mut args: Vec<'a, &'a Node<'a>>,
        end_t: Maybe<&'a Token<'a>>,
    ) -> Maybe<&'a Node<'a>> {
        let map = self.bump.alloc(HashMap::new());
        match check_duplicate_args(&mut args, map) {
            Ok(_) => {}
            Err(err_loc) => self.error(DiagnosticMessage::new_duplicated_argument_name(), err_loc),
        }

        if begin_t.is_none() && args.is_empty() && end_t.is_none() {
            return Maybe::none();
        }

        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = collection_map(&begin_t, &args, &end_t);

        Some(Node::new_args(
            self.bump,
            args,
            expression_l,
            begin_l,
            end_l,
        ))
    }

    pub(crate) fn forward_only_args(
        &self,
        begin_t: &'a Token<'a>,
        dots_t: &'a Token<'a>,
        end_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let args = bump_vec![in self.bump; self.forward_arg(dots_t)];
        let begin_l = loc(begin_t);
        let end_l = loc(end_t);
        let expression_l = begin_l.join(&end_l);
        Node::new_args(self.bump, args, expression_l, Some(begin_l), Some(end_l))
    }

    pub(crate) fn forward_arg(&self, dots_t: &'a Token<'a>) -> &'a Node<'a> {
        Node::new_forward_arg(self.bump, loc(dots_t))
    }

    pub(crate) fn arg(&self, name_t: &'a Token<'a>) -> Result<&'a Node<'a>, ()> {
        let name_l = loc(name_t);
        let name = value(name_t);

        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Ok(Node::new_arg(self.bump, name, name_l))
    }

    pub(crate) fn optarg(
        &self,
        name_t: &'a Token<'a>,
        eql_t: &'a Token<'a>,
        default: &'a Node<'a>,
    ) -> Result<&'a Node<'a>, ()> {
        let operator_l = loc(eql_t);
        let name_l = loc(name_t);
        let expression_l = loc(name_t).join(default.expression());

        let name = value(name_t);
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Ok(Node::new_optarg(
            self.bump,
            name,
            default,
            name_l,
            operator_l,
            expression_l,
        ))
    }

    pub(crate) fn restarg(
        &self,
        star_t: &'a Token<'a>,
        name_t: Maybe<&'a Token<'a>>,
    ) -> Result<&'a Node<'a>, ()> {
        let (name, name_l) = if name_t.is_some() {
            let name_t = name_t.unwrap();
            let name_l = loc(name_t);
            let name = value(name_t);
            self.check_reserved_for_numparam(name.as_str(), &name_l)?;
            (Some(name), Some(name_l))
        } else {
            (Maybe::none(), None)
        };

        let operator_l = loc(star_t);
        let expression_l = operator_l.maybe_join(&name_l);

        Ok(Node::new_restarg(
            self.bump,
            name,
            operator_l,
            name_l,
            expression_l,
        ))
    }

    pub(crate) fn kwarg(&self, name_t: &'a Token<'a>) -> Result<&'a Node<'a>, ()> {
        let name_l = loc(name_t);
        let name = value(name_t);
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        let expression_l = name_l;
        let name_l = expression_l.adjust_end(-1);

        Ok(Node::new_kwarg(self.bump, name, name_l, expression_l))
    }

    pub(crate) fn kwoptarg(
        &self,
        name_t: &'a Token<'a>,
        default: &'a Node<'a>,
    ) -> Result<&'a Node<'a>, ()> {
        let name_l = loc(name_t);
        let name = value(name_t);
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        let label_l = name_l;
        let name_l = label_l.adjust_end(-1);
        let expression_l = default.expression().join(&label_l);

        Ok(Node::new_kwoptarg(
            self.bump,
            name,
            default,
            name_l,
            expression_l,
        ))
    }

    pub(crate) fn kwrestarg(
        &self,
        dstar_t: &'a Token<'a>,
        name_t: Maybe<&'a Token<'a>>,
    ) -> Result<&'a Node<'a>, ()> {
        let (name, name_l) = if name_t.is_some() {
            let name_t = name_t.unwrap();
            let name_l = loc(name_t);
            let name = value(name_t);
            self.check_reserved_for_numparam(name.as_str(), &name_l)?;
            (Some(name), Some(name_l))
        } else {
            (Maybe::none(), None)
        };

        let operator_l = loc(dstar_t);
        let expression_l = operator_l.maybe_join(&name_l);

        Ok(Node::new_kwrestarg(
            self.bump,
            name,
            operator_l,
            name_l,
            expression_l,
        ))
    }

    pub(crate) fn kwnilarg(&self, dstar_t: &'a Token<'a>, nil_t: &'a Token<'a>) -> &'a Node<'a> {
        let dstar_l = loc(dstar_t);
        let nil_l = loc(nil_t);
        let expression_l = dstar_l.join(&nil_l);
        Node::new_kwnilarg(self.bump, nil_l, expression_l)
    }

    pub(crate) fn shadowarg(&self, name_t: &'a Token<'a>) -> Result<&'a Node<'a>, ()> {
        let name_l = loc(name_t);
        let name = value(name_t);
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Ok(Node::new_shadowarg(self.bump, name, name_l))
    }

    pub(crate) fn blockarg(
        &self,
        amper_t: &'a Token<'a>,
        name_t: &'a Token<'a>,
    ) -> Result<&'a Node<'a>, ()> {
        let name_l = loc(name_t);
        let name = value(name_t);
        self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        let operator_l = loc(amper_t);
        let expression_l = operator_l.join(&name_l);

        Ok(Node::new_blockarg(
            self.bump,
            name,
            operator_l,
            name_l,
            expression_l,
        ))
    }

    pub(crate) fn procarg0(&self, arg: &'a Node<'a>) -> &'a Node<'a> {
        match arg {
            Node::Mlhs(Mlhs {
                items,
                begin_l,
                end_l,
                expression_l,
            }) => Node::new_procarg0(
                self.bump,
                take_vec(items),
                begin_l.clone(),
                end_l.clone(),
                expression_l.clone(),
            ),
            Node::Arg(Arg { expression_l, .. }) => {
                let expression_l = expression_l.clone();
                Node::new_procarg0(
                    self.bump,
                    bump_vec![in self.bump; arg],
                    None,
                    None,
                    expression_l,
                )
            }
            _ => {
                unreachable!("unsupported procarg0 child {:?}", arg)
            }
        }
    }

    //
    // Method calls
    //

    fn call_type_for_dot(&self, dot_t: &Maybe<&'a Token<'a>>) -> MethodCallType {
        // match dot_t.as_ref() {
        //     Some(token) if token.token_type() == Lexer::tANDDOT => MethodCallType::CSend,
        //     _ => MethodCallType::Send,
        // }
        MethodCallType::Send
    }

    pub(crate) fn forwarded_args(&self, dots_t: &'a Token<'a>) -> &'a Node<'a> {
        Node::new_forwarded_args(self.bump, loc(dots_t))
    }

    pub(crate) fn call_method(
        &self,
        receiver: Maybe<&'a Node<'a>>,
        dot_t: Maybe<&'a Token<'a>>,
        selector_t: Maybe<&'a Token<'a>>,
        lparen_t: Maybe<&'a Token<'a>>,
        mut args: Vec<'a, &'a Node<'a>>,
        rparen_t: Maybe<&'a Token<'a>>,
    ) -> &'a Node<'a> {
        let begin_l = maybe_node_expr_mut(&receiver)
            .or_else(|| maybe_loc(&selector_t))
            .unwrap_or_else(|| unreachable!("can't compute begin_l"));

        let end_l = maybe_loc(&rparen_t)
            .or_else(|| maybe_node_expr_mut(&args.last().map(|x| &**x)))
            .or_else(|| maybe_loc(&selector_t))
            .unwrap_or_else(|| unreachable!("can't compute end_l"));

        let expression_l = begin_l.join(&end_l);

        let dot_l = maybe_loc(&dot_t);
        let selector_l = maybe_loc(&selector_t);
        let begin_l = maybe_loc(&lparen_t);
        let end_l = maybe_loc(&rparen_t);

        let method_name = maybe_value(selector_t);
        let method_name = if method_name.is_some() {
            method_name.unwrap()
        } else {
            String::from_str_in("call", self.bump)
        };

        self.rewrite_hash_args_to_kwargs(&mut args);

        match self.call_type_for_dot(&dot_t) {
            MethodCallType::Send => Node::new_send(
                self.bump,
                receiver,
                method_name,
                args,
                dot_l,
                selector_l,
                begin_l,
                end_l,
                None,
                expression_l,
            ),

            MethodCallType::CSend => Node::new_c_send(
                self.bump,
                receiver.expect("csend node must have a receiver"),
                method_name,
                args,
                dot_l.expect("csend node must have &."),
                selector_l,
                begin_l,
                end_l,
                None,
                expression_l,
            ),
        }
    }

    pub(crate) fn call_lambda(&self, lambda_t: &'a Token<'a>) -> &'a Node<'a> {
        Node::new_lambda(self.bump, loc(lambda_t))
    }

    fn validate_block_and_block_arg(&self, args: &Vec<'a, &'a Node<'a>>) -> Result<(), ()> {
        if let Some(last_arg) = args.last() {
            if last_arg.is_block_pass() || last_arg.is_forwarded_args() {
                self.error(
                    DiagnosticMessage::new_block_and_block_arg_given(),
                    last_arg.expression(),
                );
                Err(())
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    pub(crate) fn block(
        &self,
        method_call: &'a Node<'a>,
        begin_t: &'a Token<'a>,
        block_args: ArgsType<'a>,
        body: Maybe<&'a Node<'a>>,
        end_t: &'a Token<'a>,
    ) -> Result<&'a Node<'a>, ()> {
        let block_body = body;

        match method_call {
            Node::Yield(Yield { keyword_l, .. }) => {
                self.error(DiagnosticMessage::new_block_given_to_yield(), keyword_l);
                return Err(());
            }
            Node::Send(Send { args, .. }) => {
                self.validate_block_and_block_arg(args)?;
            }
            Node::CSend(CSend { args, .. }) => {
                self.validate_block_and_block_arg(args)?;
            }
            _ => {}
        }

        fn rewrite_args_and_loc<'a>(
            bump: &'a Bump,
            begin_t: &'a Token<'a>,
            end_t: &'a Token<'a>,
            method_args: &'a Vec<'a, &'a Node<'a>>,
            keyword_expression_l: &'a Loc,
            block_args: ArgsType<'a>,
            block_body: Maybe<&'a Node<'a>>,
        ) -> (Vec<'a, &'a Node<'a>>, Loc) {
            let method_args: &'a mut Vec<'a, &'a Node<'a>> =
                unsafe { std::mem::transmute(method_args) };
            // Code like "return foo 1 do end" is reduced in a weird sequence.
            // Here, method_call is actually (return).
            let actual_send = method_args.pop().unwrap();

            let begin_l = loc(begin_t);
            let end_l = loc(end_t);
            let expression_l = actual_send.expression().join(&end_l);

            let block = match block_args {
                ArgsType::Args(args) => Node::new_block(
                    bump,
                    actual_send,
                    args,
                    block_body,
                    begin_l,
                    end_l,
                    expression_l,
                ),
                ArgsType::Numargs(numargs) => Node::new_numblock(
                    bump,
                    actual_send,
                    numargs,
                    block_body.expect("numblock always has body"),
                    begin_l,
                    end_l,
                    expression_l,
                ),
            };

            let expr_l = keyword_expression_l.join(block.expression());

            (bump_vec![in bump; block], expr_l)
        };

        match method_call {
            Node::Send(_)
            | Node::CSend(_)
            | Node::Index(_)
            | Node::Super(_)
            | Node::ZSuper(_)
            | Node::Lambda(_) => {
                let begin_l = loc(begin_t);
                let end_l = loc(end_t);
                let expression_l = method_call.expression().join(&end_l);

                let result = match block_args {
                    ArgsType::Args(args) => Node::new_block(
                        self.bump,
                        method_call,
                        args,
                        block_body,
                        begin_l,
                        end_l,
                        expression_l,
                    ),
                    ArgsType::Numargs(numargs) => Node::new_numblock(
                        self.bump,
                        method_call,
                        numargs,
                        {
                            let block_body: Maybe<&'a Node<'a>> = block_body;
                            block_body.expect("numblock always has body")
                        },
                        begin_l,
                        end_l,
                        expression_l,
                    ),
                };
                return Ok(result);
            }
            _ => {}
        };

        let method_call = method_call;
        let result = match method_call {
            Node::Return(Return {
                args,
                keyword_l,
                expression_l,
            }) => {
                let (args, expression_l) = rewrite_args_and_loc(
                    self.bump,
                    begin_t,
                    end_t,
                    args,
                    expression_l,
                    block_args,
                    block_body,
                );
                Node::new_return(self.bump, args, keyword_l.clone(), expression_l)
            }
            Node::Next(Next {
                args,
                keyword_l,
                expression_l,
            }) => {
                let (args, expression_l) = rewrite_args_and_loc(
                    self.bump,
                    begin_t,
                    end_t,
                    args,
                    expression_l,
                    block_args,
                    block_body,
                );
                Node::new_next(self.bump, args, keyword_l.clone(), expression_l)
            }
            Node::Break(Break {
                args,
                keyword_l,
                expression_l,
            }) => {
                let (args, expression_l) = rewrite_args_and_loc(
                    self.bump,
                    begin_t,
                    end_t,
                    args,
                    expression_l,
                    block_args,
                    block_body,
                );
                Node::new_break(self.bump, args, keyword_l.clone(), expression_l)
            }
            _ => {
                unreachable!("unsupported method call {:?}", method_call)
            }
        };

        Ok(result)
    }
    pub(crate) fn block_pass(&self, amper_t: &'a Token<'a>, value: &'a Node<'a>) -> &'a Node<'a> {
        let amper_l = loc(amper_t);
        let expression_l = value.expression().join(&amper_l);

        Node::new_block_pass(self.bump, value, amper_l, expression_l)
    }

    pub(crate) fn attr_asgn(
        &self,
        receiver: &'a Node<'a>,
        dot_t: &'a Token<'a>,
        selector_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let dot_l = loc(dot_t);
        let selector_l = loc(selector_t);
        let expression_l = receiver.expression().join(&selector_l);
        let receiver: &'a Node = receiver;

        let method_name = String::from_str_in(&(value(selector_t) + "="), self.bump);

        match self.call_type_for_dot(&Some(dot_t)) {
            MethodCallType::Send => Node::new_send(
                self.bump,
                Some(receiver),
                method_name,
                bump_vec![in self.bump; ],
                Some(dot_l),
                Some(selector_l),
                None,
                None,
                None,
                expression_l,
            ),

            MethodCallType::CSend => Node::new_c_send(
                self.bump,
                receiver,
                method_name,
                bump_vec![in self.bump; ],
                dot_l,
                Some(selector_l),
                None,
                None,
                None,
                expression_l,
            ),
        }
    }

    pub(crate) fn index(
        &self,
        recv: &'a Node<'a>,
        lbrack_t: &'a Token<'a>,
        mut indexes: Vec<'a, &'a Node<'a>>,
        rbrack_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let begin_l = loc(lbrack_t);
        let end_l = loc(rbrack_t);
        let expression_l = recv.expression().join(&end_l);

        self.rewrite_hash_args_to_kwargs(&mut indexes);

        Node::new_index(self.bump, recv, indexes, begin_l, end_l, expression_l)
    }

    pub(crate) fn index_asgn(
        &self,
        recv: &'a Node<'a>,
        lbrack_t: &'a Token<'a>,
        indexes: Vec<'a, &'a Node<'a>>,
        rbrack_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let begin_l = loc(lbrack_t);
        let end_l = loc(rbrack_t);
        let expression_l = recv.expression().join(&end_l);

        Node::new_index_asgn(
            self.bump,
            recv,
            indexes,
            Maybe::none(),
            begin_l,
            end_l,
            None,
            expression_l,
        )
    }

    pub(crate) fn binary_op(
        &self,
        receiver: &'a Node<'a>,
        operator_t: &'a Token<'a>,
        arg: &'a Node<'a>,
    ) -> Result<&'a Node<'a>, ()> {
        let receiver = self.value_expr(receiver)?;
        let arg = self.value_expr(arg)?;

        let selector_l = Some(loc(operator_t));
        let expression_l = join_exprs(receiver, arg);

        Ok(Node::new_send(
            self.bump,
            Some(receiver),
            value(operator_t),
            bump_vec![in self.bump; arg],
            None,
            selector_l,
            None,
            None,
            None,
            expression_l,
        ))
    }

    pub(crate) fn match_op(
        &self,
        receiver: &'a Node<'a>,
        match_t: &'a Token<'a>,
        arg: &'a Node<'a>,
    ) -> Result<&'a Node<'a>, ()> {
        let receiver = self.value_expr(receiver)?;
        let arg = self.value_expr(arg)?;

        let selector_l = loc(match_t);
        let expression_l = join_exprs(receiver, arg);

        let result = match self.static_regexp_captures(&receiver) {
            Some(captures) => {
                for capture in captures {
                    self.static_env.declare(&capture);
                }

                Node::new_match_with_lvasgn(self.bump, receiver, arg, selector_l, expression_l)
            }
            None => Node::new_send(
                self.bump,
                Some(receiver),
                String::from_str_in("=~", self.bump),
                bump_vec![in self.bump; arg],
                None,
                Some(selector_l),
                None,
                None,
                None,
                expression_l,
            ),
        };

        Ok(result)
    }

    pub(crate) fn unary_op(
        &self,
        op_t: &'a Token<'a>,
        receiver: &'a Node<'a>,
    ) -> Result<&'a Node<'a>, ()> {
        let receiver = self.value_expr(receiver)?;

        let selector_l = loc(op_t);
        let expression_l = receiver.expression().join(&selector_l);

        let mut method_name = value(op_t);
        if method_name == "+" || method_name == "-" {
            method_name += "@";
        }
        Ok(Node::new_send(
            self.bump,
            Some(receiver),
            method_name,
            bump_vec![in self.bump; ],
            None,
            Some(selector_l),
            None,
            None,
            None,
            expression_l,
        ))
    }

    pub(crate) fn not_op(
        &self,
        not_t: &'a Token<'a>,
        begin_t: Maybe<&'a Token<'a>>,
        receiver: Maybe<&'a Node<'a>>,
        end_t: Maybe<&'a Token<'a>>,
    ) -> Result<&'a Node<'a>, ()> {
        match receiver {
            Some(receiver) => {
                let receiver = self.value_expr(receiver)?;

                let begin_l = loc(not_t);
                let end_l = maybe_loc(&end_t).unwrap_or_else(|| receiver.expression().clone());

                let expression_l = begin_l.join(&end_l);

                let selector_l = loc(not_t);
                let begin_l = maybe_loc(&begin_t);
                let end_l = maybe_loc(&end_t);

                Ok(Node::new_send(
                    self.bump,
                    Some(self.check_condition(receiver)),
                    String::from_str_in("!", self.bump),
                    bump_vec![in self.bump; ],
                    None,
                    Some(selector_l),
                    begin_l,
                    end_l,
                    None,
                    expression_l,
                ))
            }
            None => {
                let CollectionMap {
                    begin_l,
                    end_l,
                    expression_l,
                } = collection_map(&begin_t, &[], &end_t);

                let nil_node = Node::new_begin(
                    self.bump,
                    bump_vec![in self.bump; ],
                    begin_l,
                    end_l,
                    expression_l,
                );

                let selector_l = loc(not_t);
                let expression_l = nil_node.expression().join(&selector_l);
                Ok(Node::new_send(
                    self.bump,
                    Some(nil_node),
                    String::from_str_in("!", self.bump),
                    bump_vec![in self.bump; ],
                    None,
                    Some(selector_l),
                    None,
                    None,
                    None,
                    expression_l,
                ))
            }
        }
    }

    //
    // Control flow
    //

    // Logical operations: and, or

    pub(crate) fn logical_op(
        &self,
        type_: LogicalOp,
        lhs: &'a Node<'a>,
        op_t: &'a Token<'a>,
        rhs: &'a Node<'a>,
    ) -> Result<&'a Node<'a>, ()> {
        let lhs = self.value_expr(lhs)?;

        let operator_l = loc(op_t);
        let expression_l = join_exprs(lhs, rhs);
        let lhs: &'a Node = lhs;
        let rhs: &'a Node = rhs;

        let result = match type_ {
            LogicalOp::And => Node::new_and(self.bump, lhs, rhs, operator_l, expression_l),
            LogicalOp::Or => Node::new_or(self.bump, lhs, rhs, operator_l, expression_l),
        };
        Ok(result)
    }

    // Conditionals

    pub(crate) fn condition(
        &self,
        cond_t: &'a Token<'a>,
        cond: &'a Node<'a>,
        then_t: &'a Token<'a>,
        if_true: Maybe<&'a Node<'a>>,
        else_t: Maybe<&'a Token<'a>>,
        if_false: Maybe<&'a Node<'a>>,
        end_t: Maybe<&'a Token<'a>>,
    ) -> &'a Node<'a> {
        let mut end_l = maybe_loc(&end_t);
        if end_l.is_none() {
            end_l = maybe_node_expr_mut(&if_false);
        }
        if end_l.is_none() {
            end_l = maybe_loc(&else_t);
        }
        if end_l.is_none() {
            end_l = maybe_node_expr_mut(&if_true);
        }
        let end_l = if end_l.is_none() {
            loc(then_t)
        } else {
            end_l.unwrap()
        };

        let expression_l = loc(cond_t).join(&end_l);
        let keyword_l = loc(cond_t);
        let begin_l = loc(then_t);
        let else_l = maybe_loc(&else_t);
        let end_l = maybe_loc(&end_t);

        Node::new_if(
            self.bump,
            self.check_condition(cond),
            if_true,
            if_false,
            keyword_l,
            begin_l,
            else_l,
            end_l,
            expression_l,
        )
    }

    pub(crate) fn condition_mod(
        &self,
        if_true: Maybe<&'a Node<'a>>,
        if_false: Maybe<&'a Node<'a>>,
        cond_t: &'a Token<'a>,
        cond: &'a Node<'a>,
    ) -> &'a Node<'a> {
        let pre = match (if_true.as_ref(), if_false.as_ref()) {
            (None, None) => unreachable!("at least one of if_true/if_false is required"),
            (None, Some(if_false)) => if_false,
            (Some(if_true), None) => if_true,
            (Some(_), Some(_)) => unreachable!("only one of if_true/if_false is required"),
        };

        let expression_l = pre.expression().join(cond.expression());
        let keyword_l = loc(cond_t);

        Node::new_if_mod(
            self.bump,
            self.check_condition(cond),
            if_true,
            if_false,
            keyword_l,
            expression_l,
        )
    }

    pub(crate) fn ternary(
        &self,
        cond: &'a Node<'a>,
        question_t: &'a Token<'a>,
        if_true: &'a Node<'a>,
        colon_t: &'a Token<'a>,
        if_false: &'a Node<'a>,
    ) -> &'a Node<'a> {
        let expression_l = join_exprs(cond, if_false);
        let question_l = loc(question_t);
        let colon_l = loc(colon_t);

        Node::new_if_ternary(
            self.bump,
            cond,
            if_true,
            if_false,
            question_l,
            colon_l,
            expression_l,
        )
    }

    // Case matching

    pub(crate) fn when(
        &self,
        when_t: &'a Token<'a>,
        mut patterns: Vec<'a, &'a Node<'a>>,
        then_t: &'a Token<'a>,
        body: Maybe<&'a Node<'a>>,
    ) -> &'a Node<'a> {
        let begin_l = loc(then_t);

        let expr_end_l = maybe_node_expr_mut(&body)
            .or_else(|| maybe_node_expr_mut(&patterns.last().map(|x| &**x)))
            .unwrap_or_else(|| loc(when_t));
        let when_l = loc(when_t);
        let expression_l = when_l.join(&expr_end_l);

        Node::new_when(self.bump, patterns, body, when_l, begin_l, expression_l)
    }

    pub(crate) fn case(
        &self,
        case_t: &'a Token<'a>,
        expr: Maybe<&'a Node<'a>>,
        when_bodies: Vec<'a, &'a Node<'a>>,
        else_t: Maybe<&'a Token<'a>>,
        else_body: Maybe<&'a Node<'a>>,
        end_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let keyword_l = loc(case_t);
        let else_l = maybe_loc(&else_t);
        let end_l = loc(end_t);
        let expression_l = keyword_l.join(&end_l);

        Node::new_case(
            self.bump,
            expr,
            when_bodies,
            else_body,
            keyword_l,
            else_l,
            end_l,
            expression_l,
        )
    }

    // Loops

    pub(crate) fn loop_(
        &self,
        loop_type: LoopType,
        keyword_t: &'a Token<'a>,
        cond: &'a Node<'a>,
        do_t: &'a Token<'a>,
        body: Maybe<&'a Node<'a>>,
        end_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let keyword_l = loc(keyword_t);
        let begin_l = loc(do_t);
        let end_l = loc(end_t);
        let expression_l = loc(keyword_t).join(&end_l);

        let cond = self.check_condition(cond);

        match loop_type {
            LoopType::While => Node::new_while(
                self.bump,
                cond,
                body,
                keyword_l,
                Some(begin_l),
                Some(end_l),
                expression_l,
            ),
            LoopType::Until => Node::new_until(
                self.bump,
                cond,
                body,
                keyword_l,
                Some(begin_l),
                Some(end_l),
                expression_l,
            ),
        }
    }

    pub(crate) fn loop_mod(
        &self,
        loop_type: LoopType,
        body: &'a Node<'a>,
        keyword_t: &'a Token<'a>,
        cond: &'a Node<'a>,
    ) -> &'a Node<'a> {
        let expression_l = body.expression().join(cond.expression());
        let keyword_l = loc(keyword_t);

        let cond = self.check_condition(cond);

        match (loop_type, &*body) {
            (LoopType::While, node) if node.is_kw_begin() => {
                Node::new_while_post(self.bump, cond, body, keyword_l, expression_l)
            }
            (LoopType::While, _) => Node::new_while(
                self.bump,
                cond,
                Some(body),
                keyword_l,
                None,
                None,
                expression_l,
            ),
            (LoopType::Until, node) if node.is_kw_begin() => {
                Node::new_until_post(self.bump, cond, body, keyword_l, expression_l)
            }
            (LoopType::Until, _) => Node::new_until(
                self.bump,
                cond,
                Some(body),
                keyword_l,
                None,
                None,
                expression_l,
            ),
        }
    }

    pub(crate) fn for_(
        &self,
        for_t: &'a Token<'a>,
        iterator: &'a Node<'a>,
        in_t: &'a Token<'a>,
        iteratee: &'a Node<'a>,
        do_t: &'a Token<'a>,
        body: Maybe<&'a Node<'a>>,
        end_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let keyword_l = loc(for_t);
        let operator_l = loc(in_t);
        let begin_l = loc(do_t);
        let end_l = loc(end_t);
        let expression_l = keyword_l.join(&end_l);

        Node::new_for(
            self.bump,
            iterator,
            iteratee,
            body,
            keyword_l,
            operator_l,
            begin_l,
            end_l,
            expression_l,
        )
    }

    // Keywords

    pub(crate) fn keyword_cmd(
        &self,
        type_: KeywordCmd,
        keyword_t: &'a Token<'a>,
        lparen_t: Maybe<&'a Token<'a>>,
        mut args: Vec<'a, &'a Node<'a>>,
        rparen_t: Maybe<&'a Token<'a>>,
    ) -> Result<&'a Node<'a>, ()> {
        let keyword_l = loc(keyword_t);

        if type_ == KeywordCmd::Yield && !args.is_empty() {
            if let Some(last_arg) = args.last() {
                if last_arg.is_block_pass() {
                    self.error(DiagnosticMessage::new_block_given_to_yield(), &keyword_l);
                    return Err(());
                }
            }
        }

        match type_ {
            KeywordCmd::Yield | KeywordCmd::Super => {
                self.rewrite_hash_args_to_kwargs(&mut args);
            }
            _ => {}
        }

        let begin_l = maybe_loc(&lparen_t);
        let end_l = maybe_loc(&rparen_t);

        let expr_end_l = end_l
            .or_else(|| maybe_node_expr_mut(&args.last().map(|x| &**x)))
            .unwrap_or_else(|| keyword_l.clone());

        let expression_l = keyword_l.join(&expr_end_l);

        let result = match type_ {
            KeywordCmd::Break => Node::new_break(self.bump, args, keyword_l, expression_l),
            KeywordCmd::Defined => Node::new_defined(
                self.bump,
                args.pop().unwrap(),
                keyword_l,
                begin_l,
                end_l,
                expression_l,
            ),
            KeywordCmd::Next => Node::new_next(self.bump, args, keyword_l, expression_l),
            KeywordCmd::Redo => Node::new_redo(self.bump, expression_l),
            KeywordCmd::Retry => Node::new_retry(self.bump, expression_l),
            KeywordCmd::Return => Node::new_return(self.bump, args, keyword_l, expression_l),
            KeywordCmd::Super => {
                Node::new_super(self.bump, args, keyword_l, begin_l, end_l, expression_l)
            }
            KeywordCmd::Yield => {
                Node::new_yield(self.bump, args, keyword_l, begin_l, end_l, expression_l)
            }
            KeywordCmd::Zsuper => Node::new_z_super(self.bump, expression_l),
        };

        Ok(result)
    }

    // BEGIN, END

    pub(crate) fn preexe(
        &self,
        preexe_t: &'a Token<'a>,
        lbrace_t: &'a Token<'a>,
        body: Maybe<&'a Node<'a>>,
        rbrace_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let keyword_l = loc(preexe_t);
        let begin_l = loc(lbrace_t);
        let end_l = loc(rbrace_t);
        let expression_l = keyword_l.join(&end_l);

        Node::new_preexe(self.bump, body, keyword_l, begin_l, end_l, expression_l)
    }
    pub(crate) fn postexe(
        &self,
        postexe_t: &'a Token<'a>,
        lbrace_t: &'a Token<'a>,
        body: Maybe<&'a Node<'a>>,
        rbrace_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let keyword_l = loc(postexe_t);
        let begin_l = loc(lbrace_t);
        let end_l = loc(rbrace_t);
        let expression_l = keyword_l.join(&end_l);

        Node::new_postexe(self.bump, body, keyword_l, begin_l, end_l, expression_l)
    }

    // Exception handling

    pub(crate) fn rescue_body(
        &self,
        rescue_t: &'a Token<'a>,
        exc_list: Maybe<&'a Node<'a>>,
        assoc_t: Maybe<&'a Token<'a>>,
        exc_var: Maybe<&'a Node<'a>>,
        then_t: Maybe<&'a Token<'a>>,
        body: Maybe<&'a Node<'a>>,
    ) -> &'a Node<'a> {
        let mut end_l = maybe_node_expr_mut(&body);
        if end_l.is_none() {
            end_l = maybe_loc(&then_t);
        }
        if end_l.is_none() {
            end_l = maybe_node_expr_mut(&exc_var);
        }
        if end_l.is_none() {
            end_l = maybe_node_expr_mut(&exc_list);
        }
        let end_l = if end_l.is_none() {
            loc(rescue_t)
        } else {
            end_l.unwrap()
        };

        let expression_l = loc(rescue_t).join(&end_l);
        let keyword_l = loc(rescue_t);
        let assoc_l = maybe_loc(&assoc_t);
        let begin_l = maybe_loc(&then_t);

        Node::new_rescue_body(
            self.bump,
            exc_list,
            exc_var,
            body,
            keyword_l,
            assoc_l,
            begin_l,
            expression_l,
        )
    }

    pub(crate) fn begin_body(
        &self,
        compound_stmt: Maybe<&'a Node<'a>>,
        mut rescue_bodies: Vec<'a, &'a Node<'a>>,
        else_: Option<(&'a Token<'a>, Maybe<&'a Node<'a>>)>,
        ensure: Option<(&'a Token<'a>, Maybe<&'a Node<'a>>)>,
    ) -> Maybe<&'a Node<'a>> {
        let mut result: Maybe<&'a Node<'a>>;

        if !rescue_bodies.is_empty() {
            if let Some((else_t, else_)) = else_ {
                let mut begin_l = maybe_node_expr_mut(&compound_stmt);
                if begin_l.is_none() {
                    begin_l = maybe_node_expr_mut(&rescue_bodies.first_mut().map(|x| &**x));
                }
                let begin_l = if begin_l.is_none() {
                    unreachable!("can't compute begin_l")
                } else {
                    begin_l.unwrap()
                };

                let end_l = maybe_node_expr_mut(&else_).unwrap_or_else(|| loc(else_t));

                let expression_l = begin_l.join(&end_l);
                let else_l = loc(else_t);

                result = Some(Node::new_rescue(
                    self.bump,
                    compound_stmt,
                    rescue_bodies,
                    else_,
                    Some(else_l),
                    expression_l,
                ))
            } else {
                let mut begin_l = maybe_node_expr_mut(&compound_stmt);
                if begin_l.is_none() {
                    begin_l = maybe_node_expr_mut(&rescue_bodies.first().map(|v| &**v));
                }
                let begin_l = if begin_l.is_none() {
                    unreachable!("can't compute begin_l")
                } else {
                    begin_l.unwrap()
                };

                let end_l = maybe_node_expr_mut(&rescue_bodies.last().map(|v| &**v))
                    .unwrap_or_else(|| unreachable!("can't compute end_l"));

                let expression_l = begin_l.join(&end_l);
                let else_l = maybe_loc(&Maybe::none());

                result = Some(Node::new_rescue(
                    self.bump,
                    compound_stmt,
                    rescue_bodies,
                    Maybe::none(),
                    else_l,
                    expression_l,
                ))
            }
        } else if let Some((else_t, else_)) = else_ {
            let mut statements = bump_vec![in self.bump; ];

            // let compound_stmt = compound_stmt.map(|boxed| *boxed);
            if compound_stmt.is_some() {
                let compound_stmt = compound_stmt.unwrap();
                match compound_stmt {
                    Node::Begin(Begin {
                        statements: stmts, ..
                    }) => {
                        statements = take_vec(stmts);
                    }
                    _ => statements.push(compound_stmt),
                }
                // if compound_stmt.is_begin() {
                //     let internal::Begin {
                //         statements: stmts, ..
                //     } = compound_stmt.into_begin(self.bump).into_internal(self.bump);
                //     statements = *stmts;
                // } else {
                //     statements.push(compound_stmt)
                // }
            }

            let parts = if else_.is_some() {
                bump_vec![in self.bump; else_.unwrap()]
            } else {
                bump_vec![in self.bump; ]
            };
            let CollectionMap {
                begin_l,
                end_l,
                expression_l,
            } = collection_map(&Some(else_t), &parts, &Maybe::none());

            statements.push(Node::new_begin(
                self.bump,
                parts,
                begin_l,
                end_l,
                expression_l,
            ));

            let CollectionMap {
                begin_l,
                end_l,
                expression_l,
            } = collection_map(&Maybe::none(), &statements, &Maybe::none());

            result = Some(Node::new_begin(
                self.bump,
                statements,
                begin_l,
                end_l,
                expression_l,
            ))
        } else {
            result = compound_stmt;
        }

        if let Some((ensure_t, mut ensure_body)) = ensure {
            // let ensure_body = ensure;
            let keyword_l = loc(ensure_t);

            let begin_l = maybe_node_expr_mut(&result).unwrap_or_else(|| loc(ensure_t));

            let end_l = maybe_node_expr_mut(&ensure_body.as_mut().map(|x| &**x))
                .unwrap_or_else(|| loc(ensure_t));

            let expression_l = begin_l.join(&end_l);

            result = Some(Node::new_ensure(
                self.bump,
                result,
                ensure_body,
                keyword_l,
                expression_l,
            ))
        }

        result
    }

    //
    // Expression grouping
    //

    pub(crate) fn compstmt(&self, mut statements: Vec<'a, &'a Node<'a>>) -> Maybe<&'a Node<'a>> {
        match &statements[..] {
            [] => Maybe::none(),
            [_] => Some(statements.pop().unwrap()),
            _ => {
                let CollectionMap {
                    begin_l,
                    end_l,
                    expression_l,
                } = collection_map(&Maybe::none(), &statements, &Maybe::none());

                Some(Node::new_begin(
                    self.bump,
                    statements,
                    begin_l,
                    end_l,
                    expression_l,
                ))
            }
        }
    }

    pub(crate) fn begin(
        &self,
        begin_t: &'a Token<'a>,
        body: Maybe<&'a Node<'a>>,
        end_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let new_begin_l = loc(begin_t);
        let new_end_l = loc(end_t);
        let new_expression_l = new_begin_l.join(&new_end_l);

        let new_begin_l = Some(new_begin_l);
        let new_end_l = Some(new_end_l);

        let body: Maybe<&'a mut Node<'a>> = unsafe { std::mem::transmute(body) };
        match body {
            Some(body) => {
                match body {
                    Node::Mlhs(mlhs) => {
                        // Synthesized (begin) from compstmt "a; b" or (mlhs)
                        // from multi_lhs "(a, b) = *foo".
                        mlhs.set_begin_l(new_begin_l);
                        mlhs.set_end_l(new_end_l);
                        mlhs.set_expression_l(new_expression_l);
                        body
                    }
                    Node::Begin(begin) if begin.begin_l.is_none() && begin.end_l.is_none() => {
                        begin.set_begin_l(new_begin_l);
                        begin.set_end_l(new_end_l);
                        begin.set_expression_l(new_expression_l);
                        body
                    }
                    _ => {
                        let mut statements: Vec<'a, &'a Node> = bump_vec![in self.bump; ];
                        statements.push(body);
                        Node::new_begin(
                            self.bump,
                            statements,
                            new_begin_l,
                            new_end_l,
                            new_expression_l,
                        )
                    }
                }
            }
            None => {
                // A nil expression: `()'.
                Node::new_begin(
                    self.bump,
                    bump_vec![in self.bump; ],
                    new_begin_l,
                    new_end_l,
                    new_expression_l,
                )
            }
        }
    }

    pub(crate) fn begin_keyword(
        &self,
        begin_t: &'a Token<'a>,
        body: Maybe<&'a Node<'a>>,
        end_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let begin_l = loc(begin_t);
        let end_l = loc(end_t);
        let expression_l = begin_l.join(&end_l);

        let begin_l = Some(begin_l);
        let end_l = Some(end_l);

        if body.is_none() {
            // A nil expression: `begin end'.
            Node::new_kw_begin(
                self.bump,
                bump_vec![in self.bump; ],
                begin_l,
                end_l,
                expression_l,
            )
        } else {
            let body = body.unwrap();
            match body {
                Node::Begin(Begin { statements, .. }) => Node::new_kw_begin(
                    self.bump,
                    take_vec(statements),
                    begin_l,
                    end_l,
                    expression_l,
                ),
                _ => {
                    let mut statements = bump_vec![in self.bump; ];
                    statements.push(body);
                    Node::new_kw_begin(self.bump, statements, begin_l, end_l, expression_l)
                }
            }
            // if body.is_begin() {
            //     // Synthesized (begin) from compstmt "a; b".
            //     let internal::Begin { statements, .. } =
            //         body.into_begin(self.bump).into_internal(self.bump);
            //     Node::new_kw_begin(self.bump, *statements, begin_l, end_l, expression_l)
            // } else {
            //     let mut statements = bump_vec![in self.bump; ];
            //     statements.push(body);
            //     Node::new_kw_begin(self.bump, statements, begin_l, end_l, expression_l)
            // }
        }
    }

    //
    // Pattern matching
    //

    pub(crate) fn case_match(
        &self,
        case_t: &'a Token<'a>,
        expr: &'a Node<'a>,
        in_bodies: Vec<'a, &'a Node<'a>>,
        mut else_t: Maybe<&'a Token<'a>>,
        else_body: Maybe<&'a Node<'a>>,
        end_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let else_body = match (else_t.as_mut(), else_body.as_ref()) {
            (Some(else_t), None) => Some(Node::new_empty_else(self.bump, loc(&**else_t))),
            _ => else_body,
        };

        let keyword_l = loc(case_t);
        let else_l = maybe_loc(&else_t);
        let end_l = loc(end_t);
        let expression_l = loc(case_t).join(&end_l);

        Node::new_case_match(
            self.bump,
            expr,
            in_bodies,
            else_body,
            keyword_l,
            else_l,
            end_l,
            expression_l,
        )
    }

    pub(crate) fn match_pattern(
        &self,
        value: &'a Node<'a>,
        assoc_t: &'a Token<'a>,
        pattern: &'a Node<'a>,
    ) -> &'a Node<'a> {
        let operator_l = loc(assoc_t);
        let expression_l = join_exprs(value, pattern);

        Node::new_match_pattern(self.bump, value, pattern, operator_l, expression_l)
    }

    pub(crate) fn match_pattern_p(
        &self,
        value: &'a Node<'a>,
        in_t: &'a Token<'a>,
        pattern: &'a Node<'a>,
    ) -> &'a Node<'a> {
        let operator_l = loc(in_t);
        let expression_l = join_exprs(value, pattern);

        Node::new_match_pattern_p(self.bump, value, pattern, operator_l, expression_l)
    }

    pub(crate) fn in_pattern(
        &self,
        in_t: &'a Token<'a>,
        pattern: &'a Node<'a>,
        guard: Maybe<&'a Node<'a>>,
        then_t: &'a Token<'a>,
        body: Maybe<&'a Node<'a>>,
    ) -> &'a Node<'a> {
        let keyword_l = loc(in_t);
        let begin_l = loc(then_t);

        let mut expression_l = maybe_node_expr_mut(&body);
        if expression_l.is_none() {
            expression_l = maybe_node_expr_mut(&guard);
        }
        let expression_l = if expression_l.is_none() {
            pattern.expression().clone()
        } else {
            expression_l.unwrap()
        };
        let expression_l = expression_l.join(&keyword_l);

        Node::new_in_pattern(
            self.bump,
            pattern,
            guard,
            body,
            keyword_l,
            begin_l,
            expression_l,
        )
    }

    pub(crate) fn if_guard(&self, if_t: &'a Token<'a>, cond: &'a Node<'a>) -> &'a Node<'a> {
        let keyword_l = loc(if_t);
        let expression_l = keyword_l.join(cond.expression());

        Node::new_if_guard(self.bump, cond, keyword_l, expression_l)
    }
    pub(crate) fn unless_guard(&self, unless_t: &'a Token<'a>, cond: &'a Node<'a>) -> &'a Node<'a> {
        let keyword_l = loc(unless_t);
        let expression_l = keyword_l.join(cond.expression());

        Node::new_unless_guard(self.bump, cond, keyword_l, expression_l)
    }

    pub(crate) fn match_var(&self, name_t: &'a Token<'a>) -> Result<&'a Node<'a>, ()> {
        let name_l = loc(name_t);
        let expression_l = name_l;
        let name = value(name_t);

        self.check_lvar_name(name.as_str(), &name_l)?;
        self.check_duplicate_pattern_variable(name.as_str(), &name_l)?;
        self.static_env.declare(name.as_str());

        Ok(Node::new_match_var(self.bump, name, name_l, expression_l))
    }

    pub(crate) fn match_hash_var(&self, name_t: &'a Token<'a>) -> Result<&'a Node<'a>, ()> {
        let expression_l = loc(name_t);
        let name_l = expression_l.adjust_end(-1);

        let name = value(name_t);

        self.check_lvar_name(name.as_str(), &name_l)?;
        self.check_duplicate_pattern_variable(name.as_str(), &name_l)?;
        self.static_env.declare(name.as_str());

        Ok(Node::new_match_var(self.bump, name, name_l, expression_l))
    }
    pub(crate) fn match_hash_var_from_str(
        &self,
        begin_t: &'a Token<'a>,
        mut strings: Vec<'a, &'a Node<'a>>,
        end_t: &'a Token<'a>,
    ) -> Result<&'a Node<'a>, ()> {
        if strings.len() != 1 {
            self.error(
                DiagnosticMessage::new_symbol_literal_with_interpolation(),
                &loc(begin_t).join(&loc(end_t)),
            );
            return Err(());
        }

        let string = strings.remove(0);
        let result = match string {
            Node::Str(Str {
                value,
                begin_l,
                end_l,
                expression_l,
            }) => {
                let name = String::from_utf8_lossy_in(value.as_raw(), self.bump);
                let mut name_l = *expression_l;

                self.check_lvar_name(name.as_str(), &name_l)?;
                self.check_duplicate_pattern_variable(name.as_str(), &name_l)?;

                self.static_env.declare(name.as_str());

                if let Some(begin_l) = begin_l.as_ref() {
                    let begin_d: i32 = begin_l
                        .size()
                        .try_into()
                        .expect("failed to convert usize loc into i32, is it too big?");
                    name_l = name_l.adjust_begin(begin_d)
                }

                if let Some(end_l) = end_l.as_ref() {
                    let end_d: i32 = end_l
                        .size()
                        .try_into()
                        .expect("failed to convert usize loc into i32, is it too big?");
                    name_l = name_l.adjust_end(-end_d)
                }

                let expression_l = loc(begin_t).join(&expression_l).join(&loc(end_t));
                Node::new_match_var(self.bump, name, name_l, expression_l)
            }
            Node::Begin(Begin { statements, .. }) => {
                self.match_hash_var_from_str(begin_t, take_vec(statements), end_t)?
            }
            _ => {
                self.error(
                    DiagnosticMessage::new_symbol_literal_with_interpolation(),
                    &loc(begin_t).join(&loc(end_t)),
                );
                return Err(());
            }
        };

        Ok(result)
    }

    pub(crate) fn match_rest(
        &self,
        star_t: &'a Token<'a>,
        name_t: Maybe<&'a Token<'a>>,
    ) -> Result<&'a Node<'a>, ()> {
        let name = if name_t.is_none() {
            Maybe::none()
        } else {
            let t = name_t.unwrap();
            Some(self.match_var(t)?)
        };

        let operator_l = loc(star_t);
        let expression_l = operator_l.maybe_join(&maybe_node_expr_mut(&name));

        Ok(Node::new_match_rest(
            self.bump,
            name,
            operator_l,
            expression_l,
        ))
    }

    pub(crate) fn hash_pattern(
        &self,
        lbrace_t: Maybe<&'a Token<'a>>,
        kwargs: Vec<'a, &'a Node<'a>>,
        rbrace_t: Maybe<&'a Token<'a>>,
    ) -> &'a Node<'a> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = collection_map(&lbrace_t, &kwargs, &rbrace_t);

        Node::new_hash_pattern(self.bump, kwargs, begin_l, end_l, expression_l)
    }

    pub(crate) fn array_pattern(
        &self,
        lbrack_t: Maybe<&'a Token<'a>>,
        elements: Vec<'a, &'a Node<'a>>,
        trailing_comma: Maybe<&'a Token<'a>>,
        rbrack_t: Maybe<&'a Token<'a>>,
    ) -> &'a Node<'a> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = collection_map(&lbrack_t, &elements, &rbrack_t);

        let expression_l = expression_l.maybe_join(&maybe_loc(&trailing_comma));

        if elements.is_empty() {
            return Node::new_array_pattern(
                self.bump,
                bump_vec![in self.bump; ],
                begin_l,
                end_l,
                expression_l,
            );
        }

        if trailing_comma.is_some() {
            Node::new_array_pattern_with_tail(self.bump, elements, begin_l, end_l, expression_l)
        } else {
            Node::new_array_pattern(self.bump, elements, begin_l, end_l, expression_l)
        }
    }

    pub(crate) fn find_pattern(
        &self,
        lbrack_t: Maybe<&'a Token<'a>>,
        elements: Vec<'a, &'a Node<'a>>,
        rbrack_t: Maybe<&'a Token<'a>>,
    ) -> &'a Node<'a> {
        let CollectionMap {
            begin_l,
            end_l,
            expression_l,
        } = collection_map(&lbrack_t, &elements, &rbrack_t);

        Node::new_find_pattern(self.bump, elements, begin_l, end_l, expression_l)
    }

    pub(crate) fn const_pattern(
        &self,
        const_: &'a Node<'a>,
        ldelim_t: &'a Token<'a>,
        pattern: &'a Node<'a>,
        rdelim_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let begin_l = loc(ldelim_t);
        let end_l = loc(rdelim_t);
        let expression_l = const_.expression().join(&loc(rdelim_t));

        Node::new_const_pattern(self.bump, const_, pattern, begin_l, end_l, expression_l)
    }

    pub(crate) fn pin(&self, pin_t: &'a Token<'a>, var: &'a Node<'a>) -> &'a Node<'a> {
        let operator_l = loc(pin_t);
        let expression_l = var.expression().join(&operator_l);

        Node::new_pin(self.bump, var, operator_l, expression_l)
    }

    pub(crate) fn match_alt(
        &self,
        lhs: &'a Node<'a>,
        pipe_t: &'a Token<'a>,
        rhs: &'a Node<'a>,
    ) -> &'a Node<'a> {
        let operator_l = loc(pipe_t);
        let expression_l = join_exprs(lhs, rhs);

        Node::new_match_alt(self.bump, lhs, rhs, operator_l, expression_l)
    }

    pub(crate) fn match_as(
        &self,
        value: &'a Node<'a>,
        assoc_t: &'a Token<'a>,
        as_: &'a Node<'a>,
    ) -> &'a Node<'a> {
        let operator_l = loc(assoc_t);
        let expression_l = join_exprs(value, as_);

        Node::new_match_as(self.bump, value, as_, operator_l, expression_l)
    }

    pub(crate) fn match_nil_pattern(
        &self,
        dstar_t: &'a Token<'a>,
        nil_t: &'a Token<'a>,
    ) -> &'a Node<'a> {
        let operator_l = loc(dstar_t);
        let name_l = loc(nil_t);
        let expression_l = operator_l.join(&name_l);

        Node::new_match_nil_pattern(self.bump, operator_l, name_l, expression_l)
    }

    pub(crate) fn match_pair(
        &self,
        p_kw_label: PKwLabel<'a>,
        value: &'a Node<'a>,
    ) -> Result<&'a Node<'a>, ()> {
        let result = match p_kw_label {
            PKwLabel::PlainLabel(label_t) => {
                self.check_duplicate_pattern_key(clone_value(label_t).as_str(), &loc(label_t))?;
                self.pair_keyword(label_t, value)
            }
            PKwLabel::QuotedLabel((begin_t, parts, end_t)) => {
                let label_loc = loc(begin_t).join(&loc(end_t));

                match static_string(self.bump, &parts) {
                    Some(var_name) => self.check_duplicate_pattern_key(&var_name, &label_loc)?,
                    _ => {
                        self.error(
                            DiagnosticMessage::new_symbol_literal_with_interpolation(),
                            &label_loc,
                        );
                        return Err(());
                    }
                }

                self.pair_quoted(begin_t, parts, end_t, value)
            }
        };
        Ok(result)
    }

    pub(crate) fn match_label(&self, p_kw_label: PKwLabel<'a>) -> Result<&'a Node<'a>, ()> {
        match p_kw_label {
            PKwLabel::PlainLabel(label_t) => self.match_hash_var(label_t),
            PKwLabel::QuotedLabel((begin_t, parts, end_t)) => {
                self.match_hash_var_from_str(begin_t, parts, end_t)
            }
        }
    }

    //
    // Verification
    //

    pub(crate) fn check_condition(&self, cond: &'a Node<'a>) -> &'a Node<'a> {
        let cond = cond;

        match cond {
            Node::Begin(Begin {
                statements,
                begin_l,
                end_l,
                expression_l,
            }) => {
                if statements.len() == 1 {
                    let statements: &mut Vec<'a, &'a Node<'a>> =
                        unsafe { std::mem::transmute(statements) };
                    let stmt = statements.pop().unwrap();
                    let stmt = self.check_condition(stmt);
                    Node::new_begin(
                        self.bump,
                        bump_vec![in self.bump; stmt],
                        begin_l.clone(),
                        end_l.clone(),
                        expression_l.clone(),
                    )
                } else {
                    Node::new_begin(
                        self.bump,
                        take_vec(statements),
                        begin_l.clone(),
                        end_l.clone(),
                        expression_l.clone(),
                    )
                }
            }
            Node::And(And {
                lhs,
                rhs,
                operator_l,
                expression_l,
            }) => {
                let lhs = self.check_condition(&*lhs);
                let rhs = self.check_condition(&*rhs);
                Node::new_and(
                    self.bump,
                    lhs,
                    rhs,
                    operator_l.clone(),
                    expression_l.clone(),
                )
            }
            Node::Or(Or {
                lhs,
                rhs,
                operator_l,
                expression_l,
            }) => {
                let lhs = self.check_condition(&*lhs);
                let rhs = self.check_condition(&*rhs);
                Node::new_or(
                    self.bump,
                    lhs,
                    rhs,
                    operator_l.clone(),
                    expression_l.clone(),
                )
            }
            Node::Irange(Irange {
                left,
                right,
                operator_l,
                expression_l,
            }) => Node::new_i_flip_flop(
                self.bump,
                left.as_ref().map(|node| self.check_condition(&**node)),
                right.as_ref().map(|node| self.check_condition(&**node)),
                operator_l.clone(),
                expression_l.clone(),
            ),
            Node::Erange(Erange {
                left,
                right,
                operator_l,
                expression_l,
            }) => Node::new_e_flip_flop(
                self.bump,
                left.as_ref().map(|node| self.check_condition(&**node)),
                right.as_ref().map(|node| self.check_condition(&**node)),
                operator_l.clone(),
                expression_l.clone(),
            ),
            Node::Regexp(Regexp { expression_l, .. }) => {
                let expression_l = expression_l.clone();
                Node::new_match_current_line(self.bump, cond, expression_l)
            }
            _ => {
                let a = std::borrow::Cow::Borrowed(&Loc::new(1, 2));
                let b: std::borrow::Cow<'a, Loc> = std::borrow::Cow::Owned(Loc::new(1, 2));
                cond
            }
        }
    }

    pub(crate) fn check_assignment_to_numparam(&self, name: &str, loc: &Loc) -> Result<(), ()> {
        let assigning_to_numparam = self.context.is_in_dynamic_block()
            && matches!(
                name,
                "_1" | "_2" | "_3" | "_4" | "_5" | "_6" | "_7" | "_8" | "_9"
            )
            && self.max_numparam_stack.has_numparams();

        if assigning_to_numparam {
            self.error(
                DiagnosticMessage::new_cant_assign_to_numparam(String::from_str_in(
                    name, self.bump,
                )),
                loc,
            );
            return Err(());
        }
        Ok(())
    }

    pub(crate) fn check_reserved_for_numparam(&self, name: &str, loc: &Loc) -> Result<(), ()> {
        match name {
            "_1" | "_2" | "_3" | "_4" | "_5" | "_6" | "_7" | "_8" | "_9" => {
                self.error(
                    DiagnosticMessage::new_reserved_for_numparam(String::from_str_in(
                        name, self.bump,
                    )),
                    loc,
                );
                Err(())
            }
            _ => Ok(()),
        }
    }

    pub(crate) fn check_lvar_name(&self, name: &str, loc: &Loc) -> Result<(), ()> {
        let mut all_chars = name.chars();
        let first = all_chars
            .next()
            .expect("local variable name can't be empty");
        let mut rest = all_chars;

        if (first.is_lowercase() || first == '_') && rest.all(|c| c.is_alphanumeric() || c == '_') {
            Ok(())
        } else {
            self.error(
                DiagnosticMessage::new_key_must_be_valid_as_local_variable(),
                loc,
            );
            Err(())
        }
    }

    pub(crate) fn check_duplicate_pattern_variable(&self, name: &str, loc: &Loc) -> Result<(), ()> {
        if name.starts_with('_') {
            return Ok(());
        }

        if self.pattern_variables.is_declared(name) {
            self.error(DiagnosticMessage::new_duplicate_variable_name(), loc);
            return Err(());
        }

        self.pattern_variables.declare(name);
        Ok(())
    }

    pub(crate) fn check_duplicate_pattern_key(&self, name: &str, loc: &Loc) -> Result<(), ()> {
        if self.pattern_hash_keys.is_declared(name) {
            self.error(DiagnosticMessage::new_duplicate_key_name(), loc);
            return Err(());
        }

        self.pattern_hash_keys.declare(name);
        Ok(())
    }

    //
    // Helpers
    //

    #[cfg(feature = "onig")]
    pub(crate) fn build_static_regexp(
        &self,
        parts: &[&'a Node],
        options: &Maybe<String>,
        loc: &Loc,
    ) -> Option<Regex> {
        let source = static_string(self.bump, &parts)?;
        let mut reg_options = RegexOptions::REGEX_OPTION_NONE;
        reg_options |= RegexOptions::REGEX_OPTION_CAPTURE_GROUP;
        if let Some(options_s) = options.as_ref().map(|s| s.as_str()) {
            if options_s.as_bytes().contains(&b'x') {
                reg_options |= RegexOptions::REGEX_OPTION_EXTEND;
            }
        }

        let bytes = onig::EncodedBytes::ascii(source.as_bytes());

        match Regex::with_options_and_encoding(bytes, reg_options, onig::Syntax::ruby()) {
            Ok(regex) => Some(regex),
            Err(err) => {
                self.error(
                    DiagnosticMessage::new_regex_error(String::from(err.description())),
                    loc,
                );
                None
            }
        }
    }

    #[cfg(feature = "onig")]
    pub(crate) fn validate_static_regexp(
        &self,
        parts: &[&'a Node],
        options: &Maybe<String>,
        loc: &Loc,
    ) {
        self.build_static_regexp(parts, options, loc);
    }

    #[cfg(not(feature = "onig"))]
    pub(crate) fn validate_static_regexp(
        &self,
        _parts: &[&'a Node],
        _options: &Maybe<String>,
        _loc: &Loc,
    ) {
    }

    #[cfg(feature = "onig")]
    pub(crate) fn static_regexp_captures(&self, node: &Node) -> Option<Vec<String>> {
        if node.is_regexp() {
            let node = node.as_regexp().unwrap();
            let parts = node.get_parts();
            let options = node.get_options();
            let expression_l = node.get_expression_l();

            let mut re_options = &Maybe::none();
            if let Some(options) = options.as_ref() {
                if let Some(regopt) = options.as_reg_opt() {
                    re_options = regopt.get_options();
                }
            };
            let regex = self.build_static_regexp(parts, re_options, expression_l)?;

            let mut result: Vec<String> = vec![];

            regex.foreach_name(|name, _| {
                result.push(name.to_string());
                true
            });

            return Some(result);
        }
        None
    }

    #[cfg(not(feature = "onig"))]
    pub(crate) fn static_regexp_captures(&self, _node: &Node) -> Option<Vec<String>> {
        None
    }

    pub(crate) fn is_heredoc(&self, begin_t: &Maybe<&'a Token<'a>>) -> bool {
        if let Some(begin_t) = begin_t.as_ref() {
            if clone_value(begin_t).as_str().starts_with("<<") {
                return true;
            }
        }
        false
    }

    pub(crate) fn heredoc_map(
        &self,
        begin_t: &Maybe<&'a Token<'a>>,
        parts: &[&'a Node<'a>],
        end_t: &Maybe<&'a Token<'a>>,
    ) -> HeredocMap {
        let expression_l = begin_t
            .as_ref()
            .map(|t| loc(t))
            .expect("bug: begin_t must be Some");
        let heredoc_end_l = end_t
            .as_ref()
            .map(|t| loc(t))
            .expect("heredoc must have end_t");

        // let begin_t = begin_t.clone().expect("bug: begin_t must be Some");
        // let end_t = end_t.clone().expect("heredoc must have end_t");

        let heredoc_body_l = collection_expr(parts).unwrap_or_else(|| heredoc_end_l.clone());
        // let expression_l = loc(begin_t);
        // let heredoc_end_l = loc(end_t);

        HeredocMap {
            heredoc_body_l,
            heredoc_end_l,
            expression_l,
        }
    }

    pub(crate) fn error(&self, message: DiagnosticMessage<'a>, loc: &Loc) {
        self.diagnostics
            .emit(Diagnostic::new(ErrorLevel::error(), message, loc.clone()))
    }

    pub(crate) fn warn(&self, message: DiagnosticMessage<'a>, loc: &Loc) {
        self.diagnostics
            .emit(Diagnostic::new(ErrorLevel::warning(), message, loc.clone()))
    }

    pub(crate) fn value_expr(&self, node: &'a Node<'a>) -> Result<&'a Node<'a>, ()> {
        if let Some(void_node_loc) = self.void_value(node) {
            self.error(
                DiagnosticMessage::new_void_value_expression(),
                &void_node_loc,
            );
            Err(())
        } else {
            Ok(node)
        }
    }

    fn void_value_check_stmts(&self, statements: &[&Node]) -> Option<Loc> {
        match statements.last() {
            Some(last_stmt) => self.void_value(*last_stmt),
            None => None,
        }
    }

    fn void_value_check_condition(&self, if_true: &Node, if_false: &Node) -> Option<Loc> {
        if self.void_value(if_true).is_some() && self.void_value(if_false).is_some() {
            Some(if_true.expression().clone())
        } else {
            None
        }
    }

    fn void_value_check_maybe_condition(
        &self,
        if_true: &Maybe<&Node>,
        if_false: &Maybe<&Node>,
    ) -> Option<Loc> {
        match (if_true, if_false) {
            (None, None) | (None, Some(_)) | (Some(_), None) => None,
            (Some(if_true), Some(if_false)) => {
                self.void_value_check_condition(&*if_true, &*if_false)
            }
        }
    }

    fn void_value(&self, node: &Node) -> Option<Loc> {
        match node {
            Node::Return(_) | Node::Break(_) | Node::Next(_) | Node::Redo(_) | Node::Retry(_) => {
                Some(node.expression().clone())
            }
            Node::MatchPattern(MatchPattern { value, .. }) => self.void_value(value),
            Node::MatchPatternP(MatchPatternP { value, .. }) => self.void_value(value),
            Node::Begin(Begin { statements, .. }) => self.void_value_check_stmts(statements),
            Node::KwBegin(KwBegin { statements, .. }) => self.void_value_check_stmts(statements),
            Node::If(If {
                if_true, if_false, ..
            }) => self.void_value_check_maybe_condition(if_true, if_false),
            Node::IfMod(IfMod {
                if_true, if_false, ..
            }) => self.void_value_check_maybe_condition(if_true, if_false),
            Node::IfTernary(IfTernary {
                if_true, if_false, ..
            }) => self.void_value_check_condition(&**if_true, &**if_false),
            Node::And(And { lhs, .. }) => self.void_value(lhs),
            Node::Or(Or { lhs, .. }) => self.void_value(lhs),
            _ => None,
        }
    }

    fn rewrite_hash_args_to_kwargs<'b>(&self, args: &'b mut Vec<'a, &'a Node<'a>>) {
        let len = args.len();

        if args.is_empty() {
            return;
        }

        match args.get_mut(len - 1).unwrap() {
            Node::Hash(Hash {
                begin_l: None,
                end_l: None,
                pairs,
                expression_l,
            }) => {
                let kwargs = Node::new_kwargs(self.bump, take_vec(pairs), expression_l.clone());
                args.push(kwargs);
                return;
            }
            _ => {}
        }

        if len < 2 {
            return;
        }

        let last = args.pop().unwrap();
        let pre_last = args.pop().unwrap();

        match (&pre_last, last) {
            (
                Node::BlockPass(_),
                Node::Hash(Hash {
                    begin_l: None,
                    end_l: None,
                    pairs,
                    expression_l,
                }),
            ) => {
                let block_pass = pre_last;
                let kwargs = Node::new_kwargs(self.bump, take_vec(pairs), expression_l.clone());
                args.push(kwargs);
                args.push(block_pass);
            }
            _ => {}
        }

        // if !args.is_empty() && is_kwargs(args[len - 1]) {
        //     let internal::Hash {
        //         pairs,
        //         expression_l,
        //         ..
        //     } = args
        //         .pop()
        //         .unwrap()
        //         .into_hash(self.bump)
        //         .into_internal(self.bump);

        //     let kwargs = Node::new_kwargs(self.bump, *pairs, expression_l.clone());
        //     args.push(kwargs);
        // } else
        // if len > 1 && args[len - 1].is_block_pass() && is_kwargs(&args[len - 2]) {
        //     let block_pass = args.pop().unwrap();
        //     let internal::Hash {
        //         pairs,
        //         expression_l,
        //         ..
        //     } = args
        //         .pop()
        //         .unwrap()
        //         .into_hash(self.bump)
        //         .into_internal(self.bump);
        //     let kwargs = Node::new_kwargs(self.bump, *pairs, expression_l.clone());
        //     args.push(kwargs);
        //     args.push(block_pass);
        // }
    }
}

pub(crate) fn is_kwargs<'a>(node: &'a Node<'a>) -> bool {
    if let Some(hash) = node.as_hash() {
        hash.get_begin_l().is_none() && hash.get_end_l().is_none()
    } else {
        false
    }
}

pub(crate) fn loc(token: &Token) -> Loc {
    token.loc().clone()
}

pub(crate) fn maybe_loc(token: &Maybe<&Token>) -> Maybe<Loc> {
    match token.as_ref() {
        Some(token) => Some(loc(*token)),
        None => None,
    }
}

pub(crate) fn collection_map<'a>(
    begin_t: &Maybe<&Token>,
    parts: &[&'a Node<'a>],
    end_t: &Maybe<&Token>,
) -> CollectionMap {
    let begin_l = maybe_loc(begin_t);
    let end_l = maybe_loc(end_t);

    let expression_l = collection_expr(parts);
    let expression_l = join_maybe_locs(&expression_l, &begin_l);
    let expression_l = join_maybe_locs(&expression_l, &end_l);
    let expression_l = expression_l.unwrap_or_else(|| {
        unreachable!("empty collection without begin_t/end_t, can't build source map")
    });

    CollectionMap {
        begin_l,
        end_l,
        expression_l,
    }
}

pub(crate) fn maybe_node_expr_mut(node: &Option<&Node>) -> Maybe<Loc> {
    match node {
        Some(node) => Some(node.expression().clone()),
        None => None,
    }
}

pub(crate) fn maybe_node_expr(node: &Option<&Node>) -> Maybe<Loc> {
    match node {
        Some(node) => Some(node.expression().clone()),
        None => None,
    }
}

pub(crate) fn join_exprs(lhs: &Node, rhs: &Node) -> Loc {
    lhs.expression().join(rhs.expression())
}

pub(crate) fn join_maybe_locs(lhs: &Maybe<Loc>, rhs: &Maybe<Loc>) -> Maybe<Loc> {
    match (lhs.as_ref(), rhs.as_ref()) {
        (None, None) => None,
        (None, Some(rhs)) => Some(rhs.clone()),
        (Some(lhs), None) => Some(lhs.clone()),
        (Some(lhs), Some(rhs)) => Some(lhs.join(rhs)),
    }
}

pub(crate) fn join_maybe_exprs(lhs: &Option<&Node>, rhs: &Option<&Node>) -> Maybe<Loc> {
    join_maybe_locs(&maybe_node_expr(lhs), &maybe_node_expr(rhs))
}

pub(crate) fn collection_expr<'a>(nodes: &[&'a Node<'a>]) -> Maybe<Loc> {
    let first = nodes.first().map(|x| &**x);
    let last = nodes.last().map(|x| &**x);
    join_maybe_exprs(&first, &last)
}

pub(crate) fn value<'a>(token: &'a Token<'a>) -> String<'a> {
    token.into_string().unwrap()
}

pub(crate) fn lossy_value<'a>(token: &'a Token<'a>) -> String<'a> {
    token.to_string_lossy()
}

pub(crate) fn clone_value<'a>(token: &'a Token<'a>) -> String<'a> {
    token.to_string().unwrap().clone()
}

pub(crate) fn maybe_value<'a>(token: Maybe<&'a Token<'a>>) -> Maybe<String<'a>> {
    token.map(|t| value(t))
}

pub(crate) fn static_string<'a>(bump: &'a Bump, nodes: &[&'a Node<'a>]) -> Option<String<'a>> {
    let mut result = String::from_str_in("", bump);

    for node in nodes {
        match node {
            Node::Str(Str { value, .. }) => {
                let value = value.to_string_lossy();
                result.push_str(value.as_str())
            }
            Node::Begin(Begin { statements, .. }) => {
                if let Some(s) = static_string(bump, statements) {
                    result.push_str(&s)
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }

    Some(result)
}

pub(crate) struct CollectionMap {
    begin_l: Maybe<Loc>,
    end_l: Maybe<Loc>,
    expression_l: Loc,
}

pub(crate) struct HeredocMap {
    heredoc_body_l: Loc,
    heredoc_end_l: Loc,
    expression_l: Loc,
}

fn arg_name<'a>(node: &'a Node<'a>) -> Option<&'a str> {
    if let Some(arg) = node.as_arg() {
        Some(arg.get_name().as_str())
    } else if let Some(optarg) = node.as_optarg() {
        Some(optarg.get_name().as_str())
    } else if let Some(kwarg) = node.as_kwarg() {
        Some(kwarg.get_name().as_str())
    } else if let Some(kwoptarg) = node.as_kwoptarg() {
        Some(kwoptarg.get_name().as_str())
    } else if let Some(shadowarg) = node.as_shadowarg() {
        Some(shadowarg.get_name().as_str())
    } else if let Some(blockarg) = node.as_blockarg() {
        Some(blockarg.get_name().as_str())
    } else if let Some(restarg) = node.as_restarg() {
        restarg.get_name().as_ref().to_owned().map(|x| x.as_str())
    } else if let Some(kwrestarg) = node.as_kwrestarg() {
        kwrestarg.get_name().as_ref().to_owned().map(|x| x.as_str())
    } else {
        unreachable!("unsupported arg {:?}", node)
    }
}

fn arg_name_loc<'a>(node: &'a Node<'a>) -> &'a Loc {
    if let Some(arg) = node.as_arg() {
        arg.get_expression_l()
    } else if let Some(optarg) = node.as_optarg() {
        optarg.get_name_l()
    } else if let Some(kwarg) = node.as_kwarg() {
        kwarg.get_name_l()
    } else if let Some(kwoptarg) = node.as_kwoptarg() {
        kwoptarg.get_name_l()
    } else if let Some(shadowarg) = node.as_shadowarg() {
        shadowarg.get_expression_l()
    } else if let Some(blockarg) = node.as_blockarg() {
        blockarg.get_name_l()
    } else if let Some(restarg) = node.as_restarg() {
        let name_l = restarg.get_name_l();
        if name_l.is_some() {
            name_l.as_ref().unwrap()
        } else {
            restarg.get_expression_l()
        }
    } else if let Some(kwrestarg) = node.as_kwrestarg() {
        let name_l = kwrestarg.get_name_l();
        if name_l.is_some() {
            name_l.as_ref().unwrap()
        } else {
            kwrestarg.get_expression_l()
        }
    } else {
        unreachable!("unsupported arg {:?}", node)
    }
}

pub(crate) fn arg_name_collides(this_name: &str, that_name: &str) -> bool {
    &this_name[0..1] != "_" && this_name == that_name
}

pub(crate) fn check_duplicate_arg<'a>(
    this_arg: &'a Node<'a>,
    map: &mut HashMap<&'a str, &'a Node<'a>>,
) -> Result<(), &'a Loc> {
    let this_name = match arg_name(&*this_arg) {
        Some(name) => name,
        None => return Ok(()),
    };

    if !map.contains_key(this_name) {
        map.insert(this_name, this_arg);
        return Ok(());
    }

    let that_arg: Option<&&'a Node<'a>> = map.get(this_name);

    let that_arg = *that_arg.unwrap();
    let that_name = match arg_name(that_arg) {
        Some(name) => name,
        None => return Ok(()),
    };
    if arg_name_collides(this_name, that_name) {
        Err(arg_name_loc(this_arg))
    } else {
        Ok(())
    }
}

pub(crate) fn check_duplicate_args<'a>(
    args: &[&'a Node<'a>],
    map: &mut HashMap<&'a str, &'a Node<'a>>,
) -> Result<(), &'a Loc> {
    Ok(())
    // for i in 0..args.len() {
    //     match args[i] {
    //         Node::Arg(_)
    //         | Node::Optarg(_)
    //         | Node::Restarg(_)
    //         | Node::Kwarg(_)
    //         | Node::Kwoptarg(_)
    //         | Node::Kwrestarg(_)
    //         | Node::Shadowarg(_)
    //         | Node::Blockarg(_) => {
    //             self.check_duplicate_arg(&*args[i], map);
    //         }
    //         Node::Mlhs(Mlhs { items, .. }) => {
    //             self.check_duplicate_args(items, map);
    //         }
    //         Node::Procarg0(Procarg0 { args, .. }) => {
    //             self.check_duplicate_args(args, map);
    //         }
    //         Node::ForwardArg(_) | Node::Kwnilarg(_) => {
    //             // ignore
    //         }
    //         _ => {
    //             unreachable!("unsupported arg type {:?}", args[i])
    //         }
    //     }
    // }
}

fn take_vec<'a>(vec: &Vec<'a, &'a Node<'a>>) -> Vec<'a, &'a Node<'a>> {
    let vec: &mut Vec<'a, &'a Node<'a>> = unsafe { std::mem::transmute(vec) };
    vec.split_off(0)
}

fn take_str<'a>(s: &String<'a>) -> String<'a> {
    let s: &mut String<'a> = unsafe { std::mem::transmute(s) };
    s.split_off(0)
}

fn take_maybe_node<'a>(maybe_node: &Option<&'a Node<'a>>) -> Option<&'a Node<'a>> {
    let maybe_node: &mut Option<&'a Node<'a>> = unsafe { std::mem::transmute(maybe_node) };
    maybe_node.take()
}
