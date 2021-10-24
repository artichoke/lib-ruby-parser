%expect 0

%define api.parser.struct { Parser }
%define api.value.type { Value }
%define api.parser.check_debug { cfg!(feature = "debug-parser") }
%define api.parser.generic {<'a /*'*/>}

%define parse.error custom
%define parse.trace

%code parser_fields {
    result: Maybe<&'a /*'*/ Node<'a /*'*/>>,
    builder: Builder<'a /*'*/>,
    current_arg_stack: CurrentArgStack,
    /// Stack of sets of variables in current scopes.
    /// Each stack item represents locals in the scope.
    ///
    /// You can use it to pre-define some locals and parse
    /// your input as if these locals exist.
    ///
    /// For example, you can parse the following code
    ///
    /// ```text
    /// a = b + c
    /// ```
    ///
    /// as
    ///
    /// ```text
    /// Send(LocalVar(a), "+", LocalVar(b))
    /// ```
    ///
    /// by declaring `a` and `b` as locals using
    ///
    /// ```text
    /// parser.static_env.declare("a")
    /// parser.static_env.declare("b")
    /// parser.parse()
    /// ```
    pub static_env: StaticEnvironment,
    context: ParserContext,
    last_token_type: i32,
    max_numparam_stack: MaxNumparamStack,
    pattern_variables: VariablesStack,
    pattern_hash_keys: VariablesStack,
    tokens: Vec<'a /*'*/, &'a /*''*/ Token<'a /*'*/>>,
    diagnostics: Diagnostics<'a /*'*/>,
    token_rewriter: Maybe<TokenRewriter<'a /*'*/>>,
    record_tokens: bool,
}

%code use {

crate::use_native_or_external!(Ptr);
crate::use_native_or_external!(Maybe);
crate::use_native_or_external!(String);
crate::use_native_or_external!(Vec);

use crate::{ParserOptions, ParserResult};
use crate::{Token};
use crate::{Lexer, Builder, CurrentArgStack, StaticEnvironment, MaxNumparamStack, VariablesStack};
use crate::lex_states::*;
use crate::{Context as ParserContext, ContextItem};
use crate::builder::{LoopType, KeywordCmd, LogicalOp, PKwLabel, ArgsType};
use crate::builder::clone_value;
use crate::parse_value::ParseValue as Value;
use crate::parse_value::*;
use crate::Node;
use crate::{Diagnostic, DiagnosticMessage, ErrorLevel};
use crate::error::Diagnostics;
use crate::source::token_rewriter::TokenRewriter;
use crate::source::token_rewriter::TokenRewriterResult;
use crate::Loc;
use crate::parser_options::InternalParserOptions;

}

%code {
    // pre-code
}

/* Bison Declarations */
%token <token>
    kCLASS         "`class'"
    kMODULE        "`module'"
    kDEF           "`def'"
    kUNDEF         "`undef'"
    kBEGIN         "`begin'"
    kRESCUE        "`rescue'"
    kENSURE        "`ensure'"
    kEND           "`end'"
    kIF            "`if'"
    kUNLESS        "`unless'"
    kTHEN          "`then'"
    kELSIF         "`elsif'"
    kELSE          "`else'"
    kCASE          "`case'"
    kWHEN          "`when'"
    kWHILE         "`while'"
    kUNTIL         "`until'"
    kFOR           "`for'"
    kBREAK         "`break'"
    kNEXT          "`next'"
    kREDO          "`redo'"
    kRETRY         "`retry'"
    kIN            "`in'"
    kDO            "`do'"
    kDO_COND       "`do' for condition"
    kDO_BLOCK      "`do' for block"
    kDO_LAMBDA     "`do' for lambda"
    kRETURN        "`return'"
    kYIELD         "`yield'"
    kSUPER         "`super'"
    kSELF          "`self'"
    kNIL           "`nil'"
    kTRUE          "`true'"
    kFALSE         "`false'"
    kAND           "`and'"
    kOR            "`or'"
    kNOT           "`not'"
    kIF_MOD        "`if' modifier"
    kUNLESS_MOD    "`unless' modifier"
    kWHILE_MOD     "`while' modifier"
    kUNTIL_MOD     "`until' modifier"
    kRESCUE_MOD    "`rescue' modifier"
    kALIAS         "`alias'"
    kDEFINED       "`defined?'"
    klBEGIN        "`BEGIN'"
    klEND          "`END'"
    k__LINE__      "`__LINE__'"
    k__FILE__      "`__FILE__'"
    k__ENCODING__  "`__ENCODING__'"

%token <token>   tIDENTIFIER     "local variable or method"
%token <token>   tFID            "method"
%token <token>   tGVAR           "global variable"
%token <token>   tIVAR           "instance variable"
%token <token>   tCONSTANT       "constant"
%token <token>   tCVAR           "class variable"
%token <token>   tLABEL          "label"
%token <node> tINTEGER        "integer literal"
%token <node> tFLOAT          "float literal"
%token <node> tRATIONAL       "rational literal"
%token <node> tIMAGINARY      "imaginary literal"
%token <node> tCHAR           "char literal"
%token <node> tNTH_REF        "numbered reference"
%token <node> tBACK_REF       "back reference"
%token <node> tSTRING_CONTENT "literal content"
%token <num>  tREGEXP_END

%type <node> singleton strings string1 xstring regexp
%type <node> string_content
%type <node> words symbols qwords qsymbols
%type <node> literal numeric simple_numeric ssym dsym symbol cpath
%type <node> top_compstmt top_stmt
%type <node> stmt_or_begin stmt expr arg primary command command_call method_call
%type <node> expr_value arg_value primary_value rel_expr
%type <node> block_arg var_ref
%type <node> command_rhs arg_rhs
%type <node> command_asgn mrhs_arg block_call block_command
%type <node> f_block_opt
%type <node> f_arg_item f_marg f_rest_marg
%type <node> assoc backref string_dvar
%type <node> f_opt
%type <node> f_kw f_block_kw
%type <node> bvar
%type <node> lambda
%type <node> fitem
%type <node> p_top_expr_body
%type <node> p_expr p_as p_alt p_expr_basic
%type <node> p_arg
%type <node> p_value p_primitive p_variable p_var_ref p_const
%type <node> p_kw
%type <node> f_block_arg keyword_variable program
%type <node> var_lhs lhs mlhs_node mlhs mlhs_item mlhs_inner for_var

%type <node_list> assocs assoc_list opt_f_block_arg f_rest_arg f_optarg f_args
%type <node_list> f_block_optarg f_kwrest f_no_kwarg f_kwarg f_block_kwarg f_arg
%type <node_list> opt_args_tail args_tail
%type <node_list> regexp_contents xstring_contents string_contents
%type <node_list> qsym_list qword_list symbol_list word word_list
%type <node_list> string exc_list opt_rescue
%type <node_list> p_kwnorest p_kwrest p_any_kwrest p_kwarg p_kwargs p_args_post
%type <node_list> p_find p_args_tail
%type <node_list> case_args bv_decls opt_bv_decl
%type <node_list> block_param opt_block_args_tail block_args_tail f_any_kwrest f_margs f_marg_list mrhs
%type <node_list> args opt_block_arg command_args call_args opt_call_args aref_args
%type <node_list> undef_list mlhs_post mlhs_head stmts top_stmts mlhs_basic

%type <expr_value_do> expr_value_do
%type <superclass> superclass
%type <opt_ensure> opt_ensure
%type <opt_else> opt_else
%type <exc_var> exc_var
%type <if_tail> if_tail
%type <brace_body> brace_body
%type <cmd_brace_block> cmd_brace_block
%type <brace_block> brace_block
%type <do_block> do_block
%type <begin_block> begin_block
%type <lambda_body> lambda_body
%type <paren_args> paren_args
%type <opt_paren_args> opt_paren_args
%type <defn_head> defn_head
%type <defs_head> defs_head
%type <cases> cases
%type <case_body> case_body
%type <p_cases> p_cases
%type <p_case_body> p_case_body
%type <user_variable> user_variable
%type <do_body> do_body
%type <p_top_expr> p_top_expr

%type <maybe_node> compstmt bodystmt f_arglist f_paren_args opt_block_param
%type <maybe_node> block_param_def f_larglist f_opt_paren_args

%type <token>   sym operation operation2 operation3 kwrest_mark restarg_mark blkarg_mark
%type <token>   cname fname op f_norm_arg f_bad_arg
%type <token>   f_label f_arg_asgn call_op call_op2 reswords relop dot_or_colon
%type <token>   p_rest p_kw_label
%type <token>   args_forward excessed_comma def_name k_if k_elsif
%type <token>   rbrace rparen rbracket p_lparen p_lbracket k_return then term fcall
%type <token>   k_begin k_unless k_while k_until k_case k_for k_class k_module k_def k_do k_do_block
%type <token>   k_rescue k_ensure k_when k_else k_end do

%type <token_list> terms

%type <match_pattern_with_trailing_comma> p_args_head p_args

%type <none> none opt_terms trailer opt_nl

%token END_OF_INPUT 0   "end-of-input"
%token <token> tDOT
/* escaped chars, should be ignored otherwise */
%token <token> tBACKSLASH       "backslash"
%token tSP                      "escaped space"
%token <token> tSLASH_T         "escaped horizontal tab"
%token <token> tSLASH_F         "escaped form feed"
%token <token> tSLASH_R         "escaped carriage return"
%token <token> tVTAB            "escaped vertical tab"
%token <token> tUPLUS           "unary+"
%token <token> tUMINUS          "unary-"
%token <token> tPOW             "**"
%token <token> tCMP             "<=>"
%token <token> tEQ              "=="
%token <token> tEQQ             "==="
%token <token> tNEQ             "!="
%token <token> tGEQ             ">="
%token <token> tLEQ             "<="
%token <token> tANDOP           "&&"
%token <token> tOROP            "||"
%token <token> tMATCH           "=~"
%token <token> tNMATCH          "!~"
%token <token> tDOT2            ".."
%token <token> tDOT3            "..."
%token <token> tBDOT2           "(.."
%token <token> tBDOT3           "(..."
%token <token> tAREF            "[]"
%token <token> tASET            "[]="
%token <token> tLSHFT           "<<"
%token <token> tRSHFT           ">>"
%token <token> tANDDOT          "&."
%token <token> tCOLON2          "::"
%token <token> tCOLON3          ":: at EXPR_BEG"
%token <token> tOP_ASGN         "operator-assignment" /* +=, -=  etc. */
%token <token> tASSOC           "=>"
%token <token> tLPAREN          "("
%token <token> tLPAREN_ARG      "( arg"
%token <token> tRPAREN          ")"
%token <token> tLBRACK          "["
%token <token> tLBRACE          "{"
%token <token> tLBRACE_ARG      "{ arg"
%token <token> tSTAR            "*"
%token <token> tDSTAR           "**arg"
%token <token> tAMPER           "&"
%token <token> tLAMBDA          "->"
%token <token> tSYMBEG          "symbol literal"
%token <token> tSTRING_BEG      "string begin"
%token <token> tXSTRING_BEG     "backtick literal"
%token <token> tREGEXP_BEG      "regexp literal"
%token <token> tWORDS_BEG       "word list"
%token <token> tQWORDS_BEG      "verbatim word list"
%token <token> tSYMBOLS_BEG     "symbol list"
%token <token> tQSYMBOLS_BEG    "verbatim symbol list"
%token <token> tSTRING_END      "string end"
%token <token> tSTRING_DEND     "tRCURLY"
%token <token> tSTRING_DBEG
%token <token> tSTRING_DVAR
%token <token> tLAMBEG
%token <token> tLABEL_END

%token <token> tCOMMA           ","
%token <token> tLCURLY          "{ (tLCURLY)"
%token <token> tRCURLY          "}"
%token <token> tLBRACK2         "[ (tLBRACK2)"
%token <token> tEQL             "="
%token <token> tPIPE            "|"
%token <token> tAMPER2          "& (tAMPER2)"
%token <token> tGT              ">"
%token <token> tLT              "<"
%token <token> tBACK_REF2       "`"
%token <token> tCARET           "^"
%token <token> tLPAREN2         "( (tLPAREN2)"
%token <token> tRBRACK          "]"
%token <token> tSEMI            ";"
%token <token> tSPACE            " "
%token <token> tNL              "\n"
%token <token> tPLUS            "+"
%token <token> tMINUS           "-"
%token <token> tSTAR2           "* (tSTAR2)"
%token <token> tDIVIDE          "/"
%token <token> tPERCENT         "%"
%token <token> tTILDE           "~"
%token <token> tBANG            "!"

/*
 *	precedence table
 */

%nonassoc tLOWEST
%nonassoc tLBRACE_ARG

%nonassoc  kIF_MOD kUNLESS_MOD kWHILE_MOD kUNTIL_MOD kIN
%left  kOR kAND
%right kNOT
%nonassoc kDEFINED
%right tEQL tOP_ASGN
%left kRESCUE_MOD
%right tEH tCOLON
%nonassoc tDOT2 tDOT3 tBDOT2 tBDOT3
%left  tOROP
%left  tANDOP
%nonassoc  tCMP tEQ tEQQ tNEQ tMATCH tNMATCH
%left  tGT tGEQ tLT tLEQ
%left  tPIPE tCARET
%left  tAMPER2
%left  tLSHFT tRSHFT
%left  tPLUS tMINUS
%left  tSTAR2 tDIVIDE tPERCENT
%right tUMINUS_NUM tUMINUS
%right tPOW
%right tBANG tTILDE tUPLUS

%token tLAST_TOKEN

/* Grammar follows */
%%

         program:   {
                        self.yylexer.lex_state.set(EXPR_BEG);
                        self.current_arg_stack.push(None);
                        self.max_numparam_stack.push();

                        $<None>$ = Value::new_none();
                    }
                  top_compstmt
                    {
                        let top_compstmt = $<MaybeNode>2;
                        self.result = match top_compstmt {
                            Some(node) => Maybe::some(node),
                            None => Maybe::none(),
                        };
                        $$ = Value::new_none();

                        self.current_arg_stack.pop();
                        self.max_numparam_stack.pop();
                    }
                ;

    top_compstmt: top_stmts opt_terms
                    {
                        // TODO: run void_stmts
                        $$ = Value::new_maybe_node(
                            self.builder.compstmt($<NodeList>1)
                        );
                    }
                ;

       top_stmts: none
                    {
                        $$ = Value::new_node_list(
                          bump_vec![in self.bump;]
                        );
                    }
                | top_stmt
                    {
                        $$ = Value::new_node_list(
                            bump_vec![in self.bump; $<Node>1 ]
                        );
                    }
                | top_stmts terms top_stmt
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::new_node_list( nodes );
                    }
                | error top_stmt
                    {
                        $$ = Value::new_node_list(
                            bump_vec![in self.bump; $<Node>2 ]
                        );
                    }
                ;

        top_stmt: stmt
                    {
                        $$ = $1;
                    }
                | klBEGIN begin_block
                    {
                        let BeginBlock { begin_t, body, end_t } = *$<BeginBlock>2;
                        $$ = Value::new_node(
                            self.builder.preexe(
                                $<Token>1,
                                begin_t,
                                body,
                                end_t
                            )
                        );
                    }
                ;

     begin_block: tLCURLY top_compstmt tRCURLY
                    {
                        $$ = Value::new_begin_block(
                            self.bump.alloc(
                                BeginBlock {
                                    begin_t: $<Token>1,
                                    body: $<MaybeBoxedNode>2,
                                    end_t: $<Token>3
                                }
                            )
                        );
                    }
                ;

        bodystmt: compstmt opt_rescue
                  k_else
                  compstmt
                  opt_ensure
                    {
                        let compound_stmt = $<MaybeBoxedNode>1;
                        let rescue_bodies = $<NodeList>2;
                        if rescue_bodies.is_empty() {
                            return self.yyerror(@3, DiagnosticMessage::new_else_without_rescue());
                        }

                        let else_ = Some(( $<Token>3, $<MaybeBoxedNode>4 ));
                        let ensure = $<OptEnsure>5.map(|ensure| (ensure.ensure_t, ensure.body));

                        $$ = Value::new_maybe_node(
                            self.builder.begin_body(
                                compound_stmt,
                                rescue_bodies,
                                else_,
                                ensure
                            )
                        );
                    }
                | compstmt
                  opt_rescue
                  opt_ensure
                    {
                        let compound_stmt = $<MaybeBoxedNode>1;
                        let rescue_bodies = $<NodeList>2;
                        let ensure = $<OptEnsure>3.map(|ensure| (ensure.ensure_t, ensure.body));

                        $$ = Value::new_maybe_node(
                            self.builder.begin_body(
                                compound_stmt,
                                rescue_bodies,
                                None,
                                ensure
                            )
                        );
                    }
                ;

        compstmt: stmts opt_terms
                    {
                        // TODO: run void_stmts
                        $$ = Value::new_maybe_node(
                            self.builder.compstmt($<NodeList>1)
                        );
                    }
                ;

           stmts: none
                    {
                        $$ = Value::new_node_list(
                            bump_vec![in self.bump;]
                        );
                    }
                | stmt_or_begin
                    {
                        $$ = Value::new_node_list(
                            bump_vec![in self.bump; $<Node>1 ]
                        );
                    }
                | stmts terms stmt_or_begin
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::new_node_list(nodes);
                    }
                | error
                    {
                        $$ = Value::new_node_list(
                            bump_vec![in self.bump;]
                        );
                    }
                ;

   stmt_or_begin: stmt
                    {
                        $$ = $1;
                    }
                | klBEGIN
                    {
                        return self.yyerror(@1, DiagnosticMessage::new_begin_not_at_top_level());
                    }
                  begin_block
                    {
                        $$ = Value::new_none();
                    }
                ;

            stmt: kALIAS fitem
                    {
                        self.yylexer.lex_state.set(EXPR_FNAME|EXPR_FITEM);
                        $<None>$ = Value::new_none();
                    }
                  fitem
                    {
                        $$ = Value::new_node(
                            self.builder.alias($<Token>1, $<BoxedNode>2, $<BoxedNode>4)
                        );
                    }
                | kALIAS tGVAR tGVAR
                    {
                        $$ = Value::new_node(
                            self.builder.alias(
                                $<Token>1,
                                self.builder.gvar($<Token>2),
                                self.builder.gvar($<Token>3),
                            )
                        )
                    }
                | kALIAS tGVAR tBACK_REF
                    {
                        $$ = Value::new_node(
                            self.builder.alias(
                                $<Token>1,
                                self.builder.gvar($<Token>2),
                                self.builder.back_ref($<Token>3),
                            )
                        )
                    }
                | kALIAS tGVAR tNTH_REF
                    {
                        return self.yyerror(@3, DiagnosticMessage::new_alias_nth_ref());
                    }
                | kUNDEF undef_list
                    {
                        $$ = Value::new_node(
                            self.builder.undef_method(
                                $<Token>1,
                                $<NodeList>2
                            )
                        )
                    }
                | stmt kIF_MOD expr_value
                    {
                        $$ = Value::new_node(
                            self.builder.condition_mod(
                                Maybe::some($<BoxedNode>1),
                                Maybe::none(),
                                $<Token>2,
                                $<BoxedNode>3,
                            )
                        );
                    }
                | stmt kUNLESS_MOD expr_value
                    {
                        $$ = Value::new_node(
                            self.builder.condition_mod(
                                Maybe::none(),
                                Maybe::some($<BoxedNode>1),
                                $<Token>2,
                                $<BoxedNode>3,
                            )
                        );
                    }
                | stmt kWHILE_MOD expr_value
                    {
                        $$ = Value::new_node(
                            self.builder.loop_mod(
                                LoopType::While,
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>3,
                            )
                        );
                    }
                | stmt kUNTIL_MOD expr_value
                    {
                        $$ = Value::new_node(
                            self.builder.loop_mod(
                                LoopType::Until,
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>3,
                            )
                        );
                    }
                | stmt kRESCUE_MOD stmt
                    {
                        let rescue_body = self.builder.rescue_body(
                            $<Token>2,
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::some($<BoxedNode>3)
                        );

                        $$ = Value::new_node(
                            self.builder.begin_body(
                                Maybe::some($<BoxedNode>1),
                                bump_vec![in self.bump;rescue_body],
                                None,
                                None,
                            ).expect("expected begin_body to return Maybe::some (compound_stmt was given)")
                        );
                    }
                | klEND tLCURLY compstmt tRCURLY
                    {
                        if self.context.is_in_def() {
                            self.warn(@1, DiagnosticMessage::new_end_in_method());
                        }

                        $$ = Value::new_node(
                            self.builder.postexe(
                                $<Token>1,
                                $<Token>2,
                                $<MaybeBoxedNode>3,
                                $<Token>4,
                            )
                        );
                    }
                | command_asgn
                    {
                        $$ = $1;
                    }
                | mlhs tEQL command_call
                    {
                        let mut command_call = $<BoxedNode>3;
                        command_call = self.value_expr(command_call)?;

                        $$ = Value::new_node(
                            self.builder.multi_assign(
                                $<BoxedNode>1,
                                $<Token>2,
                                command_call
                            )
                        );
                    }
                | lhs tEQL mrhs
                    {
                        let mrhs = self.builder.array(
                            Maybe::none(),
                            $<NodeList>3,
                            Maybe::none()
                        );
                        let mrhs = self.value_expr(mrhs)?;

                        $$ = Value::new_node(
                            self.builder.assign(
                                $<BoxedNode>1,
                                $<Token>2,
                                mrhs
                            )
                        );
                    }
                | mlhs tEQL mrhs_arg kRESCUE_MOD stmt
                    {
                        let rescue_body = self.builder.rescue_body(
                            $<Token>4,
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::some($<BoxedNode>5)
                        );

                        let mrhs_arg = $<BoxedNode>3;
                        let mrhs_arg = self.value_expr(mrhs_arg)?;

                        let begin_body = self.builder.begin_body(
                            Maybe::some(mrhs_arg),
                            bump_vec![in self.bump; rescue_body ],
                            None,
                            None,
                        ).expect("expected begin_body to return Maybe::some (compound_stmt was given)");

                        $$ = Value::new_node(
                            self.builder.multi_assign(
                                $<BoxedNode>1,
                                $<Token>2,
                                begin_body
                            )
                        );
                    }
                | mlhs tEQL mrhs_arg
                    {
                        $$ = Value::new_node(
                            self.builder.multi_assign(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>3
                            )
                        );
                    }
                | expr
                    {
                        $$ = $1;
                    }
                ;

    command_asgn: lhs tEQL command_rhs
                    {
                        $$ = Value::new_node(
                            self.builder.assign(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>3
                            )
                        );
                    }
                | var_lhs tOP_ASGN command_rhs
                    {
                        $$ = Value::new_node(
                            self.builder.op_assign(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>3
                            )?
                        );
                    }
                | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN command_rhs
                    {
                        $$ = Value::new_node(
                            self.builder.op_assign(
                                self.builder.index(
                                    $<BoxedNode>1,
                                    $<Token>2,
                                    $<NodeList>3,
                                    $<Token>4
                                ),
                                $<Token>5,
                                $<BoxedNode>6
                            )?
                        );
                    }
                | primary_value call_op tIDENTIFIER tOP_ASGN command_rhs
                    {
                        $$ = Value::new_node(
                            self.builder.op_assign(
                                self.builder.call_method(
                                    Maybe::some($<BoxedNode>1),
                                    Maybe::some($<Token>2),
                                    Maybe::some($<Token>3),
                                    Maybe::none(),
                                    bump_vec![in self.bump;],
                                    Maybe::none()
                                ),
                                $<Token>4,
                                $<BoxedNode>5
                            )?
                        );
                    }
                | primary_value call_op tCONSTANT tOP_ASGN command_rhs
                    {
                        $$ = Value::new_node(
                            self.builder.op_assign(
                                self.builder.call_method(
                                    Maybe::some($<BoxedNode>1),
                                    Maybe::some($<Token>2),
                                    Maybe::some($<Token>3),
                                    Maybe::none(),
                                    bump_vec![in self.bump;],
                                    Maybe::none()
                                ),
                                $<Token>4,
                                $<BoxedNode>5
                            )?
                        );
                    }
                | primary_value tCOLON2 tCONSTANT tOP_ASGN command_rhs
                    {
                        let const_ = self.builder.const_op_assignable(
                            self.builder.const_fetch(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<Token>3
                            )
                        );
                        $$ = Value::new_node(
                            self.builder.op_assign(
                                const_,
                                $<Token>4,
                                $<BoxedNode>5
                            )?
                        );
                    }
                | primary_value tCOLON2 tIDENTIFIER tOP_ASGN command_rhs
                    {
                        $$ = Value::new_node(
                            self.builder.op_assign(
                                self.builder.call_method(
                                    Maybe::some($<BoxedNode>1),
                                    Maybe::some($<Token>2),
                                    Maybe::some($<Token>3),
                                    Maybe::none(),
                                    bump_vec![in self.bump;],
                                    Maybe::none()
                                ),
                                $<Token>4,
                                $<BoxedNode>5
                            )?
                        );
                    }
                | backref tOP_ASGN command_rhs
                    {
                        $$ = Value::new_node(
                            self.builder.op_assign(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>3
                            )?
                        );
                    }
                ;

     command_rhs: command_call   %prec tOP_ASGN
                    {
                        let command_call = $<BoxedNode>1;
                        let command_call = self.value_expr(command_call)?;
                        $$ = Value::new_node(command_call);
                    }
                | command_call kRESCUE_MOD stmt
                    {
                        let command_call = $<BoxedNode>1;
                        let command_call = self.value_expr(command_call)?;

                        let rescue_body = self.builder.rescue_body(
                            $<Token>2,
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::some($<BoxedNode>3)
                        );

                        $$ = Value::new_node(
                            self.builder.begin_body(
                                Maybe::some(command_call),
                                bump_vec![in self.bump; rescue_body ],
                                None,
                                None,
                            ).expect("expected begin_body to return Maybe::some (compound_stmt was given)")
                        );
                    }
                | command_asgn
                    {
                        $$ = $1;
                    }
                ;

            expr: command_call
                    {
                        $$ = $1;
                    }
                | expr kAND expr
                    {
                        $$ = Value::new_node(
                            self.builder.logical_op(
                                LogicalOp::And,
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>3
                            )?
                        );
                    }
                | expr kOR expr
                    {
                        $$ = Value::new_node(
                            self.builder.logical_op(
                                LogicalOp::Or,
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>3
                            )?
                        );
                    }
                | kNOT opt_nl expr
                    {
                        $$ = Value::new_node(
                            self.builder.not_op(
                                $<Token>1,
                                Maybe::none(),
                                Maybe::some($<BoxedNode>3),
                                Maybe::none()
                            )?
                        );
                    }
                | tBANG command_call
                    {
                        $$ = Value::new_node(
                            self.builder.not_op(
                                $<Token>1,
                                Maybe::none(),
                                Maybe::some($<BoxedNode>2),
                                Maybe::none()
                            )?
                        );
                    }
                | arg tASSOC
                    {
                        // let arg = match yystack.borrow_value_at(1) {
                        //     Value::Node(node) => node,
                        //     other => unreachable!("expected Node, got {:?}", other)
                        // };
                        // let arg = self.value_expr(arg)?;

                        self.yylexer.lex_state.set(EXPR_BEG|EXPR_LABEL);
                        self.yylexer.command_start = false;
                        self.pattern_variables.push();

                        $<Bool>$ = Value::new_bool(self.yylexer.in_kwarg);
                        self.yylexer.in_kwarg = true;
                    }
                  p_expr
                    {
                        self.pattern_variables.pop();
                        self.yylexer.in_kwarg = $<Bool>3;

                        $$ = Value::new_node(
                            self.builder.match_pattern(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>4
                            )
                        );
                    }
                | arg kIN
                    {
                        // let arg = match yystack.borrow_value_at(1) {
                        //     Value::Node(node) => node,
                        //     other => unreachable!("expected Node, got {:?}", other)
                        // };
                        // let arg = self.value_expr(arg)?;

                        self.yylexer.lex_state.set(EXPR_BEG|EXPR_LABEL);
                        self.yylexer.command_start = false;
                        self.pattern_variables.push();

                        $<Bool>$ = Value::new_bool(self.yylexer.in_kwarg);
                        self.yylexer.in_kwarg = true;
                    }
                  p_expr
                    {
                        self.pattern_variables.pop();
                        self.yylexer.in_kwarg = $<Bool>3;

                        $$ = Value::new_node(
                            self.builder.match_pattern_p(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>4
                            )
                        );
                    }
                | arg %prec tLBRACE_ARG
                    {
                        $$ = $1;
                    }
                ;

        def_name: fname
                    {
                        self.static_env.extend_static();
                        self.yylexer.cmdarg.push(false);
                        self.yylexer.cond.push(false);
                        self.current_arg_stack.push(None);

                        $$ = $1;
                    }
                ;

       defn_head: k_def def_name
                    {
                        self.context.push_def();

                        $$ = Value::new_defn_head(
                            self.bump.alloc(
                                DefnHead {
                                    def_t: $<Token>1,
                                    name_t: $<Token>2
                                }
                            )
                        );
                    }
                ;

       defs_head: k_def singleton dot_or_colon
                    {
                        self.yylexer.lex_state.set(EXPR_FNAME);
                        $<None>$ = Value::new_none();
                    }
                  def_name
                    {
                        self.yylexer.lex_state.set(EXPR_ENDFN|EXPR_LABEL);
                        self.context.push_defs();

                        $$ = Value::new_defs_head(
                            self.bump.alloc(
                                DefsHead {
                                    def_t: $<Token>1,
                                    definee: $<BoxedNode>2,
                                    dot_t: $<Token>3,
                                    name_t: $<Token>5
                                }
                            )
                        );
                    }
                ;

      expr_value: expr
                    {
                        let expr = $<BoxedNode>1;
                        let expr = self.value_expr(expr)?;
                        $$ = Value::new_node(expr);
                    }
                ;

   expr_value_do:   {
                        self.yylexer.cond.push(true);
                        $<None>$ = Value::new_none();
                    }
                  expr_value do
                    {
                        self.yylexer.cond.pop();

                        $$ = Value::new_expr_value_do(
                            self.bump.alloc(
                                ExprValueDo {
                                    value: $<BoxedNode>2,
                                    do_t: $<Token>3
                                }
                            )
                        );
                    }
                ;


    command_call: command
                    {
                        $$ = $1;
                    }
                | block_command
                    {
                        $$ = $1;
                    }
                ;

   block_command: block_call
                    {
                        $$ = $1;
                    }
                | block_call call_op2 operation2 command_args
                    {
                        $$ = Value::new_node(
                            self.builder.call_method(
                                Maybe::some($<BoxedNode>1),
                                Maybe::some($<Token>2),
                                Maybe::some($<Token>3),
                                Maybe::none(),
                                $<NodeList>4,
                                Maybe::none()
                            )
                        );
                    }
                ;

 cmd_brace_block: tLBRACE_ARG
                    {
                        self.context.push_block();
                        $<None>$ = Value::new_none();
                    }
                  brace_body tRCURLY
                    {
                        self.context.pop();
                        let BraceBody { args_type, body } = *$<BraceBody>3;
                        $$ = Value::new_cmd_brace_block(
                            self.bump.alloc(
                                CmdBraceBlock {
                                    begin_t: $<Token>1,
                                    args_type: args_type,
                                    body: body,
                                    end_t: $<Token>4
                                }
                            )
                        );
                    }
                ;

           fcall: operation
                    {
                        $$ = $1;
                    }
                ;

         command: fcall command_args       %prec tLOWEST
                    {
                        $$ = Value::new_node(
                            self.builder.call_method(
                                Maybe::none(),
                                Maybe::none(),
                                Maybe::some($<Token>1),
                                Maybe::none(),
                                $<NodeList>2,
                                Maybe::none()
                            )
                        );
                    }
                | fcall command_args cmd_brace_block
                    {
                        let method_call = self.builder.call_method(
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::some($<Token>1),
                            Maybe::none(),
                            $<NodeList>2,
                            Maybe::none()
                        );
                        let CmdBraceBlock { begin_t, args_type, body, end_t } = $<CmdBraceBlock>3;

                        $$ = Value::new_node(
                            self.builder.block(
                                method_call,
                                begin_t,
                                *args_type,
                                take_maybe_node(body),
                                end_t
                            )?
                        );
                    }
                | primary_value call_op operation2 command_args %prec tLOWEST
                    {
                        $$ = Value::new_node(
                            self.builder.call_method(
                                Maybe::some($<BoxedNode>1),
                                Maybe::some($<Token>2),
                                Maybe::some($<Token>3),
                                Maybe::none(),
                                $<NodeList>4,
                                Maybe::none()
                            )
                        );
                    }
                | primary_value call_op operation2 command_args cmd_brace_block
                    {
                        let method_call = self.builder.call_method(
                            Maybe::some($<BoxedNode>1),
                            Maybe::some($<Token>2),
                            Maybe::some($<Token>3),
                            Maybe::none(),
                            $<NodeList>4,
                            Maybe::none()
                        );
                        let CmdBraceBlock { begin_t, args_type, body, end_t } = $<CmdBraceBlock>5;

                        $$ = Value::new_node(
                            self.builder.block(
                                method_call,
                                begin_t,
                                *args_type,
                                take_maybe_node(body),
                                end_t
                            )?
                        );
                    }
                | primary_value tCOLON2 operation2 command_args %prec tLOWEST
                    {
                        $$ = Value::new_node(
                            self.builder.call_method(
                                Maybe::some($<BoxedNode>1),
                                Maybe::some($<Token>2),
                                Maybe::some($<Token>3),
                                Maybe::none(),
                                $<NodeList>4,
                                Maybe::none()
                            )
                        );
                    }
                | primary_value tCOLON2 operation2 command_args cmd_brace_block
                    {
                        let method_call = self.builder.call_method(
                            Maybe::some($<BoxedNode>1),
                            Maybe::some($<Token>2),
                            Maybe::some($<Token>3),
                            Maybe::none(),
                            $<NodeList>4,
                            Maybe::none()
                        );
                        let CmdBraceBlock { begin_t, args_type, body, end_t } = $<CmdBraceBlock>5;

                        $$ = Value::new_node(
                            self.builder.block(
                                method_call,
                                begin_t,
                                *args_type,
                                take_maybe_node(body),
                                end_t
                            )?
                        );
                    }
                | kSUPER command_args
                    {
                        $$ = Value::new_node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Super,
                                $<Token>1,
                                Maybe::none(),
                                $<NodeList>2,
                                Maybe::none()
                            )?
                        );
                    }
                | kYIELD command_args
                    {
                        $$ = Value::new_node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Yield,
                                $<Token>1,
                                Maybe::none(),
                                $<NodeList>2,
                                Maybe::none()
                            )?
                        );
                    }
                | k_return call_args
                    {
                        $$ = Value::new_node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Return,
                                $<Token>1,
                                Maybe::none(),
                                $<NodeList>2,
                                Maybe::none()
                            )?
                        );
                    }
                | kBREAK call_args
                    {
                        $$ = Value::new_node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Break,
                                $<Token>1,
                                Maybe::none(),
                                $<NodeList>2,
                                Maybe::none()
                            )?
                        );
                    }
                | kNEXT call_args
                    {
                        $$ = Value::new_node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Next,
                                $<Token>1,
                                Maybe::none(),
                                $<NodeList>2,
                                Maybe::none()
                            )?
                        );
                    }
                ;

            mlhs: mlhs_basic
                    {
                        $$ = Value::new_node(
                            self.builder.multi_lhs(
                                Maybe::none(),
                                $<NodeList>1,
                                Maybe::none()
                            )
                        );
                    }
                | tLPAREN mlhs_inner rparen
                    {
                        $$ = Value::new_node(
                            self.builder.begin(
                                $<Token>1,
                                Maybe::some($<BoxedNode>2),
                                $<Token>3
                            )
                        );
                    }
                ;

      mlhs_inner: mlhs_basic
                    {
                        $$ = Value::new_node(
                            self.builder.multi_lhs(
                                Maybe::none(),
                                $<NodeList>1,
                                Maybe::none()
                            )
                        );
                    }
                | tLPAREN mlhs_inner rparen
                    {
                        let mlhs_inner = $<Node>2;
                        let mlhs_items = match mlhs_inner {
                            Node::Mlhs(mlhs) => {
                                let items: &mut Vec<'a /*'*/, &'a /*'*/ Node<'a /*'*/>> = unsafe { std::mem::transmute(&mlhs.items) };
                                items.split_off(0)
                            },
                            _ => unreachable!("unsupported mlhs item {:?}", mlhs_inner)
                        };

                        $$ = Value::new_node(
                            self.builder.multi_lhs(
                                Maybe::some($<Token>1),
                                mlhs_items,
                                Maybe::some($<Token>3)
                            )
                        );
                    }
                ;

      mlhs_basic: mlhs_head
                    {
                        $$ = $1;
                    }
                | mlhs_head mlhs_item
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>2 );
                        $$ = Value::new_node_list(nodes);
                    }
                | mlhs_head tSTAR mlhs_node
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( self.builder.splat($<Token>2, Maybe::some($<BoxedNode>3)) );
                        $$ = Value::new_node_list(nodes);
                    }
                | mlhs_head tSTAR mlhs_node tCOMMA mlhs_post
                    {
                        let mut nodes = $<NodeList>1;
                        let mlhs_node = self.builder.splat($<Token>2, Maybe::some($<BoxedNode>3));
                        let mut mlhs_post = $<NodeList>5;

                        nodes.reserve(1 + mlhs_post.len());
                        nodes.push(mlhs_node);
                        nodes.append(&mut mlhs_post);

                        $$ = Value::new_node_list(nodes);
                    }
                | mlhs_head tSTAR
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( self.builder.splat($<Token>2, Maybe::none()) );
                        $$ = Value::new_node_list(nodes);
                    }
                | mlhs_head tSTAR tCOMMA mlhs_post
                    {
                        let mut nodes = $<NodeList>1;
                        let splat = self.builder.splat($<Token>2, Maybe::none());
                        let mut mlhs_post = $<NodeList>4;

                        nodes.reserve(1 + mlhs_post.len());
                        nodes.push(splat);
                        nodes.append(&mut mlhs_post);

                        $$ = Value::new_node_list(nodes);
                    }
                | tSTAR mlhs_node
                    {
                        $$ = Value::new_node_list(
                            bump_vec![in self.bump;
                                self.builder.splat(
                                    $<Token>1,
                                    Maybe::some($<BoxedNode>2)
                                )
                            ]
                        );
                    }
                | tSTAR mlhs_node tCOMMA mlhs_post
                    {
                        let mut nodes;
                        let splat = self.builder.splat($<Token>1, Maybe::some($<BoxedNode>2));
                        let mut mlhs_post = $<NodeList>4;

                        nodes = Vec::with_capacity_in(1 + mlhs_post.len(), self.bump);
                        nodes.push(splat);
                        nodes.append(&mut mlhs_post);

                        $$ = Value::new_node_list(nodes);
                    }
                | tSTAR
                    {
                        $$ = Value::new_node_list(
                                bump_vec![in self.bump;
                                    self.builder.splat(
                                        $<Token>1,
                                        Maybe::none()
                                    )
                                ]
                        );
                    }
                | tSTAR tCOMMA mlhs_post
                    {
                        let mut nodes;
                        let splat = self.builder.splat($<Token>1, Maybe::none());
                        let mut mlhs_post = $<NodeList>3;

                        nodes = Vec::with_capacity_in(1 + mlhs_post.len(), self.bump);
                        nodes.push(splat);
                        nodes.append(&mut mlhs_post);

                        $$ = Value::new_node_list(nodes);
                    }
                ;

       mlhs_item: mlhs_node
                    {
                        $$ = $1;
                    }
                | tLPAREN mlhs_inner rparen
                    {
                        $$ = Value::new_node(
                            self.builder.begin(
                                $<Token>1,
                                Maybe::some($<BoxedNode>2),
                                $<Token>3
                            )
                        );
                    }
                ;

       mlhs_head: mlhs_item tCOMMA
                    {
                        $$ = Value::new_node_list(
                            bump_vec![in self.bump; $<Node>1]
                        );
                    }
                | mlhs_head mlhs_item tCOMMA
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>2 );
                        $$ = Value::new_node_list(nodes);
                    }
                ;

       mlhs_post: mlhs_item
                    {
                        $$ = Value::new_node_list(
                            bump_vec![in self.bump; $<Node>1 ]
                        );
                    }
                | mlhs_post tCOMMA mlhs_item
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::new_node_list(nodes);
                    }
                ;

       mlhs_node: user_variable
                    {
                        $$ = Value::new_node(
                            self.builder.assignable($<BoxedNode>1)?
                        );
                    }
                | keyword_variable
                    {
                        $$ = Value::new_node(
                            self.builder.assignable($<BoxedNode>1)?
                        );
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                        $$ = Value::new_node(
                            self.builder.index_asgn(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<NodeList>3,
                                $<Token>4
                            )
                        );
                    }
                | primary_value call_op tIDENTIFIER
                    {
                        let op_t = $<Token>2;
                        if op_t.token_type() == Lexer::tANDDOT {
                            return self.yyerror(@2, DiagnosticMessage::new_csend_inside_masgn());
                        }

                        $$ = Value::new_node(
                            self.builder.attr_asgn(
                                $<BoxedNode>1,
                                op_t,
                                $<Token>3
                            )
                        );
                    }
                | primary_value tCOLON2 tIDENTIFIER
                    {
                        $$ = Value::new_node(
                            self.builder.attr_asgn(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<Token>3
                            )
                        );
                    }
                | primary_value call_op tCONSTANT
                    {
                        let op_t = $<Token>2;
                        if op_t.token_type() == Lexer::tANDDOT {
                            return self.yyerror(@2, DiagnosticMessage::new_csend_inside_masgn());
                        }

                        $$ = Value::new_node(
                            self.builder.attr_asgn(
                                $<BoxedNode>1,
                                op_t,
                                $<Token>3
                            )
                        );
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                        $$ = Value::new_node(
                            self.builder.assignable(
                                self.builder.const_fetch(
                                    $<BoxedNode>1,
                                    $<Token>2,
                                    $<Token>3
                                )
                            )?
                        );
                    }
                | tCOLON3 tCONSTANT
                    {
                        $$ = Value::new_node(
                            self.builder.assignable(
                                self.builder.const_global(
                                    $<Token>1,
                                    $<Token>2
                                )
                            )?
                        );
                    }
                | backref
                    {
                        $$ = Value::new_node(
                            self.builder.assignable(
                                $<BoxedNode>1
                            )?
                        );
                    }
                ;

             lhs: user_variable
                    {
                        $$ = Value::new_node(
                            self.builder.assignable($<BoxedNode>1)?
                        );
                    }
                | keyword_variable
                    {
                        $$ = Value::new_node(
                            self.builder.assignable($<BoxedNode>1)?
                        );
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                        $$ = Value::new_node(
                            self.builder.index_asgn(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<NodeList>3,
                                $<Token>4
                            )
                        )
                    }
                | primary_value call_op tIDENTIFIER
                    {
                        $$ = Value::new_node(
                            self.builder.attr_asgn(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<Token>3
                            )
                        );
                    }
                | primary_value tCOLON2 tIDENTIFIER
                    {
                        $$ = Value::new_node(
                            self.builder.attr_asgn(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<Token>3
                            )
                        );
                    }
                | primary_value call_op tCONSTANT
                    {
                        $$ = Value::new_node(
                            self.builder.attr_asgn(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<Token>3
                            )
                        );
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                        $$ = Value::new_node(
                            self.builder.assignable(
                                self.builder.const_fetch(
                                    $<BoxedNode>1,
                                    $<Token>2,
                                    $<Token>3,
                                )
                            )?
                        );
                    }
                | tCOLON3 tCONSTANT
                    {
                        $$ = Value::new_node(
                            self.builder.assignable(
                                self.builder.const_global(
                                    $<Token>1,
                                    $<Token>2,
                                )
                            )?
                        );
                    }
                | backref
                    {
                        $$ = Value::new_node(
                            self.builder.assignable(
                                $<BoxedNode>1
                            )?
                        );
                    }
                ;

           cname: tIDENTIFIER
                    {
                        return self.yyerror(@1, DiagnosticMessage::new_class_or_module_name_must_be_constant());
                    }
                | tCONSTANT
                    {
                        $$ = $1;
                    }
                ;

           cpath: tCOLON3 cname
                    {
                        $$ = Value::new_node(
                            self.builder.const_global($<Token>1, $<Token>2)
                        );
                    }
                | cname
                    {
                        $$ = Value::new_node(
                            self.builder.const_($<Token>1)
                        );
                    }
                | primary_value tCOLON2 cname
                    {
                        $$ = Value::new_node(
                            self.builder.const_fetch(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<Token>3,
                            )
                        );
                    }
                ;

           fname: tIDENTIFIER
                    {
                        $$ = $1;
                    }
                | tCONSTANT
                    {
                        $$ = $1;
                    }
                | tFID
                    {
                        $$ = $1;
                    }
                | op
                    {
                        self.yylexer.lex_state.set(EXPR_ENDFN);
                        $$ = $1;
                    }
                | reswords
                    {
                        $$ = $1;
                    }
                ;

           fitem: fname
                    {
                        $$ = Value::new_node(
                            self.builder.symbol_internal($<Token>1)
                        );
                    }
                | symbol
                    {
                        $$ = $1;
                    }
                ;

      undef_list: fitem
                    {
                        $$ = Value::new_node_list(
                            bump_vec![in self.bump; $<Node>1 ]
                        );
                    }
                | undef_list tCOMMA
                    {
                        self.yylexer.lex_state.set(EXPR_FNAME|EXPR_FITEM);
                        $<None>$ = Value::new_none();
                    }
                  fitem
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>4 );
                        $$ = Value::new_node_list(nodes);
                    }
                ;

              op: tPIPE      { $$ = $1; }
                | tCARET     { $$ = $1; }
                | tAMPER2    { $$ = $1; }
                | tCMP       { $$ = $1; }
                | tEQ        { $$ = $1; }
                | tEQQ       { $$ = $1; }
                | tMATCH     { $$ = $1; }
                | tNMATCH    { $$ = $1; }
                | tGT        { $$ = $1; }
                | tGEQ       { $$ = $1; }
                | tLT        { $$ = $1; }
                | tLEQ       { $$ = $1; }
                | tNEQ       { $$ = $1; }
                | tLSHFT     { $$ = $1; }
                | tRSHFT     { $$ = $1; }
                | tPLUS      { $$ = $1; }
                | tMINUS     { $$ = $1; }
                | tSTAR2     { $$ = $1; }
                | tSTAR      { $$ = $1; }
                | tDIVIDE    { $$ = $1; }
                | tPERCENT   { $$ = $1; }
                | tPOW       { $$ = $1; }
                | tDSTAR     { $$ = $1; }
                | tBANG      { $$ = $1; }
                | tTILDE     { $$ = $1; }
                | tUPLUS     { $$ = $1; }
                | tUMINUS    { $$ = $1; }
                | tAREF      { $$ = $1; }
                | tASET      { $$ = $1; }
                | tBACK_REF2 { $$ = $1; }
                ;

        reswords: k__LINE__     { $$ = $1; }
                | k__FILE__     { $$ = $1; }
                | k__ENCODING__ { $$ = $1; }
                | klBEGIN       { $$ = $1; }
                | klEND         { $$ = $1; }
                | kALIAS        { $$ = $1; }
                | kAND          { $$ = $1; }
                | kBEGIN        { $$ = $1; }
                | kBREAK        { $$ = $1; }
                | kCASE         { $$ = $1; }
                | kCLASS        { $$ = $1; }
                | kDEF          { $$ = $1; }
                | kDEFINED      { $$ = $1; }
                | kDO           { $$ = $1; }
                | kELSE         { $$ = $1; }
                | kELSIF        { $$ = $1; }
                | kEND          { $$ = $1; }
                | kENSURE       { $$ = $1; }
                | kFALSE        { $$ = $1; }
                | kFOR          { $$ = $1; }
                | kIN           { $$ = $1; }
                | kMODULE       { $$ = $1; }
                | kNEXT         { $$ = $1; }
                | kNIL          { $$ = $1; }
                | kNOT          { $$ = $1; }
                | kOR           { $$ = $1; }
                | kREDO         { $$ = $1; }
                | kRESCUE       { $$ = $1; }
                | kRETRY        { $$ = $1; }
                | kRETURN       { $$ = $1; }
                | kSELF         { $$ = $1; }
                | kSUPER        { $$ = $1; }
                | kTHEN         { $$ = $1; }
                | kTRUE         { $$ = $1; }
                | kUNDEF        { $$ = $1; }
                | kWHEN         { $$ = $1; }
                | kYIELD        { $$ = $1; }
                | kIF           { $$ = $1; }
                | kUNLESS       { $$ = $1; }
                | kWHILE        { $$ = $1; }
                | kUNTIL        { $$ = $1; }
                ;

             arg: lhs tEQL arg_rhs
                    {
                        $$ = Value::new_node(
                            self.builder.assign(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>3
                            )
                        );
                    }
                | var_lhs tOP_ASGN arg_rhs
                    {
                        $$ = Value::new_node(
                            self.builder.op_assign(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>3
                            )?
                        );
                    }
                | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN arg_rhs
                    {
                        $$ = Value::new_node(
                            self.builder.op_assign(
                                self.builder.index(
                                    $<BoxedNode>1,
                                    $<Token>2,
                                    $<NodeList>3,
                                    $<Token>4
                                ),
                                $<Token>5,
                                $<BoxedNode>6
                            )?
                        );
                    }
                | primary_value call_op tIDENTIFIER tOP_ASGN arg_rhs
                    {
                        $$ = Value::new_node(
                            self.builder.op_assign(
                                self.builder.call_method(
                                    Maybe::some($<BoxedNode>1),
                                    Maybe::some($<Token>2),
                                    Maybe::some($<Token>3),
                                    Maybe::none(),
                                    bump_vec![in self.bump;],
                                    Maybe::none()
                                ),
                                $<Token>4,
                                $<BoxedNode>5
                            )?
                        );
                    }
                | primary_value call_op tCONSTANT tOP_ASGN arg_rhs
                    {
                        $$ = Value::new_node(
                            self.builder.op_assign(
                                self.builder.call_method(
                                    Maybe::some($<BoxedNode>1),
                                    Maybe::some($<Token>2),
                                    Maybe::some($<Token>3),
                                    Maybe::none(),
                                    bump_vec![in self.bump;],
                                    Maybe::none()
                                ),
                                $<Token>4,
                                $<BoxedNode>5
                            )?
                        );
                    }
                | primary_value tCOLON2 tIDENTIFIER tOP_ASGN arg_rhs
                    {
                        $$ = Value::new_node(
                            self.builder.op_assign(
                                self.builder.call_method(
                                    Maybe::some($<BoxedNode>1),
                                    Maybe::some($<Token>2),
                                    Maybe::some($<Token>3),
                                    Maybe::none(),
                                    bump_vec![in self.bump;],
                                    Maybe::none()
                                ),
                                $<Token>4,
                                $<BoxedNode>5
                            )?
                        );
                    }
                | primary_value tCOLON2 tCONSTANT tOP_ASGN arg_rhs
                    {
                        let const_ = self.builder.const_op_assignable(
                            self.builder.const_fetch(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<Token>3
                            )
                        );
                        $$ = Value::new_node(
                            self.builder.op_assign(
                                const_,
                                $<Token>4,
                                $<BoxedNode>5
                            )?
                        );
                    }
                | tCOLON3 tCONSTANT tOP_ASGN arg_rhs
                    {
                        let const_ = self.builder.const_op_assignable(
                            self.builder.const_global(
                                $<Token>1,
                                $<Token>2
                            )
                        );
                        $$ = Value::new_node(
                            self.builder.op_assign(
                                const_,
                                $<Token>3,
                                $<BoxedNode>4
                            )?
                        );
                    }
                | backref tOP_ASGN arg_rhs
                    {
                        $$ = Value::new_node(
                            self.builder.op_assign(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>3
                            )?
                        );
                    }
                | arg tDOT2 arg
                    {
                        let left = $<BoxedNode>1;
                        let left = self.value_expr(left)?;

                        let right = $<BoxedNode>3;
                        let right = self.value_expr(right)?;

                        $$ = Value::new_node(
                            self.builder.range_inclusive(
                                Maybe::some(left),
                                $<Token>2,
                                Maybe::some(right)
                            )
                        );
                    }
                | arg tDOT3 arg
                    {
                        let left = $<BoxedNode>1;
                        let left = self.value_expr(left)?;

                        let right = $<BoxedNode>3;
                        let right = self.value_expr(right)?;

                        $$ = Value::new_node(
                            self.builder.range_exclusive(
                                Maybe::some(left),
                                $<Token>2,
                                Maybe::some(right)
                            )
                        );
                    }
                | arg tDOT2
                    {
                        let left = $<BoxedNode>1;
                        let left = self.value_expr(left)?;

                        $$ = Value::new_node(
                            self.builder.range_inclusive(
                                Maybe::some(left),
                                $<Token>2,
                                Maybe::none()
                            )
                        );
                    }
                | arg tDOT3
                    {
                        let left = $<BoxedNode>1;
                        let left = self.value_expr(left)?;

                        $$ = Value::new_node(
                            self.builder.range_exclusive(
                                Maybe::some(left),
                                $<Token>2,
                                Maybe::none()
                            )
                        );
                    }
                | tBDOT2 arg
                    {
                        let right = $<BoxedNode>2;
                        let right = self.value_expr(right)?;

                        $$ = Value::new_node(
                            self.builder.range_inclusive(
                                Maybe::none(),
                                $<Token>1,
                                Maybe::some(right)
                            )
                        );
                    }
                | tBDOT3 arg
                    {
                        let right = $<BoxedNode>2;
                        let right = self.value_expr(right)?;

                        $$ = Value::new_node(
                            self.builder.range_exclusive(
                                Maybe::none(),
                                $<Token>1,
                                Maybe::some(right)
                            )
                        );
                    }
                | arg tPLUS arg
                    {
                        $$ = Value::new_node(
                            self.builder.binary_op($<BoxedNode>1, $<Token>2, $<BoxedNode>3)?
                        );
                    }
                | arg tMINUS arg
                    {
                        $$ = Value::new_node(
                            self.builder.binary_op($<BoxedNode>1, $<Token>2, $<BoxedNode>3)?
                        );
                    }
                | arg tSTAR2 arg
                    {
                        $$ = Value::new_node(
                            self.builder.binary_op($<BoxedNode>1, $<Token>2, $<BoxedNode>3)?
                        );
                    }
                | arg tDIVIDE arg
                    {
                        $$ = Value::new_node(
                            self.builder.binary_op($<BoxedNode>1, $<Token>2, $<BoxedNode>3)?
                        );
                    }
                | arg tPERCENT arg
                    {
                        $$ = Value::new_node(
                            self.builder.binary_op($<BoxedNode>1, $<Token>2, $<BoxedNode>3)?
                        );
                    }
                | arg tPOW arg
                    {
                        $$ = Value::new_node(
                            self.builder.binary_op($<BoxedNode>1, $<Token>2, $<BoxedNode>3)?
                        );
                    }
                | tUMINUS_NUM simple_numeric tPOW arg
                    {
                        $$ = Value::new_node(
                            self.builder.unary_op(
                                $<Token>1,
                                self.builder.binary_op(
                                    $<BoxedNode>2,
                                    $<Token>3,
                                    $<BoxedNode>4
                                )?
                            )?
                        );
                    }
                | tUPLUS arg
                    {
                        $$ = Value::new_node(
                            self.builder.unary_op(
                                $<Token>1,
                                $<BoxedNode>2
                            )?
                        );
                    }
                | tUMINUS arg
                    {
                        $$ = Value::new_node(
                            self.builder.unary_op(
                                $<Token>1,
                                $<BoxedNode>2
                            )?
                        );
                    }
                | arg tPIPE arg
                    {
                        $$ = Value::new_node(
                            self.builder.binary_op($<BoxedNode>1, $<Token>2, $<BoxedNode>3)?
                        );
                    }
                | arg tCARET arg
                    {
                        $$ = Value::new_node(
                            self.builder.binary_op($<BoxedNode>1, $<Token>2, $<BoxedNode>3)?
                        );
                    }
                | arg tAMPER2 arg
                    {
                        $$ = Value::new_node(
                            self.builder.binary_op($<BoxedNode>1, $<Token>2, $<BoxedNode>3)?
                        );
                    }
                | arg tCMP arg
                    {
                        $$ = Value::new_node(
                            self.builder.binary_op($<BoxedNode>1, $<Token>2, $<BoxedNode>3)?
                        );
                    }
                | rel_expr   %prec tCMP
                    {
                        $$ = $1;
                    }
                | arg tEQ arg
                    {
                        $$ = Value::new_node(
                            self.builder.binary_op($<BoxedNode>1, $<Token>2, $<BoxedNode>3)?
                        );
                    }
                | arg tEQQ arg
                    {
                        $$ = Value::new_node(
                            self.builder.binary_op($<BoxedNode>1, $<Token>2, $<BoxedNode>3)?
                        );
                    }
                | arg tNEQ arg
                    {
                        $$ = Value::new_node(
                            self.builder.binary_op($<BoxedNode>1, $<Token>2, $<BoxedNode>3)?
                        );
                    }
                | arg tMATCH arg
                    {
                        $$ = Value::new_node(
                            self.builder.match_op($<BoxedNode>1, $<Token>2, $<BoxedNode>3)?
                        );
                    }
                | arg tNMATCH arg
                    {
                        $$ = Value::new_node(
                            self.builder.binary_op(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>3
                            )?
                        );
                    }
                | tBANG arg
                    {
                        $$ = Value::new_node(
                            self.builder.not_op(
                                $<Token>1,
                                Maybe::none(),
                                Maybe::some($<BoxedNode>2),
                                Maybe::none()
                            )?
                        );
                    }
                | tTILDE arg
                    {
                        $$ = Value::new_node(
                            self.builder.unary_op(
                                $<Token>1,
                                $<BoxedNode>2
                            )?
                        );
                    }
                | arg tLSHFT arg
                    {
                        $$ = Value::new_node(
                            self.builder.binary_op($<BoxedNode>1, $<Token>2, $<BoxedNode>3)?
                        );
                    }
                | arg tRSHFT arg
                    {
                        $$ = Value::new_node(
                            self.builder.binary_op($<BoxedNode>1, $<Token>2, $<BoxedNode>3)?
                        );
                    }
                | arg tANDOP arg
                    {
                        $$ = Value::new_node(
                            self.builder.logical_op(
                                LogicalOp::And,
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>3
                            )?
                        );
                    }
                | arg tOROP arg
                    {
                        $$ = Value::new_node(
                            self.builder.logical_op(
                                LogicalOp::Or,
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>3
                            )?
                        );
                    }
                | kDEFINED opt_nl arg
                    {
                        $$ = Value::new_node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Defined,
                                $<Token>1,
                                Maybe::none(),
                                bump_vec![in self.bump; $<Node>3 ],
                                Maybe::none()
                            )?
                        );
                    }
                | arg tEH arg opt_nl tCOLON arg
                    {
                        let expr = $<BoxedNode>1;
                        let expr = self.value_expr(expr)?;

                        $$ = Value::new_node(
                            self.builder.ternary(
                                expr,
                                $<Token>2,
                                $<BoxedNode>3,
                                $<Token>5,
                                $<BoxedNode>6
                            )
                        );
                    }
                | defn_head f_opt_paren_args tEQL arg
                    {
                        let DefnHead { def_t, name_t } = $<DefnHead>1;
                        self.validate_endless_method_name(&name_t)?;

                        $$ = Value::new_node(
                            self.builder.def_endless_method(
                                def_t,
                                name_t,
                                $<MaybeBoxedNode>2,
                                $<Token>3,
                                Maybe::some($<BoxedNode>4)
                            )?
                        );

                        self.yylexer.cmdarg.pop();
                        self.yylexer.cond.pop();
                        self.static_env.unextend();
                        self.context.pop();
                        self.current_arg_stack.pop();
                    }
                | defn_head f_opt_paren_args tEQL arg kRESCUE_MOD arg
                    {
                        let DefnHead { def_t, name_t } = $<DefnHead>1;
                        self.validate_endless_method_name(&name_t)?;

                        let rescue_body = self.builder.rescue_body(
                            $<Token>5,
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::some($<BoxedNode>6)
                        );

                        let method_body = self.builder.begin_body(
                            Maybe::some($<BoxedNode>4),
                            bump_vec![in self.bump; rescue_body ],
                            None,
                            None,
                        );

                        $$ = Value::new_node(
                            self.builder.def_endless_method(
                                def_t,
                                name_t,
                                $<MaybeBoxedNode>2,
                                $<Token>3,
                                method_body
                            )?
                        );

                        self.yylexer.cmdarg.pop();
                        self.yylexer.cond.pop();
                        self.static_env.unextend();
                        self.context.pop();
                        self.current_arg_stack.pop();
                    }
                | defs_head f_opt_paren_args tEQL arg
                    {
                        let DefsHead { def_t, definee, dot_t, name_t } = $<DefsHead>1;
                        self.validate_endless_method_name(&name_t)?;

                        $$ = Value::new_node(
                            self.builder.def_endless_singleton(
                                def_t,
                                definee,
                                dot_t,
                                name_t,
                                $<MaybeBoxedNode>2,
                                $<Token>3,
                                Maybe::some($<BoxedNode>4)
                            )?
                        );

                        self.yylexer.cmdarg.pop();
                        self.yylexer.cond.pop();
                        self.static_env.unextend();
                        self.context.pop();
                        self.current_arg_stack.pop();
                    }
                | defs_head f_opt_paren_args tEQL arg kRESCUE_MOD arg
                    {
                        let DefsHead { def_t, definee, dot_t, name_t } = $<DefsHead>1;
                        self.validate_endless_method_name(&name_t)?;

                        let rescue_body = self.builder.rescue_body(
                            $<Token>5,
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::some($<BoxedNode>6)
                        );

                        let method_body = self.builder.begin_body(
                            Maybe::some($<BoxedNode>4),
                            bump_vec![in self.bump; rescue_body ],
                            None,
                            None,
                        );

                        $$ = Value::new_node(
                            self.builder.def_endless_singleton(
                                def_t,
                                definee,
                                dot_t,
                                name_t,
                                $<MaybeBoxedNode>2,
                                $<Token>3,
                                method_body
                            )?
                        );

                        self.yylexer.cmdarg.pop();
                        self.yylexer.cond.pop();
                        self.static_env.unextend();
                        self.context.pop();
                        self.current_arg_stack.pop();
                    }
                | primary
                    {
                        $$ = $1;
                    }
                ;

           relop: tGT
                    {
                        $$ = $1;
                    }
                | tLT
                    {
                        $$ = $1;
                    }
                | tGEQ
                    {
                        $$ = $1;
                    }
                | tLEQ
                    {
                        $$ = $1;
                    }
                ;

        rel_expr: arg relop arg   %prec tGT
                    {
                        $$ = Value::new_node(
                            self.builder.binary_op(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>3
                            )?
                        );
                    }
                | rel_expr relop arg   %prec tGT
                    {
                        let op_t = $<Token>2;
                        self.warn(
                            @2,
                            DiagnosticMessage::new_comparison_after_comparison(clone_value(&op_t))
                        );
                        $$ = Value::new_node(
                            self.builder.binary_op(
                                $<BoxedNode>1,
                                op_t,
                                $<BoxedNode>3
                            )?
                        );
                    }
                ;

       arg_value: arg
                    {
                        let arg = $<BoxedNode>1;
                        let arg = self.value_expr(arg)?;
                        $$ = Value::new_node(arg);
                    }
                ;

       aref_args: none
                    {
                        $$ = Value::new_node_list(
                            bump_vec![in self.bump;]
                        );
                    }
                | args trailer
                    {
                        $$ = $1;
                    }
                | args tCOMMA assocs trailer
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push(
                            self.builder.associate(
                                Maybe::none(),
                                $<NodeList>3,
                                Maybe::none()
                            )
                        );
                        $$ = Value::new_node_list(nodes);
                    }
                | assocs trailer
                    {
                        $$ = Value::new_node_list(
                            bump_vec![in self.bump;
                                self.builder.associate(
                                    Maybe::none(),
                                    $<NodeList>1,
                                    Maybe::none()
                                )
                            ]
                        );
                    }
                ;

         arg_rhs: arg   %prec tOP_ASGN
                    {
                        let arg = $<BoxedNode>1;
                        let arg = self.value_expr(arg)?;
                        $$ = Value::new_node(arg);
                    }
                | arg kRESCUE_MOD arg
                    {
                        let arg = $<BoxedNode>1;
                        let arg = self.value_expr(arg)?;

                        let rescue_body = self.builder.rescue_body(
                            $<Token>2,
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::some($<BoxedNode>3)
                        );

                        $$ = Value::new_node(
                            self.builder.begin_body(
                                Maybe::some(arg),
                                bump_vec![in self.bump; rescue_body],
                                None,
                                None,
                            ).expect("expected begin_body to return Maybe::some (compound_stmt was given)")
                        );
                    }
                ;

      paren_args: tLPAREN2 opt_call_args rparen
                    {
                        $$ = Value::new_paren_args(
                            self.bump.alloc(
                                ParenArgs {
                                    begin_t: $<Token>1,
                                    args: $<NodeList>2,
                                    end_t: $<Token>3
                                }
                            )
                        );
                    }
                | tLPAREN2 args tCOMMA args_forward rparen
                    {
                        if !self.static_env.is_forward_args_declared() {
                            return self.yyerror(
                                @4,
                                DiagnosticMessage::new_unexpected_token(
                                    String::from_str_in(
                                        "tBDOT3",
                                        self.bump
                                    )
                                )
                            );
                        }

                        let mut args = $<NodeList>2;
                        args.push(self.builder.forwarded_args($<Token>4));

                        $$ = Value::new_paren_args(
                            self.bump.alloc(
                                ParenArgs {
                                    begin_t: $<Token>1,
                                    args,
                                    end_t: $<Token>5
                                }
                            )
                        );
                    }
                | tLPAREN2 args_forward rparen
                    {
                        if !self.static_env.is_forward_args_declared() {
                            return self.yyerror(
                                @2,
                                DiagnosticMessage::new_unexpected_token(
                                    String::from_str_in(
                                        "tBDOT3",
                                        self.bump
                                    )
                                )
                            );
                        }

                        $$ = Value::new_paren_args(
                            self.bump.alloc(
                                ParenArgs {
                                    begin_t: $<Token>1,
                                    args: bump_vec![in self.bump; self.builder.forwarded_args($<Token>2) ],
                                    end_t: $<Token>3
                                }
                            )
                        );
                    }
                ;

  opt_paren_args: none
                    {
                        $$ = Value::new_opt_paren_args(
                            self.bump.alloc(
                                OptParenArgs {
                                    begin_t: Maybe::none(),
                                    args: bump_vec![in self.bump;],
                                    end_t: Maybe::none()
                                }
                            )
                        );
                    }
                | paren_args
                    {
                        let ParenArgs { begin_t, args, end_t } = $<ParenArgs>1;
                        $$ = Value::new_opt_paren_args(
                            self.bump.alloc(
                                OptParenArgs {
                                    begin_t: Maybe::some(*begin_t),
                                    args: take_vec(args),
                                    end_t: Maybe::some(*end_t)
                                }
                            )
                        );
                    }
                ;

   opt_call_args: none
                    {
                        $$ = Value::new_node_list(bump_vec![in self.bump;] );
                    }
                | call_args
                    {
                        $$ = $1;
                    }
                | args tCOMMA
                    {
                        $$ = $1;
                    }
                | args tCOMMA assocs tCOMMA
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( self.builder.associate(Maybe::none(), $<NodeList>3, Maybe::none()) );
                        $$ = Value::new_node_list(nodes);
                    }
                | assocs tCOMMA
                    {
                        $$ = Value::new_node_list(
                                bump_vec![in self.bump;
                                    self.builder.associate(
                                        Maybe::none(),
                                        $<NodeList>1,
                                        Maybe::none()
                                    )
                                ]
                        );
                    }
                ;

       call_args: command
                    {
                        let command = $<Node>1;
                        let command = self.value_expr(command)?;
                        $$ = Value::new_node_list( bump_vec![in self.bump; command ]);
                    }
                | args opt_block_arg
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.append(&mut $<NodeList>2);

                        $$ = Value::new_node_list(nodes);
                    }
                | assocs opt_block_arg
                    {
                        let mut nodes;
                        let hash = self.builder.associate(Maybe::none(), $<NodeList>1, Maybe::none());
                        let mut opt_block_arg = $<NodeList>2;

                        nodes = Vec::with_capacity_in(1 + opt_block_arg.len(), self.bump);
                        nodes.push(hash);
                        nodes.append(&mut opt_block_arg);

                        $$ = Value::new_node_list(nodes);
                    }
                | args tCOMMA assocs opt_block_arg
                    {
                        let mut nodes = $<NodeList>1;
                        let hash = self.builder.associate(Maybe::none(), $<NodeList>3, Maybe::none());
                        let mut opt_block_arg = $<NodeList>4;

                        nodes.reserve(1 + opt_block_arg.len());
                        nodes.push(hash);
                        nodes.append(&mut opt_block_arg);

                        $$ = Value::new_node_list(nodes);
                    }
                | block_arg
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>1 ]);
                    }
                ;

    command_args:   {
                        let lookahead =
                            matches!(
                                self.last_token_type,
                                Lexer::tLPAREN2
                                    | Lexer::tLPAREN
                                    | Lexer:: tLPAREN_ARG
                                    | Lexer::tLBRACK2
                                    | Lexer::tLBRACK
                            );

                        if lookahead { self.yylexer.cmdarg.pop() }
                        self.yylexer.cmdarg.push(true);
                        if lookahead { self.yylexer.cmdarg.push(false) }
                        $<None>$ = Value::new_none();
                    }
                  call_args
                    {
                        let lookahead = matches!(self.last_token_type, Lexer::tLBRACE_ARG);

                        if lookahead { self.yylexer.cmdarg.pop() }
                        self.yylexer.cmdarg.pop();
                        if lookahead { self.yylexer.cmdarg.push(false) }

                        $$ = $2;
                    }
                ;

       block_arg: tAMPER arg_value
                    {
                        $$ = Value::new_node(
                            self.builder.block_pass(
                                $<Token>1,
                                $<BoxedNode>2
                            )
                        );
                    }
                ;

   opt_block_arg: tCOMMA block_arg
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>2 ]);
                    }
                | none
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump;]);
                    }
                ;

            args: arg_value
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>1 ]);
                    }
                | tSTAR arg_value
                    {
                        $$ = Value::new_node_list(
                            bump_vec![in self.bump;
                                self.builder.splat(
                                    $<Token>1,
                                    Maybe::some($<BoxedNode>2)
                                )
                            ]
                        );
                    }
                | args tCOMMA arg_value
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::new_node_list(nodes);
                    }
                | args tCOMMA tSTAR arg_value
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( self.builder.splat($<Token>3, Maybe::some($<BoxedNode>4)) );
                        $$ = Value::new_node_list(nodes);
                    }
                ;

        mrhs_arg: mrhs
                    {
                        $$ = Value::new_node(
                            self.builder.array(Maybe::none(), $<NodeList>1, Maybe::none())
                        );
                    }
                | arg_value
                    {
                        $$ = $1;
                    }
                ;

            mrhs: args tCOMMA arg_value
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::new_node_list(nodes);
                    }
                | args tCOMMA tSTAR arg_value
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push(
                            self.builder.splat($<Token>3, Maybe::some($<BoxedNode>4))
                        );
                        $$ = Value::new_node_list(nodes);
                    }
                | tSTAR arg_value
                    {
                        $$ = Value::new_node_list(
                            bump_vec![in self.bump;
                                self.builder.splat(
                                    $<Token>1,
                                    Maybe::some($<BoxedNode>2)
                                )
                            ]
                        );
                    }
                ;

         primary: literal
                    {
                        $$ = $1;
                    }
                | strings
                    {
                        $$ = $1;
                    }
                | xstring
                    {
                        $$ = $1;
                    }
                | regexp
                    {
                        $$ = $1;
                    }
                | words
                    {
                        $$ = $1;
                    }
                | qwords
                    {
                        $$ = $1;
                    }
                | symbols
                    {
                        $$ = $1;
                    }
                | qsymbols
                    {
                        $$ = $1;
                    }
                | var_ref
                    {
                        $$ = $1;
                    }
                | backref
                    {
                        $$ = $1;
                    }
                | tFID
                    {
                        $$ = Value::new_node(
                            self.builder.call_method(
                                Maybe::none(),
                                Maybe::none(),
                                Maybe::some($<Token>1),
                                Maybe::none(),
                                bump_vec![in self.bump;],
                                Maybe::none()
                            )
                        );
                    }
                | k_begin
                    {
                        self.yylexer.cmdarg.push(false);
                        $<None>$ = Value::new_none();
                    }
                  bodystmt
                  k_end
                    {
                        self.yylexer.cmdarg.pop();

                        $$ = Value::new_node(
                            self.builder.begin_keyword($<Token>1, $<MaybeBoxedNode>3, $<Token>4)
                        );
                    }
                | tLPAREN_ARG { self.yylexer.lex_state.set(EXPR_ENDARG); $<None>$ = Value::new_none(); } rparen
                    {
                        $$ = Value::new_node(
                            self.builder.begin(
                                $<Token>1,
                                Maybe::none(),
                                $<Token>3
                            )
                        );
                    }
                | tLPAREN_ARG stmt { self.yylexer.lex_state.set(EXPR_ENDARG); $<None>$ = Value::new_none(); } rparen
                    {
                        $$ = Value::new_node(
                            self.builder.begin(
                                $<Token>1,
                                Maybe::some($<BoxedNode>2),
                                $<Token>4
                            )
                        );
                    }
                | tLPAREN compstmt tRPAREN
                    {
                        $$ = Value::new_node(
                            self.builder.begin(
                                $<Token>1,
                                $<MaybeBoxedNode>2,
                                $<Token>3
                            )
                        );
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                        $$ = Value::new_node(
                            self.builder.const_fetch(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<Token>3,
                            )
                        );
                    }
                | tCOLON3 tCONSTANT
                    {
                        $$ = Value::new_node(
                            self.builder.const_global($<Token>1, $<Token>2)
                        );
                    }
                | tLBRACK aref_args tRBRACK
                    {
                        $$ = Value::new_node(
                            self.builder.array(
                                Maybe::some($<Token>1),
                                $<NodeList>2,
                                Maybe::some($<Token>3)
                            )
                        );
                    }
                | tLBRACE assoc_list tRCURLY
                    {
                        $$ = Value::new_node(
                            self.builder.associate(
                                Maybe::some($<Token>1),
                                $<NodeList>2,
                                Maybe::some($<Token>3)
                            )
                        );
                    }
                | k_return
                    {
                        $$ = Value::new_node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Return,
                                $<Token>1,
                                Maybe::none(),
                                bump_vec![in self.bump;],
                                Maybe::none()
                            )?
                        );
                    }
                | kYIELD tLPAREN2 call_args rparen
                    {
                        $$ = Value::new_node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Yield,
                                $<Token>1,
                                Maybe::some($<Token>2),
                                $<NodeList>3,
                                Maybe::some($<Token>4)
                            )?
                        );
                    }
                | kYIELD tLPAREN2 rparen
                    {
                        $$ = Value::new_node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Yield,
                                $<Token>1,
                                Maybe::some($<Token>2),
                                bump_vec![in self.bump;],
                                Maybe::some($<Token>3)
                            )?
                        );
                    }
                | kYIELD
                    {
                        $$ = Value::new_node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Yield,
                                $<Token>1,
                                Maybe::none(),
                                bump_vec![in self.bump;],
                                Maybe::none()
                            )?
                        );
                    }
                | kDEFINED opt_nl tLPAREN2 expr rparen
                    {
                        $$ = Value::new_node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Defined,
                                $<Token>1,
                                Maybe::some($<Token>3),
                                bump_vec![in self.bump; $<Node>4 ],
                                Maybe::some($<Token>5)
                            )?
                        );
                    }
                | kNOT tLPAREN2 expr rparen
                    {
                        $$ = Value::new_node(
                            self.builder.not_op(
                                $<Token>1,
                                Maybe::some($<Token>2),
                                Maybe::some($<BoxedNode>3),
                                Maybe::some($<Token>4)
                            )?
                        );
                    }
                | kNOT tLPAREN2 rparen
                    {
                        $$ = Value::new_node(
                            self.builder.not_op(
                                $<Token>1,
                                Maybe::some($<Token>2),
                                Maybe::none(),
                                Maybe::some($<Token>3)
                            )?
                        );
                    }
                | fcall brace_block
                    {
                        let method_call = self.builder.call_method(
                            Maybe::none(),
                            Maybe::none(),
                            Maybe::some($<Token>1),
                            Maybe::none(),
                            bump_vec![in self.bump;],
                            Maybe::none()
                        );
                        let BraceBlock { begin_t, args_type, body, end_t } = *$<BraceBlock>2;

                        $$ = Value::new_node(
                            self.builder.block(
                                method_call,
                                begin_t,
                                args_type,
                                body,
                                end_t
                            )?
                        );
                    }
                | method_call
                    {
                        $$ = $1;
                    }
                | method_call brace_block
                    {
                        let BraceBlock { begin_t, args_type, body, end_t } = *$<BraceBlock>2;
                        $$ = Value::new_node(
                            self.builder.block(
                                $<BoxedNode>1,
                                begin_t,
                                args_type,
                                body,
                                end_t
                            )?
                        );
                    }
                | lambda
                    {
                        $$ = $1;
                    }
                | k_if expr_value then
                  compstmt
                  if_tail
                  k_end
                    {
                        let IfTail { keyword_t, body: else_body } = *$<IfTail>5;

                        $$ = Value::new_node(
                            self.builder.condition(
                                $<Token>1,
                                $<BoxedNode>2,
                                $<Token>3,
                                $<MaybeBoxedNode>4,
                                keyword_t,
                                else_body,
                                Maybe::some($<Token>6)
                            )
                        );
                    }
                | k_unless expr_value then
                  compstmt
                  opt_else
                  k_end
                    {
                        let (else_t, body) = $<OptElse>5.map(|else_| (Maybe::some(else_.else_t), else_.body)).unwrap_or_else(|| (Maybe::none(), Maybe::none()));

                        $$ = Value::new_node(
                            self.builder.condition(
                                $<Token>1,
                                $<BoxedNode>2,
                                $<Token>3,
                                body,
                                else_t,
                                $<MaybeBoxedNode>4,
                                Maybe::some($<Token>6)
                            )
                        );
                    }
                | k_while expr_value_do
                  compstmt
                  k_end
                    {
                        let ExprValueDo { value, do_t } = $<ExprValueDo>2;
                        $$ = Value::new_node(
                            self.builder.loop_(
                                LoopType::While,
                                $<Token>1,
                                value,
                                do_t,
                                $<MaybeBoxedNode>3,
                                $<Token>4
                            )
                        );
                    }
                | k_until expr_value_do
                  compstmt
                  k_end
                    {
                        let ExprValueDo { value, do_t } = $<ExprValueDo>2;
                        $$ = Value::new_node(
                            self.builder.loop_(
                                LoopType::Until,
                                $<Token>1,
                                value,
                                do_t,
                                $<MaybeBoxedNode>3,
                                $<Token>4
                            )
                        );
                    }
                | k_case expr_value opt_terms
                    {
                        // TODO: there's a warning that wq/parser doesn't trigger,
                        // search for `p->case_labels`
                        $<None>$ = Value::new_none();
                    }
                  case_body
                  k_end
                    {
                        let CaseBody { when_bodies, opt_else } = $<CaseBody>5;
                        let (else_t, else_body) = opt_else.map(|else_| (Maybe::some(else_.else_t), else_.body)).unwrap_or_else(|| (Maybe::none(), Maybe::none()));

                        $$ = Value::new_node(
                            self.builder.case(
                                $<Token>1,
                                Maybe::some($<BoxedNode>2),
                                take_vec(when_bodies),
                                else_t,
                                else_body,
                                $<Token>6
                            )
                        );
                    }
                | k_case opt_terms
                    {
                        // TODO: there's a warning that wq/parser doesn't trigger,
                        // search for `p->case_labels`
                        $<None>$ = Value::new_none();
                    }
                  case_body
                  k_end
                    {
                        let CaseBody { when_bodies, opt_else } = $<CaseBody>4;
                        let (else_t, else_body) = opt_else.map(|else_| (Maybe::some(else_.else_t), else_.body)).unwrap_or_else(|| (Maybe::none(), Maybe::none()));

                        $$ = Value::new_node(
                            self.builder.case(
                                $<Token>1,
                                Maybe::none(),
                                take_vec(when_bodies),
                                else_t,
                                else_body,
                                $<Token>5
                            )
                        );
                    }
                | k_case expr_value opt_terms
                  p_case_body
                  k_end
                    {
                        let PCaseBody { in_bodies, opt_else } = $<PCaseBody>4;
                        let (else_t, else_body) = opt_else.map(|else_| (Maybe::some(else_.else_t), else_.body)).unwrap_or_else(|| (Maybe::none(), Maybe::none()));

                        $$ = Value::new_node(
                            self.builder.case_match(
                                $<Token>1,
                                $<BoxedNode>2,
                                take_vec(in_bodies),
                                else_t,
                                else_body,
                                $<Token>5
                            )
                        );
                    }
                | k_for for_var kIN expr_value_do
                  compstmt
                  k_end
                    {
                        let ExprValueDo { value, do_t } = $<ExprValueDo>4;
                        $$ = Value::new_node(
                            self.builder.for_(
                                $<Token>1,
                                $<BoxedNode>2,
                                $<Token>3,
                                value,
                                do_t,
                                $<MaybeBoxedNode>5,
                                $<Token>6
                            )
                        );
                    }
                | k_class cpath superclass
                    {
                        self.static_env.extend_static();
                        self.yylexer.cmdarg.push(false);
                        self.yylexer.cond.push(false);
                        self.context.push_class();
                        $<None>$ = Value::new_none();
                    }
                  bodystmt
                  k_end
                    {
                        if !self.context.is_class_definition_allowed() {
                            return self.yyerror(@1, DiagnosticMessage::new_class_definition_in_method_body());
                        }

                        let Superclass { lt_t, value } = $<Superclass>3;

                        $$ = Value::new_node(
                            self.builder.def_class(
                                $<Token>1,
                                $<BoxedNode>2,
                                take_maybe_token(lt_t),
                                take_maybe_node(value),
                                $<MaybeBoxedNode>5,
                                $<Token>6
                            )
                        );

                        self.yylexer.cmdarg.pop();
                        self.yylexer.cond.pop();
                        self.static_env.unextend();
                        self.context.pop();
                    }
                | k_class tLSHFT expr
                    {
                        self.static_env.extend_static();
                        self.yylexer.cmdarg.push(false);
                        self.yylexer.cond.push(false);
                        self.context.push_sclass();
                        $<None>$ = Value::new_none();
                    }
                  term
                  bodystmt
                  k_end
                    {
                        $$ = Value::new_node(
                            self.builder.def_sclass(
                                $<Token>1,
                                $<Token>2,
                                $<BoxedNode>3,
                                $<MaybeBoxedNode>6,
                                $<Token>7
                            )
                        );

                        self.yylexer.cmdarg.pop();
                        self.yylexer.cond.pop();
                        self.static_env.unextend();
                        self.context.pop();
                    }
                | k_module cpath
                    {
                        self.static_env.extend_static();
                        self.yylexer.cmdarg.push(false);
                        self.context.push_module();
                        $<None>$ = Value::new_none();
                    }
                  bodystmt
                  k_end
                    {
                        if !self.context.is_module_definition_allowed() {
                            return self.yyerror(@1, DiagnosticMessage::new_module_definition_in_method_body());
                        }

                        $$ = Value::new_node(
                            self.builder.def_module(
                                $<Token>1,
                                $<BoxedNode>2,
                                $<MaybeBoxedNode>4,
                                $<Token>5
                            )
                        );

                        self.yylexer.cmdarg.pop();
                        self.static_env.unextend();
                        self.context.pop();
                    }
                | defn_head
                  f_arglist
                  bodystmt
                  k_end
                    {
                        let DefnHead { def_t, name_t } = $<DefnHead>1;

                        $$ = Value::new_node(
                            self.builder.def_method(
                                def_t,
                                name_t,
                                $<MaybeBoxedNode>2,
                                $<MaybeBoxedNode>3,
                                $<Token>4
                            )?
                        );

                        self.yylexer.cmdarg.pop();
                        self.yylexer.cond.pop();
                        self.static_env.unextend();
                        self.context.pop();
                        self.current_arg_stack.pop();
                    }
                | defs_head
                  f_arglist
                  bodystmt
                  k_end
                    {
                        let DefsHead { def_t, definee, dot_t, name_t } = $<DefsHead>1;

                        $$ = Value::new_node(
                            self.builder.def_singleton(
                                def_t,
                                definee,
                                dot_t,
                                name_t,
                                $<MaybeBoxedNode>2,
                                $<MaybeBoxedNode>3,
                                $<Token>4
                            )?
                        );

                        self.yylexer.cmdarg.pop();
                        self.yylexer.cond.pop();
                        self.static_env.unextend();
                        self.context.pop();
                        self.current_arg_stack.pop();
                    }
                | kBREAK
                    {
                        $$ = Value::new_node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Break,
                                $<Token>1,
                                Maybe::none(),
                                bump_vec![in self.bump;],
                                Maybe::none()
                            )?
                        );
                    }
                | kNEXT
                    {
                        $$ = Value::new_node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Next,
                                $<Token>1,
                                Maybe::none(),
                                bump_vec![in self.bump;],
                                Maybe::none()
                            )?
                        );
                    }
                | kREDO
                    {
                        $$ = Value::new_node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Redo,
                                $<Token>1,
                                Maybe::none(),
                                bump_vec![in self.bump;],
                                Maybe::none()
                            )?
                        );
                    }
                | kRETRY
                    {
                        $$ = Value::new_node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Retry,
                                $<Token>1,
                                Maybe::none(),
                                bump_vec![in self.bump;],
                                Maybe::none()
                            )?
                        );
                    }
                ;

   primary_value: primary
                    {
                        let primary = $<BoxedNode>1;
                        let primary = self.value_expr(primary)?;
                        $$ = Value::new_node(primary);
                    }
                ;

         k_begin: kBEGIN
                    {
                        $$ = $1;
                    }
                ;

            k_if: kIF
                    {
                        self.warn_eol(@1, "if");
                        $$ = $1;
                    }
                ;

        k_unless: kUNLESS
                    {
                        $$ = $1;
                    }
                ;

         k_while: kWHILE
                    {
                        $$ = $1;
                    }
                ;

         k_until: kUNTIL
                    {
                        $$ = $1;
                    }
                ;

          k_case: kCASE
                    {
                        $$ = $1;
                    }
                ;

           k_for: kFOR
                    {
                        $$ = $1;
                    }
                ;

         k_class: kCLASS
                    {
                        $$ = $1;
                    }
                ;

        k_module: kMODULE
                    {
                        $$ = $1;
                    }
                ;

           k_def: kDEF
                    {
                        $$ = $1;
                    }
                ;

            k_do: kDO
                    {
                        $$ = $1;
                    }
                ;

      k_do_block: kDO_BLOCK
                    {
                        $$ = $1;
                    }
                ;

        k_rescue: kRESCUE
                    {
                        $$ = $1;
                    }
                ;

        k_ensure: kENSURE
                    {
                        $$ = $1;
                    }
                ;

          k_when: kWHEN
                    {
                        $$ = $1;
                    }
                ;

          k_else: kELSE
                    {
                        $$ = $1;
                    }
                ;

         k_elsif: kELSIF
                    {
                        self.warn_eol(@1, "elsif");
                        $$ = $1;
                    }
                ;

           k_end: kEND
                    {
                        $$ = $1;
                    }
                ;

        k_return: kRETURN
                    {
                        if self.context.is_in_class() {
                            return self.yyerror(@1, DiagnosticMessage::new_invalid_return_in_class_or_module_body());
                        }
                        $$ = $1;
                    }
                ;

            then: term
                    {
                        $$ = $1;
                    }
                | kTHEN
                    {
                        $$ = $1;
                    }
                | term kTHEN
                    {
                        $$ = $2;
                    }
                ;

              do: term
                    {
                        $$ = $1;
                    }
                | kDO_COND
                    {
                        $$ = $1;
                    }
                ;

         if_tail: opt_else
                    {
                        let (keyword_t, body) = $<OptElse>1.map(|else_| (Maybe::some(else_.else_t), else_.body)).unwrap_or_else(|| (Maybe::none(), Maybe::none()));
                        $$ = Value::new_if_tail(
                            self.bump.alloc(
                                IfTail { keyword_t, body }
                            )
                        );
                    }
                | k_elsif expr_value then
                  compstmt
                  if_tail
                    {
                        let IfTail { keyword_t, body: else_body } = *$<IfTail>5;

                        let elsif_t = $<Token>1;

                        $$ = Value::new_if_tail(
                            self.bump.alloc(
                                IfTail {
                                    keyword_t: Maybe::some(elsif_t),
                                    body: Maybe::some(
                                        self.builder.condition(
                                            elsif_t,
                                            $<BoxedNode>2,
                                            $<Token>3,
                                            $<MaybeBoxedNode>4,
                                            keyword_t,
                                            else_body,
                                            Maybe::none()
                                        )
                                    )
                                }
                            )
                        );
                    }
                ;

        opt_else: none
                    {
                        $$ = Value::new_opt_else(
                            None
                        );
                    }
                | k_else compstmt
                    {
                        let else_t = $<Token>1;
                        let body   = $<MaybeBoxedNode>2;
                        $$ = Value::new_opt_else(
                            Some(
                                self.bump.alloc(
                                    Else { else_t, body }
                                )
                            )
                        );
                    }
                ;

         for_var: lhs
                    {
                        $$ = $1;
                    }
                | mlhs
                    {
                        $$ = $1;
                    }
                ;

          f_marg: f_norm_arg
                    {
                        $$ = Value::new_node(
                            self.builder.arg($<Token>1)?
                        );
                    }
                | tLPAREN f_margs rparen
                    {
                        $$ = Value::new_node(
                            self.builder.multi_lhs(
                                Maybe::some($<Token>1),
                                $<NodeList>2,
                                Maybe::some($<Token>3)
                            )
                        );
                    }
                ;

     f_marg_list: f_marg
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>1 ]);
                    }
                | f_marg_list tCOMMA f_marg
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::new_node_list(nodes);
                    }
                ;

         f_margs: f_marg_list
                    {
                        $$ = $1;
                    }
                | f_marg_list tCOMMA f_rest_marg
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::new_node_list(nodes);
                    }
                | f_marg_list tCOMMA f_rest_marg tCOMMA f_marg_list
                    {
                        let mut nodes = $<NodeList>1;
                        let f_rest_marg = $<Node>3;
                        let mut f_marg_list = $<NodeList>5;

                        nodes.reserve(1 + f_marg_list.len());
                        nodes.push(f_rest_marg);
                        nodes.append(&mut f_marg_list);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_rest_marg
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>1 ]);
                    }
                | f_rest_marg tCOMMA f_marg_list
                    {
                        let mut nodes;
                        let f_rest_marg = $<Node>1;
                        let mut f_marg_list = $<NodeList>3;

                        nodes = Vec::with_capacity_in(1 + f_marg_list.len(), self.bump);
                        nodes.push(f_rest_marg);
                        nodes.append(&mut f_marg_list);

                        $$ = Value::new_node_list(nodes);
                    }
                ;

     f_rest_marg: tSTAR f_norm_arg
                    {
                        $$ = Value::new_node(
                            self.builder.restarg($<Token>1, Maybe::some($<Token>2))?
                        );
                    }
                | tSTAR
                    {
                        $$ = Value::new_node(
                            self.builder.restarg($<Token>1, Maybe::none())?
                        );
                    }
                ;

    f_any_kwrest: f_kwrest
                    {
                        $$ = $1;
                    }
                | f_no_kwarg
                    {
                        $$ = $1;
                    }
                ;

 block_args_tail: f_block_kwarg tCOMMA f_kwrest opt_f_block_arg
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_kwrest = $<NodeList>3;
                        let mut opt_f_block_arg = $<NodeList>4;

                        nodes.reserve(f_kwrest.len() + opt_f_block_arg.len());
                        nodes.append(&mut f_kwrest);
                        nodes.append(&mut opt_f_block_arg);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_block_kwarg opt_f_block_arg
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.append(&mut $<NodeList>2);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_any_kwrest opt_f_block_arg
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.append(&mut $<NodeList>2);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_block_arg
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>1 ]);
                    }
                ;

opt_block_args_tail:
                  tCOMMA block_args_tail
                    {
                        $$ = $2;
                    }
                | /* none */
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump;]);
                    }
                ;

  excessed_comma: tCOMMA
                    {
                        $$ = $1;
                    }
                ;

     block_param: f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg opt_block_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_block_optarg = $<NodeList>3;
                        let mut f_rest_arg = $<NodeList>5;
                        let mut opt_block_args_tail = $<NodeList>6;

                        nodes.reserve(f_block_optarg.len() + f_rest_arg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_block_optarg);
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut opt_block_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_block_optarg = $<NodeList>3;
                        let mut f_rest_arg = $<NodeList>5;
                        let mut f_arg = $<NodeList>7;
                        let mut opt_block_args_tail = $<NodeList>8;

                        nodes.reserve(f_block_optarg.len() + f_rest_arg.len() + f_arg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_block_optarg);
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_block_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_arg tCOMMA f_block_optarg opt_block_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_block_optarg = $<NodeList>3;
                        let mut opt_block_args_tail = $<NodeList>4;

                        nodes.reserve(f_block_optarg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_block_optarg);
                        nodes.append(&mut opt_block_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_arg tCOMMA f_block_optarg tCOMMA f_arg opt_block_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_block_optarg = $<NodeList>3;
                        let mut f_arg = $<NodeList>5;
                        let mut opt_block_args_tail = $<NodeList>6;

                        nodes.reserve(f_block_optarg.len() + f_arg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_block_optarg);
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_block_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_arg tCOMMA f_rest_arg opt_block_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_rest_arg = $<NodeList>3;
                        let mut opt_block_args_tail = $<NodeList>4;

                        nodes.reserve(f_rest_arg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut opt_block_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_arg excessed_comma
                    {
                        $$ = $1;
                    }
                | f_arg tCOMMA f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_rest_arg = $<NodeList>3;
                        let mut f_arg = $<NodeList>5;
                        let mut opt_block_args_tail = $<NodeList>6;

                        nodes.reserve(f_rest_arg.len() + f_arg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_block_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_arg opt_block_args_tail
                    {
                        let f_arg = $<NodeList>1;
                        let mut opt_block_args_tail = $<NodeList>2;
                        let mut nodes;

                        if opt_block_args_tail.is_empty() && f_arg.len() == 1 {
                            let procarg0 = self.builder.procarg0(
                                f_arg.take_first()
                            );
                            nodes = bump_vec![in self.bump; procarg0 ];
                        } else {
                            nodes = f_arg;
                            nodes.append(&mut opt_block_args_tail);
                        }

                        $$ = Value::new_node_list(nodes);
                    }
                | f_block_optarg tCOMMA f_rest_arg opt_block_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_rest_arg = $<NodeList>3;
                        let mut opt_block_args_tail = $<NodeList>4;

                        nodes.reserve(f_rest_arg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut opt_block_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_block_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_rest_arg = $<NodeList>3;
                        let mut f_arg = $<NodeList>5;
                        let mut opt_block_args_tail = $<NodeList>6;

                        nodes.reserve(f_rest_arg.len() + f_arg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_block_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_block_optarg opt_block_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.append(&mut $<NodeList>2);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_block_optarg tCOMMA f_arg opt_block_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_arg = $<NodeList>3;
                        let mut opt_block_args_tail = $<NodeList>4;

                        nodes.reserve(f_arg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_block_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_rest_arg opt_block_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.append(&mut $<NodeList>2);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_arg = $<NodeList>3;
                        let mut opt_block_args_tail = $<NodeList>4;

                        nodes.reserve(f_arg.len() + opt_block_args_tail.len());
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_block_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | block_args_tail
                    {
                        $$ = $1;
                    }
                ;

 opt_block_param: none
                    {
                        $$ = Value::new_maybe_node(
                            self.builder.args(Maybe::none(), bump_vec![in self.bump;], Maybe::none())
                        );
                    }
                | block_param_def
                    {
                        self.yylexer.command_start = true;
                        $$ = $1;
                    }
                ;

 block_param_def: tPIPE opt_bv_decl tPIPE
                    {
                        self.max_numparam_stack.set_has_ordinary_params();
                        self.current_arg_stack.set(None);

                        $$ = Value::new_maybe_node(
                            self.builder.args(
                                Maybe::some($<Token>1),
                                $<NodeList>2,
                                Maybe::some($<Token>3)
                            )
                        );
                    }
                | tPIPE block_param opt_bv_decl tPIPE
                    {
                        self.max_numparam_stack.set_has_ordinary_params();
                        self.current_arg_stack.set(None);

                        let mut nodes = $<NodeList>2;
                        nodes.append(&mut $<NodeList>3);

                        $$ = Value::new_maybe_node(
                            self.builder.args(
                                Maybe::some($<Token>1),
                                nodes,
                                Maybe::some($<Token>4)
                            )
                        );
                    }
                ;


     opt_bv_decl: opt_nl
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump;]);
                    }
                | opt_nl tSEMI bv_decls opt_nl
                    {
                        $$ = $3;
                    }
                ;

        bv_decls: bvar
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>1 ]);
                    }
                | bv_decls tCOMMA bvar
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::new_node_list(nodes);
                    }
                ;

            bvar: tIDENTIFIER
                    {
                        let ident_t = $<Token>1;
                        self.static_env.declare(clone_value(&ident_t).as_str());
                        $$ = Value::new_node(
                            self.builder.shadowarg(ident_t)?
                        );
                    }
                | f_bad_arg
                    {
                        $$ = Value::new_none();
                    }
                ;

          lambda: tLAMBDA
                    {
                        self.static_env.extend_dynamic();
                        self.max_numparam_stack.push();
                        self.context.push_lambda();
                        $<Num>$ = Value::new_num( self.yylexer.lpar_beg);
                        self.yylexer.lpar_beg = self.yylexer.paren_nest;
                    }
                  f_larglist
                    {
                        self.context.pop();
                        self.yylexer.cmdarg.push(false);
                        $<None>$ = Value::new_none();
                    }
                  lambda_body
                    {
                        self.yylexer.lpar_beg = $<Num>2;

                        let lambda_call = self.builder.call_lambda($<Token>1);
                        let args = if self.max_numparam_stack.has_numparams() {
                            ArgsType::Numargs(self.max_numparam_stack.top() as u8)
                        } else {
                            ArgsType::Args($<MaybeBoxedNode>3)
                        };
                        let LambdaBody { begin_t, body, end_t } = *$<LambdaBody>5;

                        self.max_numparam_stack.pop();
                        self.static_env.unextend();
                        self.yylexer.cmdarg.pop();

                        $$ = Value::new_node(
                            self.builder.block(
                                lambda_call,
                                begin_t,
                                args,
                                body,
                                end_t
                            )?
                        );
                    }
                ;

      f_larglist: tLPAREN2 f_args opt_bv_decl tRPAREN
                    {
                        self.max_numparam_stack.set_has_ordinary_params();

                        let mut nodes = $<NodeList>2;
                        nodes.append(&mut $<NodeList>3);

                        $$ = Value::new_maybe_node(
                            self.builder.args(
                                Maybe::some($<Token>1),
                                nodes,
                                Maybe::some($<Token>4)
                            )
                        );
                    }
                | f_args
                    {
                        let args = $<NodeList>1;
                        if !args.is_empty() {
                            self.max_numparam_stack.set_has_ordinary_params();
                        }
                        $$ = Value::new_maybe_node(
                            self.builder.args(Maybe::none(), args, Maybe::none())
                        );
                    }
                ;

     lambda_body: tLAMBEG
                    {
                        self.context.push_lambda();
                        $<None>$ = Value::new_none();
                    }
                  compstmt tRCURLY
                    {
                        self.context.pop();
                        $$ = Value::new_lambda_body(
                            self.bump.alloc(
                                LambdaBody {
                                    begin_t: $<Token>1,
                                    body: $<MaybeBoxedNode>3,
                                    end_t: $<Token>4
                                }
                            )
                        );
                    }
                | kDO_LAMBDA
                    {
                        self.context.push_lambda();
                        $<None>$ = Value::new_none();
                    }
                  bodystmt k_end
                    {
                        self.context.pop();
                        $$ = Value::new_lambda_body(
                            self.bump.alloc(
                                LambdaBody {
                                    begin_t: $<Token>1,
                                    body: $<MaybeBoxedNode>3,
                                    end_t: $<Token>4
                                }
                            )
                        );
                    }
                ;

        do_block: k_do_block
                    {
                        self.context.push_block();
                        $<None>$ = Value::new_none();
                    }
                  do_body k_end
                    {
                        let DoBody { args_type, body } = *$<DoBody>3;
                        self.context.pop();
                        $$ = Value::new_do_block(
                            self.bump.alloc(
                                DoBlock {
                                    begin_t: $<Token>1,
                                    args_type,
                                    body,
                                    end_t: $<Token>4
                                }
                            )
                        );
                    }
                ;

      block_call: command do_block
                    {
                        let DoBlock { begin_t, args_type, body, end_t } = *$<DoBlock>2;
                        $$ = Value::new_node(
                            self.builder.block(
                                $<BoxedNode>1,
                                begin_t,
                                args_type,
                                body,
                                end_t
                            )?
                        );
                    }
                | block_call call_op2 operation2 opt_paren_args
                    {
                        let OptParenArgs { begin_t, args, end_t } = $<OptParenArgs>4;
                        $$ = Value::new_node(
                            self.builder.call_method(
                                Maybe::some($<BoxedNode>1),
                                Maybe::some($<Token>2),
                                Maybe::some($<Token>3),
                                *begin_t,
                                take_vec(args),
                                *end_t
                            )
                        );
                    }
                | block_call call_op2 operation2 opt_paren_args brace_block
                    {
                        let OptParenArgs { begin_t, args, end_t } = $<OptParenArgs>4;
                        let method_call = self.builder.call_method(
                            Maybe::some($<BoxedNode>1),
                            Maybe::some($<Token>2),
                            Maybe::some($<Token>3),
                            *begin_t,
                            take_vec(args),
                            *end_t
                        );

                        let BraceBlock { begin_t, args_type, body, end_t } = *$<BraceBlock>5;
                        $$ = Value::new_node(
                            self.builder.block(
                                method_call,
                                begin_t,
                                args_type,
                                body,
                                end_t
                            )?
                        );
                    }
                | block_call call_op2 operation2 command_args do_block
                    {
                        let method_call = self.builder.call_method(
                            Maybe::some($<BoxedNode>1),
                            Maybe::some($<Token>2),
                            Maybe::some($<Token>3),
                            Maybe::none(),
                            $<NodeList>4,
                            Maybe::none()
                        );

                        let DoBlock { begin_t, args_type, body, end_t } = *$<DoBlock>5;
                        $$ = Value::new_node(
                            self.builder.block(
                                method_call,
                                begin_t,
                                args_type,
                                body,
                                end_t
                            )?
                        );
                    }
                ;

     method_call: fcall paren_args
                    {
                        let ParenArgs { begin_t, args, end_t } = $<ParenArgs>2;

                        $$ = Value::new_node(
                            self.builder.call_method(
                                Maybe::none(),
                                Maybe::none(),
                                Maybe::some($<Token>1),
                                Maybe::some(begin_t),
                                take_vec(args),
                                Maybe::some(end_t)
                            )
                        );
                    }
                | primary_value call_op operation2 opt_paren_args
                    {
                        let OptParenArgs { begin_t, args, end_t } = $<OptParenArgs>4;

                        $$ = Value::new_node(
                            self.builder.call_method(
                                Maybe::some($<BoxedNode>1),
                                Maybe::some($<Token>2),
                                Maybe::some($<Token>3),
                                *begin_t,
                                take_vec(args),
                                *end_t
                            )
                        );
                    }
                | primary_value tCOLON2 operation2 paren_args
                    {
                        let ParenArgs { begin_t, args, end_t } = $<ParenArgs>4;

                        $$ = Value::new_node(
                            self.builder.call_method(
                                Maybe::some($<BoxedNode>1),
                                Maybe::some($<Token>2),
                                Maybe::some($<Token>3),
                                Maybe::some(begin_t),
                                take_vec(args),
                                Maybe::some(end_t)
                            )
                        );
                    }
                | primary_value tCOLON2 operation3
                    {
                        $$ = Value::new_node(
                            self.builder.call_method(
                                Maybe::some($<BoxedNode>1),
                                Maybe::some($<Token>2),
                                Maybe::some($<Token>3),
                                Maybe::none(),
                                bump_vec![in self.bump;],
                                Maybe::none()
                            )
                        );
                    }
                | primary_value call_op paren_args
                    {
                        let ParenArgs { begin_t, args, end_t } = $<ParenArgs>3;

                        $$ = Value::new_node(
                            self.builder.call_method(
                                Maybe::some($<BoxedNode>1),
                                Maybe::some($<Token>2),
                                Maybe::none(),
                                Maybe::some(begin_t),
                                take_vec(args),
                                Maybe::some(end_t)
                            )
                        );
                    }
                | primary_value tCOLON2 paren_args
                    {
                        let ParenArgs { begin_t, args, end_t } = $<ParenArgs>3;

                        $$ = Value::new_node(
                            self.builder.call_method(
                                Maybe::some($<BoxedNode>1),
                                Maybe::some($<Token>2),
                                Maybe::none(),
                                Maybe::some(begin_t),
                                take_vec(args),
                                Maybe::some(end_t)
                            )
                        );
                    }
                | kSUPER paren_args
                    {
                        let ParenArgs { begin_t, args, end_t } = $<ParenArgs>2;

                        $$ = Value::new_node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Super,
                                $<Token>1,
                                Maybe::some(begin_t),
                                take_vec(args),
                                Maybe::some(end_t)
                            )?
                        );
                    }
                | kSUPER
                    {
                        $$ = Value::new_node(
                            self.builder.keyword_cmd(
                                KeywordCmd::Zsuper,
                                $<Token>1,
                                Maybe::none(),
                                bump_vec![in self.bump;],
                                Maybe::none()
                            )?
                        );
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                        $$ = Value::new_node(
                            self.builder.index(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<NodeList>3,
                                $<Token>4
                            )
                        );
                    }
                ;

     brace_block: tLCURLY
                    {
                        self.context.push_block();
                        $<None>$ = Value::new_none();
                    }
                  brace_body tRCURLY
                    {
                        let BraceBody { args_type, body } = *$<BraceBody>3;
                        self.context.pop();

                        $$ = Value::new_brace_block(
                            self.bump.alloc(
                                BraceBlock {
                                    begin_t: $<Token>1,
                                    args_type,
                                    body,
                                    end_t: $<Token>4
                                }
                            )
                        );
                    }
                | k_do
                    {
                        self.context.push_block();
                        $<None>$ = Value::new_none();
                    }
                  do_body k_end
                    {
                        let DoBody { args_type, body } = *$<DoBody>3;
                        self.context.pop();

                        $$ = Value::new_brace_block(
                            self.bump.alloc(
                                BraceBlock {
                                    begin_t: $<Token>1,
                                    args_type,
                                    body,
                                    end_t: $<Token>4
                                }
                            )
                        );
                    }
                ;

      brace_body:   {
                        self.static_env.extend_dynamic();
                        self.max_numparam_stack.push();
                        $<None>$ = Value::new_none();
                    }
                  opt_block_param compstmt
                    {
                        let args_type = if self.max_numparam_stack.has_numparams() {
                            ArgsType::Numargs(self.max_numparam_stack.top() as u8)
                        } else {
                            ArgsType::Args($<MaybeBoxedNode>2)
                        };

                        self.max_numparam_stack.pop();
                        self.static_env.unextend();

                        $$ = Value::new_brace_body(
                            self.bump.alloc(
                                BraceBody {
                                    args_type,
                                    body: $<MaybeBoxedNode>3
                                }
                            )
                        );
                    }
                ;

         do_body:   {
                        self.static_env.extend_dynamic();
                        self.max_numparam_stack.push();
                        self.yylexer.cmdarg.push(false);
                        $<None>$ = Value::new_none();
                    }
                  opt_block_param bodystmt
                    {
                        let args_type = if self.max_numparam_stack.has_numparams() {
                            ArgsType::Numargs(self.max_numparam_stack.top() as u8)
                        } else {
                            ArgsType::Args($<MaybeBoxedNode>2)
                        };

                        self.max_numparam_stack.pop();
                        self.static_env.unextend();
                        self.yylexer.cmdarg.pop();

                        $$ = Value::new_do_body(
                            self.bump.alloc(
                                DoBody { args_type, body: $<MaybeBoxedNode>3 }
                            )
                        );
                    }
                ;

       case_args: arg_value
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>1 ]);
                    }
                | tSTAR arg_value
                    {
                        $$ = Value::new_node_list(
                                bump_vec![in self.bump;
                                    self.builder.splat(
                                        $<Token>1,
                                        Maybe::some($<BoxedNode>2)
                                    )
                                ]
                        );
                    }
                | case_args tCOMMA arg_value
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::new_node_list(nodes);
                    }
                | case_args tCOMMA tSTAR arg_value
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( self.builder.splat($<Token>3, Maybe::some($<BoxedNode>4)) );
                        $$ = Value::new_node_list(nodes);
                    }
                ;

       case_body: k_when case_args then
                  compstmt
                  cases
                    {
                        let when = self.builder.when($<Token>1, $<NodeList>2, $<Token>3, $<MaybeBoxedNode>4);
                        let Cases { when_bodies, opt_else } = $<Cases>5;
                        let mut when_bodies = take_vec(when_bodies);

                        let mut nodes = Vec::with_capacity_in(1 + when_bodies.len(), self.bump);
                        nodes.push(when);
                        nodes.append(&mut when_bodies);

                        $$ = Value::new_case_body(
                            self.bump.alloc(
                                CaseBody { when_bodies: nodes, opt_else: *opt_else }
                            )
                        );
                    }
                ;

           cases: opt_else
                    {
                        $$ = Value::new_cases(
                            self.bump.alloc(
                                Cases { when_bodies: bump_vec![in self.bump;], opt_else: $<OptElse>1 }
                            )
                        );
                    }
                | case_body
                    {
                        let CaseBody { when_bodies, .. } = $<CaseBody>1;
                        $$ = Value::new_cases(
                            self.bump.alloc(
                                Cases { when_bodies: take_vec(when_bodies), opt_else: None }
                            )
                        );
                    }
                ;

     p_case_body: kIN
                    {
                        self.yylexer.lex_state.set(EXPR_BEG|EXPR_LABEL);
                        self.yylexer.command_start = false;
                        self.pattern_variables.push();
                        self.pattern_hash_keys.push();

                        $<Bool>$ = Value::new_bool(self.yylexer.in_kwarg);
                        self.yylexer.in_kwarg = true;
                    }
                  p_top_expr then
                    {
                        self.yylexer.in_kwarg = $<Bool>2;
                        self.pattern_variables.pop();
                        self.pattern_hash_keys.pop();
                        $<None>$ = Value::new_none();
                    }
                  compstmt
                  p_cases
                    {
                        let PCases { in_bodies, opt_else } = $<PCases>7;
                        let mut in_bodies = take_vec(in_bodies);
                        let PTopExpr { pattern, guard } = *$<PTopExpr>3;

                        let mut nodes = Vec::with_capacity_in(1 + in_bodies.len(), self.bump);
                        nodes.push(
                            self.builder.in_pattern(
                                $<Token>1,
                                pattern,
                                guard,
                                $<Token>4,
                                $<MaybeBoxedNode>6
                            )
                        );
                        nodes.append(&mut in_bodies);

                        $$ = Value::new_p_case_body(
                            self.bump.alloc(
                                PCaseBody { in_bodies: nodes, opt_else: *opt_else  }
                            )
                        );
                    }
                ;

         p_cases: opt_else
                    {
                        $$ = Value::new_p_cases(
                            self.bump.alloc(
                                PCases { in_bodies: bump_vec![in self.bump;], opt_else: $<OptElse>1 }
                            )
                        );
                    }
                | p_case_body
                    {
                        let PCaseBody { in_bodies, .. } = $<PCaseBody>1;
                        $$ = Value::new_p_cases(
                            self.bump.alloc(
                                PCases { in_bodies: take_vec(in_bodies), opt_else: None }
                            )
                        );
                    }
                ;

      p_top_expr: p_top_expr_body
                    {
                        $$ = Value::new_p_top_expr(
                            self.bump.alloc(
                                PTopExpr { pattern: $<BoxedNode>1, guard: Maybe::none() }
                            )
                        );
                    }
                | p_top_expr_body kIF_MOD expr_value
                    {
                        let guard = self.builder.if_guard($<Token>2, $<BoxedNode>3);
                        $$ = Value::new_p_top_expr(
                            self.bump.alloc(
                                PTopExpr { pattern: $<BoxedNode>1, guard: Maybe::some(guard) }
                            )
                        );
                    }
                | p_top_expr_body kUNLESS_MOD expr_value
                    {
                        let guard = self.builder.unless_guard($<Token>2, $<BoxedNode>3);
                        $$ = Value::new_p_top_expr(
                            self.bump.alloc(
                                PTopExpr { pattern: $<BoxedNode>1, guard: Maybe::some(guard) }
                            )
                        );
                    }
                ;

 p_top_expr_body: p_expr
                    {
                        $$ = $1;
                    }
                | p_expr tCOMMA
                    {
                        $$ = Value::new_node(
                            self.builder.array_pattern(
                                Maybe::none(),
                                bump_vec![in self.bump; $<Node>1 ],
                                Maybe::some($<Token>2),
                                Maybe::none()
                            )
                        );
                    }
                | p_expr tCOMMA p_args
                    {
                        let MatchPatternWithTrailingComma { elements, trailing_comma } = $<MatchPatternWithTrailingComma>3;
                        let mut elements = take_vec(elements);

                        let mut nodes = Vec::with_capacity_in(1 + elements.len(), self.bump);
                        nodes.push($<Node>1);
                        nodes.append(&mut elements);

                        $$ = Value::new_node(
                            self.builder.array_pattern(Maybe::none(), nodes, take_maybe_token(trailing_comma), Maybe::none())
                        );
                    }
                | p_find
                    {
                        $$ = Value::new_node(
                            self.builder.find_pattern(Maybe::none(), $<NodeList>1, Maybe::none())
                        );
                    }
                | p_args_tail
                    {
                        $$ = Value::new_node(
                            self.builder.array_pattern(Maybe::none(), $<NodeList>1, Maybe::none(), Maybe::none())
                        );
                    }
                | p_kwargs
                    {
                        $$ = Value::new_node(
                            self.builder.hash_pattern(Maybe::none(), $<NodeList>1, Maybe::none())
                        );
                    }
                ;

          p_expr: p_as
                    {
                        $$ = $1;
                    }
                ;

            p_as: p_expr tASSOC p_variable
                    {
                        $$ = Value::new_node(
                            self.builder.match_as(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>3
                            )
                        );
                    }
                | p_alt
                    {
                        $$ = $1;
                    }
                ;

           p_alt: p_alt tPIPE p_expr_basic
                    {
                        $$ = Value::new_node(
                            self.builder.match_alt(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>3
                            )
                        );
                    }
                | p_expr_basic
                    {
                        $$ = $1;
                    }
                ;

        p_lparen: tLPAREN2
                    {
                        $$ = $1;
                        self.pattern_hash_keys.push();
                    }
                ;

      p_lbracket: tLBRACK2
                    {
                        $$ = $1;
                        self.pattern_hash_keys.push();
                    }
                ;

    p_expr_basic: p_value
                    {
                        $$ = $1;
                    }
                | p_const p_lparen p_args rparen
                    {
                        self.pattern_hash_keys.pop();
                        let MatchPatternWithTrailingComma { elements, trailing_comma } = $<MatchPatternWithTrailingComma>3;
                        let pattern = self.builder.array_pattern(Maybe::none(), take_vec(elements), *trailing_comma, Maybe::none());
                        $$ = Value::new_node(
                            self.builder.const_pattern(
                                $<BoxedNode>1,
                                $<Token>2,
                                pattern,
                                $<Token>4
                            )
                        );
                    }
                | p_const p_lparen p_find rparen
                    {
                        self.pattern_hash_keys.pop();
                        let pattern = self.builder.find_pattern(Maybe::none(), $<NodeList>3, Maybe::none());
                        $$ = Value::new_node(
                            self.builder.const_pattern(
                                $<BoxedNode>1,
                                $<Token>2,
                                pattern,
                                $<Token>4
                            )
                        );
                    }
                | p_const p_lparen p_kwargs rparen
                    {
                        self.pattern_hash_keys.pop();
                        let pattern = self.builder.hash_pattern(Maybe::none(), $<NodeList>3, Maybe::none());
                        $$ = Value::new_node(
                            self.builder.const_pattern(
                                $<BoxedNode>1,
                                $<Token>2,
                                pattern,
                                $<Token>4
                            )
                        );
                    }
                | p_const tLPAREN2 rparen
                    {
                        let lparen = $<Token>2;
                        let rparen = $<Token>3;
                        let pattern = self.builder.array_pattern(
                            Maybe::some(lparen),
                            bump_vec![in self.bump;],
                            Maybe::none(),
                            Maybe::some(rparen)
                        );
                        $$ = Value::new_node(
                            self.builder.const_pattern(
                                $<BoxedNode>1,
                                lparen,
                                pattern,
                                rparen
                            )
                        );
                    }
                | p_const p_lbracket p_args rbracket
                    {
                        self.pattern_hash_keys.pop();
                        let MatchPatternWithTrailingComma { elements, trailing_comma } = $<MatchPatternWithTrailingComma>3;
                        let pattern = self.builder.array_pattern(Maybe::none(), take_vec(elements), *trailing_comma, Maybe::none());
                        $$ = Value::new_node(
                            self.builder.const_pattern(
                                $<BoxedNode>1,
                                $<Token>2,
                                pattern,
                                $<Token>4
                            )
                        );
                    }
                | p_const p_lbracket p_find rbracket
                    {
                        self.pattern_hash_keys.pop();
                        let pattern = self.builder.find_pattern(Maybe::none(), $<NodeList>3, Maybe::none());
                        $$ = Value::new_node(
                            self.builder.const_pattern(
                                $<BoxedNode>1,
                                $<Token>2,
                                pattern,
                                $<Token>4
                            )
                        );
                    }
                | p_const p_lbracket p_kwargs rbracket
                    {
                        self.pattern_hash_keys.pop();
                        let pattern = self.builder.hash_pattern(Maybe::none(), $<NodeList>3, Maybe::none());
                        $$ = Value::new_node(
                            self.builder.const_pattern(
                                $<BoxedNode>1,
                                $<Token>2,
                                pattern,
                                $<Token>4
                            )
                        );
                    }
                | p_const tLBRACK2 rbracket
                    {
                        let lparen = $<Token>2;
                        let rparen = $<Token>3;
                        let pattern = self.builder.array_pattern(
                            Maybe::some(lparen),
                            bump_vec![in self.bump;],
                            Maybe::none(),
                            Maybe::some(rparen)
                        );
                        $$ = Value::new_node(
                            self.builder.const_pattern(
                                $<BoxedNode>1,
                                lparen,
                                pattern,
                                rparen
                            )
                        );
                    }
                | tLBRACK p_args rbracket
                    {
                        let MatchPatternWithTrailingComma { elements, trailing_comma } = $<MatchPatternWithTrailingComma>2;
                        $$ = Value::new_node(
                            self.builder.array_pattern(
                                Maybe::some($<Token>1),
                                take_vec(elements),
                                *trailing_comma,
                                Maybe::some($<Token>3)
                            )
                        );
                    }
                | tLBRACK p_find rbracket
                    {
                        $$ = Value::new_node(
                            self.builder.find_pattern(
                                Maybe::some($<Token>1),
                                $<NodeList>2,
                                Maybe::some($<Token>3)
                            )
                        );
                    }
                | tLBRACK rbracket
                    {
                        $$ = Value::new_node(
                            self.builder.array_pattern(
                                Maybe::some($<Token>1),
                                bump_vec![in self.bump;],
                                Maybe::none(),
                                Maybe::some($<Token>2)
                            )
                        );
                    }
                | tLBRACE
                    {
                        self.pattern_hash_keys.push();
                        $<Bool>$ = Value::new_bool(self.yylexer.in_kwarg);
                        self.yylexer.in_kwarg = false;
                    }
                  p_kwargs rbrace
                    {
                        self.pattern_hash_keys.pop();
                        self.yylexer.in_kwarg = $<Bool>2;
                        $$ = Value::new_node(
                            self.builder.hash_pattern(
                                Maybe::some($<Token>1),
                                $<NodeList>3,
                                Maybe::some($<Token>4)
                            )
                        );
                    }
                | tLBRACE rbrace
                    {
                        $$ = Value::new_node(
                            self.builder.hash_pattern(
                                Maybe::some($<Token>1),
                                bump_vec![in self.bump;],
                                Maybe::some($<Token>2),
                            )
                        );
                    }
                | tLPAREN
                    {
                        self.pattern_hash_keys.push();
                        $<None>$ = Value::new_none();
                    }
                  p_expr rparen
                    {
                        self.pattern_hash_keys.pop();
                        $$ = Value::new_node(
                            self.builder.begin(
                                $<Token>1,
                                Maybe::some($<BoxedNode>3),
                                $<Token>4
                            )
                        );
                    }
                ;

          p_args: p_expr
                    {
                        $$ = Value::new_match_pattern_with_trailing_comma(
                            self.bump.alloc(
                                MatchPatternWithTrailingComma {
                                    elements: bump_vec![in self.bump; $<Node>1 ],
                                    trailing_comma: Maybe::none()
                                }
                            )
                        );
                    }
                | p_args_head
                    {
                        $$ = $1;
                    }
                | p_args_head p_arg
                    {
                        let mut elements = take_vec(& $<MatchPatternWithTrailingComma>1.elements);
                        elements.push($<Node>2);

                        $$ = Value::new_match_pattern_with_trailing_comma(
                            self.bump.alloc(
                                MatchPatternWithTrailingComma {
                                    elements,
                                    trailing_comma: Maybe::none()
                                }
                            )
                        );
                    }
                | p_args_head tSTAR tIDENTIFIER
                    {
                        let match_rest = self.builder.match_rest($<Token>2, Maybe::some($<Token>3))?;

                        let mut elements = take_vec(& $<MatchPatternWithTrailingComma>1.elements);
                        elements.push(match_rest);

                        $$ = Value::new_match_pattern_with_trailing_comma(
                            self.bump.alloc(
                                MatchPatternWithTrailingComma {
                                    elements,
                                    trailing_comma: Maybe::none()
                                }
                            )
                        );
                    }
                | p_args_head tSTAR tIDENTIFIER tCOMMA p_args_post
                    {
                        let match_rest = self.builder.match_rest($<Token>2, Maybe::some($<Token>3))?;

                        let mut elements = take_vec(& $<MatchPatternWithTrailingComma>1.elements);
                        let mut p_args_post = $<NodeList>5;
                        elements.reserve(1 + p_args_post.len());
                        elements.push(match_rest);
                        elements.append(&mut p_args_post);

                        $$ = Value::new_match_pattern_with_trailing_comma(
                            self.bump.alloc(
                                MatchPatternWithTrailingComma {
                                    elements,
                                    trailing_comma: Maybe::none()
                                }
                            )
                        );
                    }
                | p_args_head tSTAR
                    {
                        let match_rest = self.builder.match_rest($<Token>2, Maybe::none())?;

                        let mut elements = take_vec(& $<MatchPatternWithTrailingComma>1.elements);
                        elements.push(match_rest);

                        $$ = Value::new_match_pattern_with_trailing_comma(
                            self.bump.alloc(
                                MatchPatternWithTrailingComma {
                                    elements,
                                    trailing_comma: Maybe::none()
                                }
                            )
                        );
                    }
                | p_args_head tSTAR tCOMMA p_args_post
                    {
                        let match_rest = self.builder.match_rest($<Token>2, Maybe::none())?;

                        let mut elements = take_vec(& $<MatchPatternWithTrailingComma>1.elements);
                        let mut p_args_post = $<NodeList>4;
                        elements.push(match_rest);
                        elements.append(&mut p_args_post);

                        $$ = Value::new_match_pattern_with_trailing_comma(
                            self.bump.alloc(
                                MatchPatternWithTrailingComma {
                                    elements,
                                    trailing_comma: Maybe::none()
                                }
                            )
                        );
                    }
                | p_args_tail
                    {
                        $$ = Value::new_match_pattern_with_trailing_comma(
                            self.bump.alloc(
                                MatchPatternWithTrailingComma {
                                    elements: $<NodeList>1,
                                    trailing_comma: Maybe::none()
                                }
                            )
                        );
                    }
                ;

     p_args_head: p_arg tCOMMA
                    {
                        $$ = Value::new_match_pattern_with_trailing_comma(
                            self.bump.alloc(
                                MatchPatternWithTrailingComma {
                                    elements: bump_vec![in self.bump;$<Node>1],
                                    trailing_comma: Maybe::some($<Token>2),
                                }
                            )
                        );
                    }
                | p_args_head p_arg tCOMMA
                    {
                        let mut elements = take_vec(& $<MatchPatternWithTrailingComma>1.elements);
                        elements.push($<Node>2);

                        $$ = Value::new_match_pattern_with_trailing_comma(
                            self.bump.alloc(
                                MatchPatternWithTrailingComma {
                                    elements,
                                    trailing_comma: Maybe::some($<Token>3),
                                }
                            )
                        );
                    }
                ;

     p_args_tail: p_rest
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>1 ]);
                    }
                | p_rest tCOMMA p_args_post
                    {
                        let mut nodes;
                        let mut p_args_post = $<NodeList>3;
                        nodes = Vec::with_capacity_in(1 + p_args_post.len(), self.bump);
                        nodes.push($<Node>1);
                        nodes.append(&mut p_args_post);

                        $$ = Value::new_node_list(nodes);
                    }
                ;

          p_find: p_rest tCOMMA p_args_post tCOMMA p_rest
                    {
                        let mut nodes;
                        let mut p_args_post = $<NodeList>3;
                        nodes = Vec::with_capacity_in(1 + p_args_post.len() + 1, self.bump);
                        nodes.push($<Node>1);
                        nodes.append(&mut p_args_post);
                        nodes.push($<Node>5);

                        $$ = Value::new_node_list(nodes);
                    }
                ;


          p_rest: tSTAR tIDENTIFIER
                    {
                        $$ = Value::new_node(
                            self.builder.match_rest($<Token>1, Maybe::some($<Token>2))?
                        );
                    }
                | tSTAR
                    {
                        $$ = Value::new_node(
                            self.builder.match_rest($<Token>1, Maybe::none())?
                        );
                    }
                ;

     p_args_post: p_arg
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>1 ]);
                    }
                | p_args_post tCOMMA p_arg
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::new_node_list(nodes);
                    }
                ;

           p_arg: p_expr
                    {
                        $$ = $1;
                    }
                ;

        p_kwargs: p_kwarg tCOMMA p_any_kwrest
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.append(&mut $<NodeList>3);

                        $$ = Value::new_node_list(nodes);
                    }
                | p_kwarg
                    {
                        $$ = $1;
                    }
                | p_kwarg tCOMMA
                    {
                        $$ = $1;
                    }
                | p_any_kwrest
                    {
                        $$ = $1;
                    }
                ;

         p_kwarg: p_kw
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>1 ]);
                    }
                | p_kwarg tCOMMA p_kw
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::new_node_list(nodes);
                    }
                ;

            p_kw: p_kw_label p_expr
                    {
                        $$ = Value::new_node(
                            self.builder.match_pair(
                                $<PKwLabel>1,
                                $<BoxedNode>2
                            )?
                        );
                    }
                | p_kw_label
                    {
                        $$ = Value::new_node(
                            self.builder.match_label(
                                $<PKwLabel>1,
                            )?
                        );
                    }
                ;

      p_kw_label: tLABEL
                    {
                        $$ = Value::new_p_kw_label(
                            self.bump.alloc(
                                PKwLabel::PlainLabel($<Token>1)
                            )
                        );
                    }
                | tSTRING_BEG string_contents tLABEL_END
                    {
                        $$ = Value::new_p_kw_label(
                            self.bump.alloc(
                                PKwLabel::QuotedLabel( ($<Token>1, $<NodeList>2, $<Token>3) )
                            )
                        );
                    }
                ;

        p_kwrest: kwrest_mark tIDENTIFIER
                    {
                        $$ = Value::new_node_list(
                            bump_vec![in self.bump;
                                (
                                    self.builder.match_rest(
                                        $<Token>1,
                                        Maybe::some($<Token>2)
                                    )?
                                )
                            ]
                        );
                    }
                | kwrest_mark
                    {
                        $$ = Value::new_node_list(
                            bump_vec![in self.bump;
                                (
                                    self.builder.match_rest(
                                        $<Token>1,
                                        Maybe::none()
                                    )?
                                )
                            ]
                        );
                    }
                ;

      p_kwnorest: kwrest_mark kNIL
                    {
                        $$ = Value::new_node_list(
                            bump_vec![in self.bump;
                                self.builder.match_nil_pattern(
                                    $<Token>1,
                                    $<Token>2
                                )
                            ]
                        );
                    }
                ;

    p_any_kwrest: p_kwrest
                    {
                        $$ = $1;
                    }
                | p_kwnorest
                    {
                        $$ = $1;
                    }
                ;

         p_value: p_primitive
                    {
                        $$ = $1;
                    }
                | p_primitive tDOT2 p_primitive
                    {
                        let left = $<BoxedNode>1;
                        let left = self.value_expr(left)?;

                        let right = $<BoxedNode>3;
                        let right = self.value_expr(right)?;

                        $$ = Value::new_node(
                            self.builder.range_inclusive(
                                Maybe::some(left),
                                $<Token>2,
                                Maybe::some(right)
                            )
                        );
                    }
                | p_primitive tDOT3 p_primitive
                    {
                        let left = $<BoxedNode>1;
                        let left = self.value_expr(left)?;

                        let right = $<BoxedNode>3;
                        let right = self.value_expr(right)?;

                        $$ = Value::new_node(
                            self.builder.range_exclusive(
                                Maybe::some(left),
                                $<Token>2,
                                Maybe::some(right)
                            )
                        );
                    }
                | p_primitive tDOT2
                    {
                        let left = $<BoxedNode>1;
                        let left = self.value_expr(left)?;

                        $$ = Value::new_node(
                            self.builder.range_inclusive(
                                Maybe::some(left),
                                $<Token>2,
                                Maybe::none()
                            )
                        );
                    }
                | p_primitive tDOT3
                    {
                        let left = $<BoxedNode>1;
                        let left = self.value_expr(left)?;

                        $$ = Value::new_node(
                            self.builder.range_exclusive(
                                Maybe::some(left),
                                $<Token>2,
                                Maybe::none()
                            )
                        );
                    }
                | p_variable
                    {
                        $$ = $1;
                    }
                | p_var_ref
                    {
                        $$ = $1;
                    }
                | p_const
                    {
                        $$ = $1;
                    }
                | tBDOT2 p_primitive
                    {
                        let right = $<BoxedNode>2;
                        let right = self.value_expr(right)?;

                        $$ = Value::new_node(
                            self.builder.range_inclusive(
                                Maybe::none(),
                                $<Token>1,
                                Maybe::some(right)
                            )
                        );
                    }
                | tBDOT3 p_primitive
                    {
                        let right = $<BoxedNode>2;
                        let right = self.value_expr(right)?;

                        $$ = Value::new_node(
                            self.builder.range_exclusive(
                                Maybe::none(),
                                $<Token>1,
                                Maybe::some(right)
                            )
                        );
                    }
                ;

     p_primitive: literal
                    {
                        $$ = $1;
                    }
                | strings
                    {
                        $$ = $1;
                    }
                | xstring
                    {
                        $$ = $1;
                    }
                | regexp
                    {
                        $$ = $1;
                    }
                | words
                    {
                        $$ = $1;
                    }
                | qwords
                    {
                        $$ = $1;
                    }
                | symbols
                    {
                        $$ = $1;
                    }
                | qsymbols
                    {
                        $$ = $1;
                    }
                | keyword_variable
                    {
                        $$ = Value::new_node(
                            self.builder.accessible($<BoxedNode>1)
                        );
                    }
                | lambda
                    {
                        $$ = $1;
                    }
                ;

      p_variable: tIDENTIFIER
                    {
                        $$ = Value::new_node(
                            self.builder.match_var($<Token>1)?
                        );
                    }
                ;

       p_var_ref: tCARET tIDENTIFIER
                    {
                        let ident_t = $<Token>2;
                        let name = clone_value(&ident_t);

                        if !self.static_env.is_declared(name.as_str()) {
                            return self.yyerror(@2, DiagnosticMessage::new_no_such_local_variable(name));
                        }

                        let lvar = self.builder.accessible(self.builder.lvar(ident_t));
                        $$ = Value::new_node(
                            self.builder.pin($<Token>1, lvar)
                        );
                    }
                ;

         p_const: tCOLON3 cname
                    {
                        $$ = Value::new_node(
                            self.builder.const_global($<Token>1, $<Token>2)
                        );
                    }
                | p_const tCOLON2 cname
                    {
                        $$ = Value::new_node(
                            self.builder.const_fetch(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<Token>3,
                            )
                        );
                    }
                | tCONSTANT
                    {
                        $$ = Value::new_node(self.builder.const_($<Token>1));
                    }
                ;

      opt_rescue: k_rescue exc_list exc_var then
                  compstmt
                  opt_rescue
                    {
                        let ExcVar { assoc_t, exc_var } = *$<ExcVar>3;

                        let exc_list = $<NodeList>2;
                        let exc_list = if exc_list.is_empty() {
                            Maybe::none()
                        } else {
                            Maybe::some(self.builder.array(Maybe::none(), exc_list, Maybe::none()))
                        };

                        let rescue_body = self.builder.rescue_body(
                            $<Token>1,
                            exc_list,
                            assoc_t,
                            exc_var,
                            Maybe::some($<Token>4),
                            $<MaybeBoxedNode>5
                        );
                        let mut nodes;
                        let mut opt_rescue = $<NodeList>6;
                        nodes = Vec::with_capacity_in(1 + opt_rescue.len(), self.bump);
                        nodes.push(rescue_body);
                        nodes.append(&mut opt_rescue);

                        $$ = Value::new_node_list(nodes);
                    }
                | none
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump;]);
                    }
                ;

        exc_list: arg_value
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>1 ]);
                    }
                | mrhs
                    {
                        $$ = $1;
                    }
                | none
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump;]);
                    }
                ;

         exc_var: tASSOC lhs
                    {
                        let assoc_t = Maybe::some($<Token>1);
                        let exc_var = Maybe::some($<BoxedNode>2);
                        $$ = Value::new_exc_var(
                            self.bump.alloc(
                                ExcVar { assoc_t, exc_var }
                            )
                        );
                    }
                | none
                    {
                        $$ = Value::new_exc_var(
                            self.bump.alloc(
                                ExcVar { assoc_t: Maybe::none(), exc_var: Maybe::none() }
                            )
                        );
                    }
                ;

      opt_ensure: k_ensure compstmt
                    {
                        let ensure_t = $<Token>1;
                        let body = $<MaybeBoxedNode>2;
                        $$ = Value::new_opt_ensure(
                            Some(
                                self.bump.alloc(
                                    Ensure { ensure_t, body }
                                )
                            )
                        );
                    }
                | none
                    {
                        $$ = Value::new_opt_ensure(

                            None
                        );
                    }
                ;

         literal: numeric
                    {
                        $$ = $1;
                    }
                | symbol
                    {
                        $$ = $1;
                    }
                ;

         strings: string
                    {
                        $$ = Value::new_node(
                            self.builder.string_compose(
                                Maybe::none(),
                                $<NodeList>1,
                                Maybe::none()
                            )
                        );
                    }
                ;

          string: tCHAR
                    {
                        $$ = Value::new_node_list(
                                bump_vec![in self.bump;
                                    self.builder.character($<Token>1)
                                ]
                        );
                    }
                | string1
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>1 ] );
                    }
                | string string1
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>2 );
                        $$ = Value::new_node_list(nodes);
                    }
                ;

         string1: tSTRING_BEG string_contents tSTRING_END
                    {
                        let mut string = self.builder.string_compose(Maybe::some($<Token>1), $<NodeList>2, Maybe::some($<Token>3));
                        let indent = self.yylexer.buffer.heredoc_indent;
                        self.yylexer.buffer.heredoc_indent = 0;
                        self.builder.heredoc_dedent(string, indent);
                        $$ = Value::new_node(string);
                    }
                ;

         xstring: tXSTRING_BEG xstring_contents tSTRING_END
                    {
                        let mut string = self.builder.xstring_compose($<Token>1, $<NodeList>2, $<Token>3);
                        let indent = self.yylexer.buffer.heredoc_indent;
                        self.yylexer.buffer.heredoc_indent = 0;
                        self.builder.heredoc_dedent(string, indent);
                        $$ = Value::new_node(string);
                    }
                ;

          regexp: tREGEXP_BEG regexp_contents tREGEXP_END
                    {
                        let regexp_end = $<Token>3;
                        let opts = self.builder.regexp_options(regexp_end);
                        $$ = Value::new_node(
                            self.builder.regexp_compose(
                                $<Token>1,
                                $<NodeList>2,
                                regexp_end,
                                opts
                            )
                        );
                    }
                ;

           words: tWORDS_BEG tSPACE word_list tSTRING_END
                    {
                        $$ = Value::new_node(
                            self.builder.words_compose(
                                $<Token>1,
                                $<NodeList>3,
                                $<Token>4
                            )
                        );
                    }
                ;

       word_list: /* none */
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump;]);

                    }
                | word_list word tSPACE
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push(
                            self.builder.word( $<NodeList>2 )
                        );
                        $$ = Value::new_node_list(nodes);
                    }
                ;

            word: string_content
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>1 ]);
                    }
                | word string_content
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>2 );
                        $$ = Value::new_node_list(nodes);
                    }
                ;

         symbols: tSYMBOLS_BEG tSPACE symbol_list tSTRING_END
                    {
                        $$ = Value::new_node(
                            self.builder.symbols_compose(
                                $<Token>1,
                                $<NodeList>3,
                                $<Token>4
                            )
                        );
                    }
                ;

     symbol_list: /* none */
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump;]);
                    }
                | symbol_list word tSPACE
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push(
                            self.builder.word( $<NodeList>2 )
                        );
                        $$ = Value::new_node_list(nodes);
                    }
                ;

          qwords: tQWORDS_BEG tSPACE qword_list tSTRING_END
                    {
                        $$ = Value::new_node(
                            self.builder.words_compose(
                                $<Token>1,
                                $<NodeList>3,
                                $<Token>4
                            )
                        );
                    }
                ;

        qsymbols: tQSYMBOLS_BEG tSPACE qsym_list tSTRING_END
                    {
                        $$ = Value::new_node(
                            self.builder.symbols_compose(
                                $<Token>1,
                                $<NodeList>3,
                                $<Token>4
                            )
                        );
                    }
                ;

      qword_list: /* none */
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump;]);
                    }
                | qword_list tSTRING_CONTENT tSPACE
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push(
                            self.builder.string_internal( $<Token>2 )
                        );
                        $$ = Value::new_node_list(nodes);
                    }
                ;

       qsym_list: /* none */
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump;]);
                    }
                | qsym_list tSTRING_CONTENT tSPACE
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push(
                            self.builder.symbol_internal( $<Token>2 )
                        );
                        $$ = Value::new_node_list(nodes);
                    }
                ;

 string_contents: /* none */
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump;]);
                    }
                | string_contents string_content
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push($<Node>2);
                        $$ = Value::new_node_list(nodes);
                    }
                ;

xstring_contents: /* none */
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump;]);
                    }
                | xstring_contents string_content
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push($<Node>2);
                        $$ = Value::new_node_list(nodes);
                    }
                ;

 regexp_contents: /* none */
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump;]);
                    }
                | regexp_contents string_content
                    {
                        let mut  nodes = $<NodeList>1;
                        nodes.push( $<Node>2 );
                        $$ = Value::new_node_list(nodes);
                    }
                ;

  string_content: tSTRING_CONTENT
                    {
                        $$ = Value::new_node(
                            self.builder.string_internal($<Token>1)
                        );
                    }
                | tSTRING_DVAR
                    {
                        $<MaybeStrTerm>$ = Value::new_maybe_str_term(

                            std::mem::take(&mut self.yylexer.strterm)
                        );
                        self.yylexer.lex_state.set(EXPR_BEG);
                    }
                  string_dvar
                    {
                        self.yylexer.strterm = $<MaybeStrTerm>2;
                        $$ = $3;
                    }
                | tSTRING_DBEG
                    {
                        self.yylexer.cmdarg.push(false);
                        self.yylexer.cond.push(false);
                        $<None>$ = Value::new_none();
                    }
                    {
                        $<MaybeStrTerm>$ = Value::new_maybe_str_term(

                            std::mem::take(&mut self.yylexer.strterm)
                        );
                    }
                    {
                        $<Num>$ = Value::new_num(

                            self.yylexer.lex_state.get()
                        );
                        self.yylexer.lex_state.set(EXPR_BEG);
                    }
                    {
                        $<Num>$ = Value::new_num(

                            self.yylexer.brace_nest
                        );
                        self.yylexer.brace_nest = 0;
                    }
                    {
                        $<Num>$ = Value::new_num(

                            self.yylexer.buffer.heredoc_indent
                        );
                        self.yylexer.buffer.heredoc_indent = 0;
                    }
                  compstmt tSTRING_DEND
                    {
                        self.yylexer.cond.pop();
                        self.yylexer.cmdarg.pop();
                        self.yylexer.strterm = $<MaybeStrTerm>3;
                        self.yylexer.lex_state.set($<Num>4);
                        self.yylexer.brace_nest = $<Num>5;
                        self.yylexer.buffer.heredoc_indent = $<Num>6;
                        self.yylexer.buffer.heredoc_line_indent = -1;

                        $$ = Value::new_node(
                            self.builder.begin(
                                $<Token>1,
                                $<MaybeBoxedNode>7,
                                $<Token>8
                            )
                        );
                    }
                ;

     string_dvar: tGVAR
                    {
                        $$ = Value::new_node(self.builder.gvar($<Token>1));
                    }
                | tIVAR
                    {
                        $$ = Value::new_node(self.builder.ivar($<Token>1));

                    }
                | tCVAR
                    {
                        $$ = Value::new_node(self.builder.cvar($<Token>1));
                    }
                | backref
                    {
                        $$ = $1;
                    }
                ;

          symbol: ssym { $$ = $1; }
                | dsym { $$ = $1; }
                ;

            ssym: tSYMBEG sym
                    {
                        self.yylexer.lex_state.set(EXPR_END);
                        $$ = Value::new_node(
                            self.builder.symbol($<Token>1, $<Token>2)
                        );
                    }
                ;

             sym: fname { $$ = $1; }
                | tIVAR { $$ = $1; }
                | tGVAR { $$ = $1; }
                | tCVAR { $$ = $1; }
                ;

            dsym: tSYMBEG string_contents tSTRING_END
                    {
                        self.yylexer.lex_state.set(EXPR_END);
                        $$ = Value::new_node(
                            self.builder.symbol_compose($<Token>1, $<NodeList>2, $<Token>3)
                        );
                    }
                ;

         numeric: simple_numeric
                    {
                        $$ = $1;
                    }
                | tUMINUS_NUM simple_numeric   %prec tLOWEST
                    {
                        $$ = Value::new_node(
                            self.builder.unary_num(
                                $<Token>1,
                                $<BoxedNode>2
                            )
                        );
                    }
                ;

  simple_numeric: tINTEGER
                    {
                        $$ = Value::new_node(
                            self.builder.integer($<Token>1)
                        );
                    }
                | tFLOAT
                    {
                        $$ = Value::new_node(
                            self.builder.float($<Token>1)
                        );
                    }
                | tRATIONAL
                    {
                        $$ = Value::new_node(
                            self.builder.rational($<Token>1)
                        );
                    }
                | tIMAGINARY
                    {
                        $$ = Value::new_node(
                            self.builder.complex($<Token>1)
                        );
                    }
                ;

   user_variable: tIDENTIFIER
                    {
                        $$ = Value::new_node(
                            self.builder.lvar($<Token>1)
                        );
                    }
                | tIVAR
                    {
                        $$ = Value::new_node(
                            self.builder.ivar($<Token>1)
                        );
                    }
                | tGVAR
                    {
                        $$ = Value::new_node(
                            self.builder.gvar($<Token>1)
                        );
                    }
                | tCONSTANT
                    {
                        $$ = Value::new_node(
                            self.builder.const_($<Token>1)
                        );
                    }
                | tCVAR
                    {
                        $$ = Value::new_node(
                            self.builder.cvar($<Token>1)
                        );
                    }
                ;

keyword_variable: kNIL
                    {
                        $$ = Value::new_node(
                            self.builder.nil($<Token>1)
                        );
                    }
                | kSELF
                    {
                        $$ = Value::new_node(
                            self.builder.self_($<Token>1)
                        );
                    }
                | kTRUE
                    {
                        $$ = Value::new_node(
                            self.builder.true_($<Token>1)
                        );
                    }
                | kFALSE
                    {
                        $$ = Value::new_node(
                            self.builder.false_($<Token>1)
                        );
                    }
                | k__FILE__
                    {
                        $$ = Value::new_node(
                            self.builder.__file__($<Token>1)
                        );
                    }
                | k__LINE__
                    {
                        $$ = Value::new_node(
                            self.builder.__line__($<Token>1)
                        );
                    }
                | k__ENCODING__
                    {
                        $$ = Value::new_node(
                            self.builder.__encoding__($<Token>1)
                        );
                    }
                ;

         var_ref: user_variable
                    {
                        let node = $<BoxedNode>1;
                        if let Some(node) = node.as_lvar() {
                            let name = node.get_name().as_str();
                            match name.as_bytes()[..] {
                                [b'_', n] if (b'1'..=b'9').contains(&n) => {
                                    if !self.static_env.is_declared(name) && self.context.is_in_dynamic_block() {
                                        /* definitely an implicit param */

                                        if self.max_numparam_stack.has_ordinary_params() {
                                            return self.yyerror(
                                                @1,
                                                DiagnosticMessage::new_ordinary_param_defined(),
                                            );
                                        }

                                        let mut raw_context = self.context.inner_clone();
                                        let mut raw_max_numparam_stack = self.max_numparam_stack.inner_clone();

                                        /* ignore current block scope */
                                        raw_context.pop();
                                        raw_max_numparam_stack.pop();

                                        for outer_scope in raw_context.iter().rev() {
                                            if *outer_scope == ContextItem::Block || *outer_scope == ContextItem::Lambda {
                                                let outer_scope_has_numparams = raw_max_numparam_stack
                                                    .pop()
                                                    .unwrap_or(0) > 0;

                                                if outer_scope_has_numparams {
                                                    return self.yyerror(
                                                        @1,
                                                        DiagnosticMessage::new_numparam_used(),
                                                    );
                                                } else {
                                                    /* for now it's ok, but an outer scope can also be a block
                                                        with numparams, so we need to continue */
                                                }
                                            } else {
                                                /* found an outer scope that can't have numparams
                                                    like def/class/etc */
                                                break;
                                            }
                                        }

                                        self.static_env.declare(name);
                                        self.max_numparam_stack.register((n - b'0') as i32)
                                    }
                                },
                                _ => {}
                            }
                        }

                        $$ = Value::new_node(
                            self.builder.accessible(node)
                        );
                    }
                | keyword_variable
                    {
                        $$ = Value::new_node(
                            self.builder.accessible($<BoxedNode>1)
                        );
                    }
                ;

         var_lhs: user_variable
                    {
                        $$ = Value::new_node(
                            self.builder.assignable($<BoxedNode>1)?
                        );
                    }
                | keyword_variable
                    {
                        $$ = Value::new_node(
                            self.builder.assignable($<BoxedNode>1)?
                        );
                    }
                ;

         backref: tNTH_REF
                    {
                        $$ = Value::new_node(
                            self.builder.nth_ref($<Token>1)
                        );
                    }
                | tBACK_REF
                    {
                        $$ = Value::new_node(
                            self.builder.back_ref($<Token>1)
                        );
                    }
                ;

      superclass: tLT
                    {
                        self.yylexer.lex_state.set(EXPR_BEG);
                        self.yylexer.command_start = true;
                        $<None>$ = Value::new_none();
                    }
                  expr_value term
                    {
                        let lt_t  = Maybe::some($<Token>1);
                        let value = Maybe::some($<BoxedNode>3);
                        $$ = Value::new_superclass(
                            self.bump.alloc(
                                Superclass { lt_t, value }
                            )
                        );
                    }
                | /* none */
                    {
                        $$ = Value::new_superclass(
                            self.bump.alloc(
                                Superclass { lt_t: Maybe::none(), value: Maybe::none() }
                            )
                        );
                    }
                ;

f_opt_paren_args: f_paren_args
                    {
                        $$ = $1;
                    }
                | none
                    {
                        $$ = Value::new_maybe_node(Maybe::none());
                    }
                ;

    f_paren_args: tLPAREN2 f_args rparen
                    {
                        $$ = Value::new_maybe_node(
                            self.builder.args(Maybe::some($<Token>1), $<NodeList>2, Maybe::some($<Token>3))
                        );

                        self.yylexer.lex_state.set(EXPR_BEG);
                        self.yylexer.command_start = true;
                    }
                | tLPAREN2 f_arg tCOMMA args_forward rparen
                    {
                        let mut args = $<NodeList>2;
                        args.push(self.builder.forward_arg($<Token>4));

                        $$ = Value::new_maybe_node(
                            self.builder.args(
                                Maybe::some($<Token>1),
                                args,
                                Maybe::some($<Token>5)
                            )
                        );

                        self.static_env.declare_forward_args();
                        self.yylexer.lex_state.set(EXPR_BEG);
                        self.yylexer.command_start = true;
                    }
                | tLPAREN2 args_forward rparen
                    {
                        $$ = Value::new_maybe_node(
                            Maybe::some(
                                self.builder.forward_only_args($<Token>1, $<Token>2, $<Token>3)
                            )
                        );

                        self.static_env.declare_forward_args();
                        self.yylexer.lex_state.set(EXPR_BEG);
                        self.yylexer.command_start = true;
                    }
                ;

       f_arglist: f_paren_args
                    {
                        $$ = $1;
                    }
                |   {
                        $<Bool>$ = Value::new_bool(self.yylexer.in_kwarg);
                        self.yylexer.in_kwarg = true;
                        self.yylexer.lex_state.set(self.yylexer.lex_state.get()|EXPR_LABEL);
                    }
                  f_args term
                    {
                        self.yylexer.in_kwarg = $<Bool>1;
                        $$ = Value::new_maybe_node(
                            self.builder.args(Maybe::none(), $<NodeList>2, Maybe::none())
                        );
                        self.yylexer.lex_state.set(EXPR_BEG);
                        self.yylexer.command_start = true;
                    }
                ;

       args_tail: f_kwarg tCOMMA f_kwrest opt_f_block_arg
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_kwrest = $<NodeList>3;
                        let mut opt_f_block_arg = $<NodeList>4;

                        nodes.reserve(f_kwrest.len() + opt_f_block_arg.len());
                        nodes.append(&mut f_kwrest);
                        nodes.append(&mut opt_f_block_arg);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_kwarg opt_f_block_arg
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.append(&mut $<NodeList>2);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_any_kwrest opt_f_block_arg
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.append(&mut $<NodeList>2);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_block_arg
                    {
                        $$ = Value::new_node_list(
                                bump_vec![in self.bump; $<Node>1 ]
                        );
                    }
                ;

   opt_args_tail: tCOMMA args_tail
                    {
                        $$ = $2;
                    }
                | /* none */
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump;]);
                    }
                ;

          f_args: f_arg tCOMMA f_optarg tCOMMA f_rest_arg opt_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_optarg = $<NodeList>3;
                        let mut f_rest_arg = $<NodeList>5;
                        let mut opt_args_tail = $<NodeList>6;

                        nodes.reserve(f_optarg.len() + f_rest_arg.len() + opt_args_tail.len());
                        nodes.append(&mut f_optarg);
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut opt_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_arg tCOMMA f_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_optarg = $<NodeList>3;
                        let mut f_rest_arg = $<NodeList>5;
                        let mut f_arg = $<NodeList>7;
                        let mut opt_args_tail = $<NodeList>8;

                        nodes.reserve(f_optarg.len() + f_rest_arg.len() + f_arg.len() + opt_args_tail.len());
                        nodes.append(&mut f_optarg);
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_arg tCOMMA f_optarg opt_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_optarg = $<NodeList>3;
                        let mut opt_args_tail = $<NodeList>4;

                        nodes.reserve(f_optarg.len() + opt_args_tail.len());
                        nodes.append(&mut f_optarg);
                        nodes.append(&mut opt_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_arg tCOMMA f_optarg tCOMMA f_arg opt_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_optarg = $<NodeList>3;
                        let mut f_arg = $<NodeList>5;
                        let mut opt_args_tail = $<NodeList>6;

                        nodes.reserve(f_optarg.len() + f_arg.len() + opt_args_tail.len());
                        nodes.append(&mut f_optarg);
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_arg tCOMMA f_rest_arg opt_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_rest_arg = $<NodeList>3;
                        let mut opt_args_tail = $<NodeList>4;

                        nodes.reserve(f_rest_arg.len() + opt_args_tail.len());
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut opt_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_arg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_rest_arg = $<NodeList>3;
                        let mut f_arg = $<NodeList>5;
                        let mut opt_args_tail = $<NodeList>6;

                        nodes.reserve(f_rest_arg.len() + f_arg.len() + opt_args_tail.len());
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_arg opt_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.append(&mut $<NodeList>2);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_optarg tCOMMA f_rest_arg opt_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_rest_arg = $<NodeList>3;
                        let mut opt_args_tail = $<NodeList>4;

                        nodes.reserve(f_rest_arg.len() + opt_args_tail.len());
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut opt_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_rest_arg = $<NodeList>3;
                        let mut f_arg = $<NodeList>5;
                        let mut opt_args_tail = $<NodeList>6;

                        nodes.reserve(f_rest_arg.len() + f_arg.len() + opt_args_tail.len());
                        nodes.append(&mut f_rest_arg);
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_optarg opt_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.append(&mut $<NodeList>2);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_optarg tCOMMA f_arg opt_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_arg = $<NodeList>3;
                        let mut opt_args_tail = $<NodeList>4;

                        nodes.reserve(f_arg.len() + opt_args_tail.len());
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_rest_arg opt_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.append(&mut $<NodeList>2);

                        $$ = Value::new_node_list(nodes);
                    }
                | f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                        let mut nodes = $<NodeList>1;
                        let mut f_arg = $<NodeList>3;
                        let mut opt_args_tail = $<NodeList>4;

                        nodes.reserve(f_arg.len() + opt_args_tail.len());
                        nodes.append(&mut f_arg);
                        nodes.append(&mut opt_args_tail);

                        $$ = Value::new_node_list(nodes);
                    }
                | args_tail
                    {
                        $$ = Value::new_node_list($<NodeList>1);
                    }
                | /* none */
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump;]);
                    }
                ;

    args_forward: tBDOT3
                    {
                        $$ = $1;
                    }
                ;

       f_bad_arg: tCONSTANT
                    {
                        return self.yyerror(@1, DiagnosticMessage::new_const_argument());
                    }
                | tIVAR
                    {
                        return self.yyerror(@1, DiagnosticMessage::new_ivar_argument());
                    }
                | tGVAR
                    {
                        return self.yyerror(@1, DiagnosticMessage::new_gvar_argument());
                    }
                | tCVAR
                    {
                        return self.yyerror(@1, DiagnosticMessage::new_cvar_argument());
                    }
                ;

      f_norm_arg: f_bad_arg
                    {
                        $$ = $1;
                    }
                | tIDENTIFIER
                    {
                        let ident_t = $<Token>1;
                        let name = clone_value(&ident_t);
                        self.static_env.declare(name.as_str());
                        self.max_numparam_stack.set_has_ordinary_params();
                        $$ = Value::new_token(

                            ident_t
                        );
                    }
                ;

      f_arg_asgn: f_norm_arg
                    {
                        let arg_t = $<Token>1;
                        let arg_name = clone_value(&arg_t).to_string();
                        self.current_arg_stack.set(Some(arg_name));
                        $$ = Value::new_token(

                            arg_t
                        );
                    }
                ;

      f_arg_item: f_arg_asgn
                    {
                        self.current_arg_stack.set(None);
                        $$ = Value::new_node(
                            self.builder.arg($<Token>1)?
                        );
                    }
                | tLPAREN f_margs rparen
                    {
                        $$ = Value::new_node(
                            self.builder.multi_lhs(
                                Maybe::some($<Token>1),
                                $<NodeList>2,
                                Maybe::some($<Token>3)
                            )
                        );
                    }
                ;

           f_arg: f_arg_item
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>1 ]);
                    }
                | f_arg tCOMMA f_arg_item
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::new_node_list(nodes);
                    }
                ;


         f_label: tLABEL
                    {
                        let ident_t = $<Token>1;
                        self.check_kwarg_name(&ident_t)?;

                        let ident = clone_value(&ident_t).to_string();
                        self.static_env.declare(&ident);

                        self.max_numparam_stack.set_has_ordinary_params();

                        self.current_arg_stack.set(Some(ident));

                        $$ = Value::new_token(

                            ident_t
                        );
                    }
                ;

            f_kw: f_label arg_value
                    {
                        self.current_arg_stack.set(None);
                        $$ = Value::new_node(
                            self.builder.kwoptarg($<Token>1, $<BoxedNode>2)?
                        );
                    }
                | f_label
                    {
                        self.current_arg_stack.set(None);
                        $$ = Value::new_node(
                            self.builder.kwarg($<Token>1)?
                        );
                    }
                ;

      f_block_kw: f_label primary_value
                    {
                        $$ = Value::new_node(
                            self.builder.kwoptarg($<Token>1, $<BoxedNode>2)?
                        );
                    }
                | f_label
                    {
                        $$ = Value::new_node(
                            self.builder.kwarg($<Token>1)?
                        );
                    }
                ;

   f_block_kwarg: f_block_kw
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>1 ]);
                    }
                | f_block_kwarg tCOMMA f_block_kw
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::new_node_list(nodes);
                    }
                ;


         f_kwarg: f_kw
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>1 ]);
                    }
                | f_kwarg tCOMMA f_kw
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::new_node_list(nodes);
                    }
                ;

     kwrest_mark: tPOW
                    {
                        $$ = $1;
                    }
                | tDSTAR
                    {
                        $$ = $1;
                    }
                ;

      f_no_kwarg: kwrest_mark kNIL
                    {
                        $$ = Value::new_node_list(
                                bump_vec![in self.bump;
                                    self.builder.kwnilarg(
                                        $<Token>1,
                                        $<Token>2
                                    )
                                ]
                        );
                    }
                ;

        f_kwrest: kwrest_mark tIDENTIFIER
                    {
                        let ident_t = $<Token>2;
                        self.static_env.declare(clone_value(&ident_t).as_str());
                        $$ = Value::new_node_list(
                                bump_vec![in self.bump;
                                    (self.builder.kwrestarg($<Token>1, Maybe::some(ident_t))?)
                                ]
                        );
                    }
                | kwrest_mark
                    {
                        $$ = Value::new_node_list(
                                bump_vec![in self.bump;
                                    (self.builder.kwrestarg($<Token>1, Maybe::none())?)
                                ]
                        );
                    }
                ;

           f_opt: f_arg_asgn tEQL arg_value
                    {
                        self.current_arg_stack.set(None);
                        $$ = Value::new_node(
                            self.builder.optarg(
                                $<Token>1,
                                $<Token>2,
                                $<BoxedNode>3
                            )?
                        );
                    }
                ;

     f_block_opt: f_arg_asgn tEQL primary_value
                    {
                        self.current_arg_stack.set(None);
                        $$ = Value::new_node(
                            self.builder.optarg(
                                $<Token>1,
                                $<Token>2,
                                $<BoxedNode>3
                            )?
                        );
                    }
                ;

  f_block_optarg: f_block_opt
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>1 ]);
                    }
                | f_block_optarg tCOMMA f_block_opt
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::new_node_list(nodes);
                    }
                ;

        f_optarg: f_opt
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>1 ]);
                    }
                | f_optarg tCOMMA f_opt
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push( $<Node>3 );
                        $$ = Value::new_node_list(nodes);
                    }
                ;

    restarg_mark: tSTAR2
                    {
                        $$ = $1;
                    }
                | tSTAR
                    {
                        $$ = $1;
                    }
                ;

      f_rest_arg: restarg_mark tIDENTIFIER
                    {
                        let ident_t = $<Token>2;
                        self.static_env.declare(clone_value(&ident_t).as_str());

                        $$ = Value::new_node_list(
                                bump_vec![in self.bump;
                                    (self.builder.restarg($<Token>1, Maybe::some(ident_t))?)
                                ]
                        );
                    }
                | restarg_mark
                    {
                        $$ = Value::new_node_list(
                                bump_vec![in self.bump;
                                    (self.builder.restarg($<Token>1, Maybe::none())?)
                                ]
                        );
                    }
                ;

     blkarg_mark: tAMPER2
                    {
                        $$ = $1;
                    }
                | tAMPER
                    {
                        $$ = $1;
                    }
                ;

     f_block_arg: blkarg_mark tIDENTIFIER
                    {
                        let ident_t = $<Token>2;
                        self.static_env.declare(clone_value(&ident_t).as_str());
                        $$ = Value::new_node(
                            self.builder.blockarg($<Token>1, ident_t)?
                        );
                    }
                ;

 opt_f_block_arg: tCOMMA f_block_arg
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>2 ]);
                    }
                | none
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump;]);
                    }
                ;

       singleton: var_ref
                    {
                        let var_ref = $<BoxedNode>1;
                        let var_ref = self.value_expr(var_ref)?;
                        $$ = Value::new_node(var_ref);
                    }
                | tLPAREN2 { self.yylexer.lex_state.set(EXPR_BEG); $<None>$ = Value::new_none(); } expr rparen
                    {
                        let mut expr = $<BoxedNode>3;

                        if expr.is_int() ||
                            expr.is_float() ||
                            expr.is_rational() ||
                            expr.is_complex() ||
                            expr.is_str() ||
                            expr.is_dstr() ||
                            expr.is_sym() ||
                            expr.is_dsym() ||
                            expr.is_heredoc() ||
                            expr.is_x_heredoc() ||
                            expr.is_regexp() ||
                            expr.is_array() ||
                            expr.is_hash() {
                            self.yyerror1(
                                DiagnosticMessage::new_singleton_literal(),
                                expr.expression().clone(),
                            )?;
                        } else {
                            expr = self.value_expr(expr)?
                        }

                        $$ = Value::new_node(expr);
                    }
                ;

      assoc_list: none
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump;]);
                    }
                | assocs trailer
                    {
                        $$ = $1;
                    }
                ;

          assocs: assoc
                    {
                        $$ = Value::new_node_list( bump_vec![in self.bump; $<Node>1 ]);
                    }
                | assocs tCOMMA assoc
                    {
                        let mut nodes = $<NodeList>1;
                        nodes.push($<Node>3);
                        $$ = Value::new_node_list(nodes);
                    }
                ;

           assoc: arg_value tASSOC arg_value
                    {
                        $$ = Value::new_node(
                            self.builder.pair(
                                $<BoxedNode>1,
                                $<Token>2,
                                $<BoxedNode>3
                            )
                        );
                    }
                | tLABEL arg_value
                    {
                        $$ = Value::new_node(
                            self.builder.pair_keyword(
                                $<Token>1,
                                $<BoxedNode>2
                            )
                        );
                    }
                | tSTRING_BEG string_contents tLABEL_END arg_value
                    {
                        $$ = Value::new_node(
                            self.builder.pair_quoted(
                                $<Token>1,
                                $<NodeList>2,
                                $<Token>3,
                                $<BoxedNode>4
                            )
                        );
                    }
                | tDSTAR arg_value
                    {
                        $$ = Value::new_node(
                            self.builder.kwsplat($<Token>1, $<BoxedNode>2)
                        );
                    }
                ;

       operation: tIDENTIFIER
                    {
                        $$ = $1;
                    }
                | tCONSTANT
                    {
                        $$ = $1;
                    }
                | tFID
                    {
                        $$ = $1;
                    }
                ;

      operation2: tIDENTIFIER
                    {
                        $$ = $1;
                    }
                | tCONSTANT
                    {
                        $$ = $1;
                    }
                | tFID
                    {
                        $$ = $1;
                    }
                | op
                    {
                        $$ = $1;
                    }
                ;

      operation3: tIDENTIFIER
                    {
                        $$ = $1;
                    }
                | tFID
                    {
                        $$ = $1;
                    }
                | op
                    {
                        $$ = $1;
                    }
                ;

    dot_or_colon: tDOT
                    {
                        $$ = $1;
                    }
                | tCOLON2
                    {
                        $$ = $1;
                    }
                ;

         call_op: tDOT
                    {
                        $$ = $1;
                    }
                | tANDDOT
                    {
                        $$ = $1;
                    }
                ;

        call_op2: call_op
                    {
                        $$ = $1;
                    }
                | tCOLON2
                    {
                        $$ = $1;
                    }
                ;

       opt_terms: /* none */
                    {
                        $$ = Value::new_none();
                    }
                | terms
                    {
                        $$ = Value::new_none();
                    }
                ;

          opt_nl: /* none */
                    {
                        $$ = Value::new_none();
                    }
                | tNL
                    {
                        $$ = Value::new_none();
                    }
                ;

          rparen: opt_nl tRPAREN
                    {
                        $$ = $2;
                    }
                ;

        rbracket: opt_nl tRBRACK
                    {
                        $$ = $2;
                    }
                ;

          rbrace: opt_nl tRCURLY
                    {
                        $$ = $2;
                    }
                ;

         trailer: /* none */
                    {
                        $$ = Value::new_none();
                    }
                | tNL
                    {
                        $$ = Value::new_none();
                    }
                | tCOMMA
                    {
                        $$ = Value::new_none();
                    }
                ;

            term: tSEMI
                    {
                        $$ = $1;
                    }
                | tNL
                    {
                        $$ = $1;
                    }
                ;

           terms: term
                    {
                        $$ = Value::new_token_list(

                            bump_vec![in self.bump;]
                        );
                    }
                | terms tSEMI
                    {
                        $$ = Value::new_token_list(

                            bump_vec![in self.bump;]
                        );
                    }
                ;

            none: /* empty */
                  {
                        $$ = Value::new_none();
                  }
                ;

%%

impl<'a /*'*/> Parser<'a /*'*/> {
    /// Constructs a parser with given `input` and `options`.
    ///
    /// Returns an error if given `input` is invalid.
    pub fn new<TInput>(bump: &'a /*'*/ bumpalo::Bump, input: TInput, options: ParserOptions<'a /*'*/>) -> Self
    where
        TInput: Into<Vec<'a /*'*/, u8>>
    {
        let InternalParserOptions {
            buffer_name,
            decoder,
            token_rewriter,
            record_tokens,
        } = options.into();

        let context = ParserContext::new();
        let current_arg_stack = CurrentArgStack::new();
        let max_numparam_stack = MaxNumparamStack::new();
        let pattern_variables = VariablesStack::new();
        let pattern_hash_keys = VariablesStack::new();
        let static_env = StaticEnvironment::new();
        let diagnostics = Diagnostics::new(bump);

        let input: Vec<'a /*'*/, u8> = input.into();

        let mut lexer = Lexer::new(bump, input, buffer_name, decoder);
        lexer.context = context.clone();
        lexer.static_env = static_env.clone();
        lexer.diagnostics = diagnostics.clone();

        let builder = Builder::new(
            bump,
            static_env.clone(),
            context.clone(),
            current_arg_stack.clone(),
            max_numparam_stack.clone(),
            pattern_variables.clone(),
            pattern_hash_keys.clone(),
            diagnostics.clone(),
        );

        let last_token_type = 0;

        Self {
            yy_error_verbose: true,
            yynerrs: 0,
            yyerrstatus_: 0,
            result: Maybe::none(),
            bump,

            builder,
            context,
            current_arg_stack,
            max_numparam_stack,
            pattern_variables,
            pattern_hash_keys,
            static_env,
            last_token_type,
            tokens: bump_vec![in bump;],
            diagnostics,
            yylexer: lexer,
            token_rewriter,
            record_tokens,
        }
    }

    /// Parses given input and returns:
    ///
    /// 1. AST
    /// 2. tokens
    /// 3. diagnostics
    /// 4. coments
    /// 5. magic comments
    pub fn do_parse(mut self) -> ParserResult<'a /*'*/>  {
        self.parse();

        ParserResult::new(
            self.bump,
            self.result,
            self.tokens,
            self.diagnostics.take_inner(),
            self.yylexer.comments,
            self.yylexer.magic_comments,
            self.yylexer.buffer.input.decoded,
        )
    }

    #[doc(hidden)]
    pub fn do_parse_with_state_validation(mut self) -> ParserResult<'a /*'*/> {
        self.parse();

        self.assert_state_is_final();

        ParserResult::new(
            self.bump,
            self.result,
            self.tokens,
            self.diagnostics.take_inner(),
            self.yylexer.comments,
            self.yylexer.magic_comments,
            self.yylexer.buffer.input.decoded,
        )
    }

    fn warn(&mut self, loc: &Loc, message: DiagnosticMessage<'a /*'*/>) {
        let diagnostic = Diagnostic::new(
            ErrorLevel::warning(),
            message,
            loc.clone(),
        );
        self.diagnostics.emit(diagnostic);
    }

    fn yylex(&mut self) -> &'a /*'*/ Token<'a /*'*/> {
        self.yylexer.yylex()
    }

    fn next_token(&mut self) -> &'a /*'*/ Token<'a /*'*/> {
        let mut token = self.yylex();

        if let Some(token_rewriter) = self.token_rewriter.as_ref() {
            let TokenRewriterResult { rewritten_token, token_action, lex_state_action } =
                token_rewriter.call(token, self.yylexer.buffer.input.decoded.bytes.as_slice());

            if lex_state_action.is_keep() {
                // keep
            } else if lex_state_action.is_set() {
                self.yylexer.lex_state.set(lex_state_action.next_state());
            } else {
                panic!("Unknown LexStateAction variant");
            }

            if token_action.is_drop() {
                return self.next_token()
            } else if token_action.is_keep() {
                token = rewritten_token
            } else {
                panic!("Unknown RewriteAction variant");
            }
        }

        self.last_token_type = token.token_type();

        if self.record_tokens {
            self.tokens.push(token);
        }

        token
    }

    fn check_kwarg_name(&self, ident_t: &Token) -> Result<(), ()> {
        let name = clone_value(ident_t);
        let first_char = name.as_str().chars().next().expect("kwarg name can't be empty");
        if first_char.is_lowercase() || first_char == '_' {
            Ok(())
        } else {
            let loc = ident_t.loc().clone();
            self.diagnostics.emit(
                Diagnostic::new(
                    ErrorLevel::error(),
                    DiagnosticMessage::new_const_argument(),
                    loc
                )
            );
            Err(())
        }
    }

    fn validate_endless_method_name(&mut self, name_t: &Token) -> Result<(), ()> {
        let name = clone_value(name_t);
        if name.as_str().ends_with('=') {
            self.yyerror(name_t.loc(), DiagnosticMessage::new_endless_setter_definition()).map(|_| ())
        } else {
            Ok(())
        }
    }

    fn yyerror(&mut self, loc: &Loc, message: DiagnosticMessage<'a /*'*/>) -> Result<i32, ()> {
        self.yyerror1(
            message,
            loc.clone()
        )
    }

    fn yyerror1(&mut self, message: DiagnosticMessage<'a /*'*/>, loc: Loc) -> Result<i32, ()> {
        let diagnostic = Diagnostic::new(ErrorLevel::error(), message, loc);
        self.diagnostics.emit(diagnostic);
        Err(())
    }

    fn report_syntax_error(&mut self, ctx: &Context) {
        let id: usize = ctx.token().code().try_into().expect("failed to convert token code into i32, is it too big?");
        let diagnostic = Diagnostic::new(
            ErrorLevel::error(),
            DiagnosticMessage::new_unexpected_token(
                String::from_str_in(
                    Lexer::TOKEN_NAMES[id],
                    self.bump
                )
            ),
            ctx.location().clone(),
        );
        self.diagnostics.emit(diagnostic);
    }

    fn warn_eol(&mut self, loc: &Loc, tok: &str) {
        if self.yylexer.buffer.is_looking_at_eol() {
            self.warn(
                loc,
                DiagnosticMessage::new_tok_at_eol_without_expression(
                    String::from_str_in(tok, self.bump)
                )
            );
        }
    }

    fn value_expr(&self, node: &'a /*'*/ Node<'a /*'*/>) -> Result<&'a /*'*/ Node<'a /*'*/>, ()> {
        self.builder.value_expr(node)
    }

    #[doc(hidden)]
    fn assert_state_is_final(&self) {
        assert!(self.yylexer.cmdarg.is_empty());
        assert!(self.yylexer.cond.is_empty());
        assert!(self.yylexer.paren_nest == 0);

        assert!(self.static_env.is_empty());
        assert!(self.context.is_empty());
        assert!(self.max_numparam_stack.is_empty());
        assert!(self.current_arg_stack.is_empty());
        assert!(self.pattern_variables.is_empty());
        assert!(self.pattern_hash_keys.is_empty());
    }
}

fn as_mut<T>(t: &T) -> &mut T {
    unsafe { std::mem::transmute(t) }
}

fn take_vec<'a /*'*/>(vec: &Vec<'a /*'*/, &'a /*'*/ Node<'a /*'*/>>) -> Vec<'a /*'*/, &'a /*'*/ Node<'a /*'*/>> {
    as_mut(vec).split_off(0)
}

fn take_str<'a /*'*/>(s: &String<'a /*'*/>) -> String<'a /*'*/> {
    as_mut(s).split_off(0)
}

fn take_maybe_node<'a /*'*/>(maybe_node: &Option<&'a /*'*/ Node<'a /*'*/>>) -> Option<&'a /*'*/ Node<'a /*'*/>> {
    as_mut(maybe_node).take()
}

fn take_maybe_token<'a /*'*/>(maybe_token: &Option<&'a /*'*/ Token<'a /*'*/>>) -> Option<&'a /*'*/ Token<'a /*'*/>> {
    as_mut(maybe_token).take()
}
