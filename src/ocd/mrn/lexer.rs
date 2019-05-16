use crate::ocd::config::{Config, Verbosity};

#[derive(Debug, PartialEq)]

pub enum Token {
    Comma,
    Space,
    End,
    Number { value: usize },
    String { value: String },
    PatternMatch,
    LowerCase,
    UpperCase,
    TitleCase,
    SentenceCase,
    CamelCaseJoin,
    CamelCaseSplit,
    ExtensionAdd,
    ExtensionRemove,
    Insert,
    InteractiveTokenize,
    InteractivePatternMatch,
    Delete,
    Replace,
    ReplaceSpaceDash,
    ReplaceSpacePeriod,
    ReplaceSpaceUnder,
    ReplaceDashSpace,
    ReplaceDashPeriod,
    ReplaceDashUnder,
    ReplacePeriodDash,
    ReplacePeriodSpace,
    ReplacePeriodUnder,
    ReplaceUnderSpace,
    ReplaceUnderDash,
    ReplaceUnderPeriod,
    Sanitize,
}

enum TokenizerState {
    Init,
    Error,
    Comma,
    Space,
    String,
    Number,
    C,
    CC,
    CCJ,
    CCS,
    DP,
    DS,
    DU,
    D,
    E,
    EN,
    END,
    EA,
    ER,
    I,
    IP,
    IT,
    L,
    LC,
    P,
    PD,
    PS,
    PU,
    R,
    S,
    SC,
    SP,
    SD,
    SU,
    T,
    TC,
    U,
    UC,
    UD,
    US,
    UP,
}

struct Tokenizer {
    state: TokenizerState,
    string: String,
    number: String,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            state: TokenizerState::Init,
            string: String::new(),
            number: String::new(),
        }
    }

    pub fn run(&mut self, config: &Config, input: &str) -> Result<Vec<Token>, &'static str> {
        let mut tokens = Vec::new();
        for c in input.chars() {
            match self.state {
                TokenizerState::Init => self.state_init(config, c, &mut tokens),
                TokenizerState::Error => return Err("Lexer error"),
                TokenizerState::Comma => self.state_comma(config, c, &mut tokens),
                TokenizerState::Space => self.state_space(config, c, &mut tokens),
                TokenizerState::String => self.state_string(config, c, &mut tokens),
                TokenizerState::Number => self.state_number(config, c, &mut tokens),
                TokenizerState::C => self.state_c(config, c, &mut tokens),
                TokenizerState::CC => self.state_cc(config, c, &mut tokens),
                TokenizerState::CCJ => self.state_ccj(config, c, &mut tokens),
                TokenizerState::CCS => self.state_ccs(config, c, &mut tokens),
                TokenizerState::DP => self.state_dp(config, c, &mut tokens),
                TokenizerState::DS => self.state_ds(config, c, &mut tokens),
                TokenizerState::DU => self.state_du(config, c, &mut tokens),
                TokenizerState::D => self.state_d(config, c, &mut tokens),
                TokenizerState::E => self.state_e(config, c, &mut tokens),
                TokenizerState::EN => self.state_en(config, c, &mut tokens),
                TokenizerState::END => self.state_end(config, c, &mut tokens),
                TokenizerState::EA => self.state_ea(config, c, &mut tokens),
                TokenizerState::ER => self.state_er(config, c, &mut tokens),
                TokenizerState::I => self.state_i(config, c, &mut tokens),
                TokenizerState::IP => self.state_ip(config, c, &mut tokens),
                TokenizerState::IT => self.state_it(config, c, &mut tokens),
                TokenizerState::L => self.state_l(config, c, &mut tokens),
                TokenizerState::LC => self.state_lc(config, c, &mut tokens),
                TokenizerState::P => self.state_p(config, c, &mut tokens),
                TokenizerState::PS => self.state_ps(config, c, &mut tokens),
                TokenizerState::PD => self.state_pd(config, c, &mut tokens),
                TokenizerState::PU => self.state_pu(config, c, &mut tokens),
                TokenizerState::R => self.state_r(config, c, &mut tokens),
                TokenizerState::S => self.state_s(config, c, &mut tokens),
                TokenizerState::SC => self.state_sc(config, c, &mut tokens),
                TokenizerState::SP => self.state_sp(config, c, &mut tokens),
                TokenizerState::SD => self.state_sd(config, c, &mut tokens),
                TokenizerState::SU => self.state_su(config, c, &mut tokens),
                TokenizerState::T => self.state_t(config, c, &mut tokens),
                TokenizerState::TC => self.state_tc(config, c, &mut tokens),
                TokenizerState::U => self.state_u(config, c, &mut tokens),
                TokenizerState::UC => self.state_uc(config, c, &mut tokens),
                TokenizerState::UD => self.state_ud(config, c, &mut tokens),
                TokenizerState::US => self.state_us(config, c, &mut tokens),
                TokenizerState::UP => self.state_up(config, c, &mut tokens),
            }
        }
        match self.state {
            TokenizerState::Init => {}
            TokenizerState::Comma => {
                tokens.push(Token::Comma);
            }
            TokenizerState::Space => {
                tokens.push(Token::Space);
            }
            TokenizerState::Number => match self.number.parse::<usize>() {
                Ok(value) => {
                    tokens.push(Token::Number { value });
                }
                Err(_err) => return Err("Error: unable to read number"),
            },
            TokenizerState::CCJ => {
                tokens.push(Token::CamelCaseJoin);
            }
            TokenizerState::CCS => {
                tokens.push(Token::CamelCaseSplit);
            }
            TokenizerState::D => {
                tokens.push(Token::Delete);
            }
            TokenizerState::DP => {
                tokens.push(Token::ReplaceDashPeriod);
            }
            TokenizerState::DS => {
                tokens.push(Token::ReplaceDashSpace);
            }
            TokenizerState::DU => {
                tokens.push(Token::ReplaceDashUnder);
            }
            TokenizerState::EA => {
                tokens.push(Token::ExtensionAdd);
            }
            TokenizerState::ER => {
                tokens.push(Token::ExtensionRemove);
            }
            TokenizerState::END => tokens.push(Token::End),
            TokenizerState::I => {
                tokens.push(Token::Insert);
            }
            TokenizerState::IP => {
                tokens.push(Token::InteractivePatternMatch);
            }
            TokenizerState::IT => {
                tokens.push(Token::InteractiveTokenize);
            }
            TokenizerState::LC => {
                tokens.push(Token::LowerCase);
            }
            TokenizerState::P => {
                tokens.push(Token::PatternMatch);
            }
            TokenizerState::PD => {
                tokens.push(Token::ReplacePeriodDash);
            }
            TokenizerState::PS => {
                tokens.push(Token::ReplacePeriodSpace);
            }
            TokenizerState::PU => {
                tokens.push(Token::ReplacePeriodUnder);
            }
            TokenizerState::R => {
                tokens.push(Token::Replace);
            }
            TokenizerState::S => {
                tokens.push(Token::Sanitize);
            }
            TokenizerState::SC => {
                tokens.push(Token::SentenceCase);
            }
            TokenizerState::SP => {
                tokens.push(Token::ReplaceSpacePeriod);
            }
            TokenizerState::SD => {
                tokens.push(Token::ReplaceSpaceDash);
            }
            TokenizerState::SU => {
                tokens.push(Token::ReplaceSpaceUnder);
            }
            TokenizerState::TC => {
                tokens.push(Token::TitleCase);
            }
            TokenizerState::UC => {
                tokens.push(Token::UpperCase);
            }
            TokenizerState::UD => {
                tokens.push(Token::ReplaceUnderDash);
            }
            TokenizerState::UP => {
                tokens.push(Token::ReplaceUnderPeriod);
            }
            TokenizerState::US => {
                tokens.push(Token::ReplaceUnderSpace);
            }
            TokenizerState::String => return Err("Error: unfinished string"),
            TokenizerState::C => return Err("Error: unfinished rule, read: 'c'"),
            TokenizerState::CC => return Err("Error: unfinished rule, read: 'cc'"),
            TokenizerState::E => return Err("Error: unfinished rule, read: 'e'"),
            TokenizerState::EN => return Err("Error: unfinished end"),
            TokenizerState::L => return Err("Error: unfinished rule, read: 'l'"),
            TokenizerState::T => return Err("Error: unfinished rule, read: 't'"),
            TokenizerState::U => return Err("Error: unfinished rule, read: 'u'"),
            TokenizerState::Error => return Err("Error while reading input"),
        }
        Ok(tokens)
    }

    fn state_init(&mut self, config: &Config, c: char, _tokens: &mut Vec<Token>) {
        match c {
            ',' => {
                self.state = TokenizerState::Comma;
            }
            ' ' => {
                self.state = TokenizerState::Space;
            }
            '"' => {
                self.string.clear();
                self.state = TokenizerState::String;
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                self.number.clear();
                self.number.push(c);
                self.state = TokenizerState::Number;
            }
            'c' => {
                self.state = TokenizerState::C;
            }
            'd' => {
                self.state = TokenizerState::D;
            }
            'e' => {
                self.state = TokenizerState::E;
            }
            'i' => {
                self.state = TokenizerState::I;
            }
            'l' => {
                self.state = TokenizerState::L;
            }
            'p' => {
                self.state = TokenizerState::P;
            }
            'r' => {
                self.state = TokenizerState::R;
            }
            's' => {
                self.state = TokenizerState::S;
            }
            't' => {
                self.state = TokenizerState::T;
            }
            'u' => {
                self.state = TokenizerState::U;
            }
            _ => {
                if let Verbosity::Debug = config.verbosity {
                    eprintln!("*Init*")
                };
                self.state = TokenizerState::Error;
            }
        }
    }

    fn state_comma(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        match c {
            ',' => {
                tokens.push(Token::Comma);
                self.state = TokenizerState::Comma;
            }
            ' ' => {
                tokens.push(Token::Comma);
                self.state = TokenizerState::Space;
            }
            '"' => {
                self.string.clear();
                self.state = TokenizerState::String;
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                tokens.push(Token::Comma);
                self.number.clear();
                self.number.push(c);
                self.state = TokenizerState::Number;
            }
            'c' => {
                tokens.push(Token::Comma);
                self.state = TokenizerState::C;
            }
            'd' => {
                tokens.push(Token::Comma);
                self.state = TokenizerState::D;
            }
            'e' => {
                tokens.push(Token::Comma);
                self.state = TokenizerState::E;
            }
            'i' => {
                tokens.push(Token::Comma);
                self.state = TokenizerState::I;
            }
            'l' => {
                tokens.push(Token::Comma);
                self.state = TokenizerState::L;
            }
            'p' => {
                tokens.push(Token::Comma);
                self.state = TokenizerState::P;
            }
            'r' => {
                tokens.push(Token::Comma);
                self.state = TokenizerState::R;
            }
            's' => {
                tokens.push(Token::Comma);
                self.state = TokenizerState::S;
            }
            't' => {
                tokens.push(Token::Comma);
                self.state = TokenizerState::T;
            }
            'u' => {
                tokens.push(Token::Comma);
                self.state = TokenizerState::U;
            }
            _ => {
                if let Verbosity::Debug = config.verbosity {
                    eprintln!("*Comma*")
                };
                self.state = TokenizerState::Error;
            }
        }
    }

    fn state_space(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        match c {
            ' ' => {}
            ',' => {
                tokens.push(Token::Space);
                self.state = TokenizerState::Comma;
            }
            '"' => {
                tokens.push(Token::Space);
                self.string.clear();
                self.state = TokenizerState::String;
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                tokens.push(Token::Space);
                self.number.clear();
                self.number.push(c);
                self.state = TokenizerState::Number;
            }
            'c' => {
                tokens.push(Token::Space);
                self.state = TokenizerState::C;
            }
            'd' => {
                tokens.push(Token::Space);
                self.state = TokenizerState::D;
            }
            'e' => {
                tokens.push(Token::Space);
                self.state = TokenizerState::E;
            }
            'i' => {
                tokens.push(Token::Space);
                self.state = TokenizerState::I;
            }
            'l' => {
                tokens.push(Token::Space);
                self.state = TokenizerState::L;
            }
            'p' => {
                tokens.push(Token::Space);
                self.state = TokenizerState::P;
            }
            'r' => {
                tokens.push(Token::Space);
                self.state = TokenizerState::R;
            }
            's' => {
                tokens.push(Token::Space);
                self.state = TokenizerState::S;
            }
            't' => {
                tokens.push(Token::Space);
                self.state = TokenizerState::T;
            }
            'u' => {
                tokens.push(Token::Space);
                self.state = TokenizerState::U;
            }
            _ => {
                if let Verbosity::Debug = config.verbosity {
                    eprintln!("*Space*")
                };
                self.state = TokenizerState::Error;
            }
        }
    }

    fn state_string(&mut self, _config: &Config, c: char, tokens: &mut Vec<Token>) {
        match c {
            '"' => {
                tokens.push(Token::String {
                    value: self.string.clone(),
                });
                self.string.clear();
                self.state = TokenizerState::Init;
            }
            _ => {
                self.string.push(c);
            }
        }
    }

    fn state_number(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        match c {
            ',' => match self.number.parse::<usize>() {
                Ok(value) => {
                    tokens.push(Token::Number { value });
                    self.state = TokenizerState::Comma;
                }
                Err(_err) => {
                    if let Verbosity::Debug = config.verbosity {
                        eprintln!("*Number* err: {}", _err)
                    };
                    self.state = TokenizerState::Error;
                }
            },
            ' ' => match self.number.parse::<usize>() {
                Ok(value) => {
                    tokens.push(Token::Number { value });
                    self.state = TokenizerState::Space;
                }
                Err(_err) => {
                    if let Verbosity::Debug = config.verbosity {
                        eprintln!("*Number*: err: {}", _err)
                    };
                    self.state = TokenizerState::Error;
                }
            },
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                self.number.push(c);
                self.state = TokenizerState::Number;
            }
            _ => {
                if let Verbosity::Debug = config.verbosity {
                    eprintln!("*Number* c: {}", c)
                };
                self.state = TokenizerState::Error;
            }
        }
    }

    fn state_c(&mut self, config: &Config, c: char, _tokens: &mut Vec<Token>) {
        match c {
            'c' => {
                self.state = TokenizerState::CC;
            }
            _ => {
                if let Verbosity::Debug = config.verbosity {
                    eprintln!("*C*")
                };
                self.state = TokenizerState::Error;
            }
        }
    }

    fn state_cc(&mut self, config: &Config, c: char, _tokens: &mut Vec<Token>) {
        match c {
            'j' => {
                self.state = TokenizerState::CCJ;
            }
            's' => {
                self.state = TokenizerState::CCS;
            }
            _ => {
                if let Verbosity::Debug = config.verbosity {
                    eprintln!("*CC*")
                };
                self.state = TokenizerState::Error;
            }
        }
    }

    fn state_ccj(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::CamelCaseJoin, "*CCJ*")
    }

    fn state_ccs(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::CamelCaseSplit, "*CCS*")
    }

    fn state_d(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        match c {
            ',' => {
                tokens.push(Token::Delete);
                self.state = TokenizerState::Comma;
            }
            ' ' => {
                tokens.push(Token::Delete);
                self.state = TokenizerState::Space;
            }
            'p' => {
                self.state = TokenizerState::DP;
            }
            's' => {
                self.state = TokenizerState::DS;
            }
            'u' => {
                self.state = TokenizerState::DU;
            }
            _ => {
                if let Verbosity::Debug = config.verbosity {
                    eprintln!("*D*")
                };
                self.state = TokenizerState::Error;
            }
        }
    }

    fn state_dp(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::ReplaceDashPeriod, "*DP*")
    }

    fn state_ds(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::ReplaceDashSpace, "*DS*")
    }

    fn state_du(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::ReplaceDashUnder, "*DU*")
    }

    fn state_e(&mut self, config: &Config, c: char, _tokens: &mut Vec<Token>) {
        match c {
            'a' => {
                self.state = TokenizerState::EA;
            }
            'r' => {
                self.state = TokenizerState::ER;
            }
            'n' => {
                self.state = TokenizerState::EN;
            }
            _ => {
                if let Verbosity::Debug = config.verbosity {
                    eprintln!("*E*")
                };
                self.state = TokenizerState::Error;
            }
        }
    }

    fn state_ea(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::ExtensionAdd, "*EA*")
    }

    fn state_er(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::ExtensionRemove, "*ER*")
    }

    fn state_en(&mut self, config: &Config, c: char, _tokens: &mut Vec<Token>) {
        match c {
            'd' => {
                self.state = TokenizerState::END;
            }
            _ => {
                if let Verbosity::Debug = config.verbosity {
                    eprintln!("*EN*")
                };
                self.state = TokenizerState::Error;
            }
        }
    }

    fn state_end(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::End, "*END*")
    }

    fn state_i(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        match c {
            ',' => {
                tokens.push(Token::Insert);
                self.state = TokenizerState::Comma;
            }
            ' ' => {
                tokens.push(Token::Insert);
                self.state = TokenizerState::Space;
            }
            'p' => {
                self.state = TokenizerState::IP;
            }
            't' => {
                self.state = TokenizerState::IT;
            }
            _ => {
                if let Verbosity::Debug = config.verbosity {
                    eprintln!("*I*")
                };
                self.state = TokenizerState::Error;
            }
        }
    }

    fn state_ip(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::InteractivePatternMatch, "*IP*")
    }

    fn state_it(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::InteractiveTokenize, "*IT*")
    }

    fn state_l(&mut self, config: &Config, c: char, _tokens: &mut Vec<Token>) {
        match c {
            'c' => {
                self.state = TokenizerState::LC;
            }
            _ => {
                if let Verbosity::Debug = config.verbosity {
                    eprintln!("*L*")
                };
                self.state = TokenizerState::Error;
            }
        }
    }

    fn state_lc(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::LowerCase, "*LC*")
    }

    fn state_p(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        match c {
            ',' => {
                tokens.push(Token::PatternMatch);
                self.state = TokenizerState::Comma;
            }
            ' ' => {
                tokens.push(Token::PatternMatch);
                self.state = TokenizerState::Space;
            }
            's' => {
                self.state = TokenizerState::PS;
            }
            'd' => {
                self.state = TokenizerState::PD;
            }
            'u' => {
                self.state = TokenizerState::PU;
            }
            _ => {
                if let Verbosity::Debug = config.verbosity {
                    eprintln!("*P*")
                };
                self.state = TokenizerState::Error;
            }
        }
    }

    fn state_pd(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::ReplacePeriodDash, "*PD*")
    }

    fn state_ps(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::ReplacePeriodSpace, "*PS*")
    }

    fn state_pu(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::ReplacePeriodUnder, "*PU*")
    }

    fn state_r(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::Replace, "*R*")
    }

    fn state_s(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        match c {
            ',' => {
                tokens.push(Token::Sanitize);
                self.state = TokenizerState::Comma;
            }
            ' ' => {
                tokens.push(Token::Sanitize);
                self.state = TokenizerState::Space;
            }
            'c' => {
                self.state = TokenizerState::SC;
            }
            'p' => {
                self.state = TokenizerState::SP;
            }
            'd' => {
                self.state = TokenizerState::SD;
            }
            'u' => {
                self.state = TokenizerState::SU;
            }
            _ => {
                if let Verbosity::Debug = config.verbosity {
                    eprintln!("*S*")
                };
                self.state = TokenizerState::Error;
            }
        }
    }

    fn state_sc(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::SentenceCase, "*SC*")
    }

    fn state_sp(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::ReplaceSpacePeriod, "*SP*")
    }

    fn state_sd(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::ReplaceSpaceDash, "*SD*")
    }

    fn state_su(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::ReplaceSpaceUnder, "*SU*")
    }

    fn state_t(&mut self, config: &Config, c: char, _tokens: &mut Vec<Token>) {
        match c {
            'c' => {
                self.state = TokenizerState::TC;
            }
            _ => {
                if let Verbosity::Debug = config.verbosity {
                    eprintln!("*T*")
                };
                self.state = TokenizerState::Error;
            }
        }
    }

    fn state_tc(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::TitleCase, "*TC*")
    }

    fn state_u(&mut self, config: &Config, c: char, _tokens: &mut Vec<Token>) {
        match c {
            'c' => {
                self.state = TokenizerState::UC;
            }
            'd' => {
                self.state = TokenizerState::UD;
            }
            'p' => {
                self.state = TokenizerState::UP;
            }
            's' => {
                self.state = TokenizerState::US;
            }
            _ => {
                if let Verbosity::Debug = config.verbosity {
                    eprintln!("*U*")
                };
                self.state = TokenizerState::Error;
            }
        }
    }

    fn state_uc(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::UpperCase, "*UC*")
    }

    fn state_ud(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::ReplaceUnderDash, "*UD*")
    }

    fn state_us(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::ReplaceUnderSpace, "*US*")
    }

    fn state_up(&mut self, config: &Config, c: char, tokens: &mut Vec<Token>) {
        self.emit_token(config, c, tokens, Token::ReplaceUnderPeriod, "*UP*")
    }

    fn emit_token(
        &mut self,
        config: &Config,
        c: char,
        tokens: &mut Vec<Token>,
        token: Token,
        error_msg: &str,
    ) {
        match c {
            ',' => {
                tokens.push(token);
                self.state = TokenizerState::Comma;
            }
            ' ' => {
                tokens.push(token);
                self.state = TokenizerState::Space;
            }
            _ => {
                if let Verbosity::Debug = config.verbosity {
                    eprintln!("{}", error_msg)
                };
                self.state = TokenizerState::Error;
            }
        }
    }
}

pub fn tokenize(config: &Config, input: &str) -> Result<Vec<Token>, &'static str> {
    Tokenizer::new().run(config, input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_test() {
        let empty: [Token; 0] = [];
        assert_eq!(&empty, tokenize(&Config::new(), "").unwrap().as_slice());
    }

    #[test]
    fn comma_test() {
        assert_eq!(
            &[Token::Comma],
            tokenize(&Config::new(), ",").unwrap().as_slice()
        );
    }

    #[test]
    fn space_test() {
        assert_eq!(
            &[Token::Space],
            tokenize(&Config::new(), " ").unwrap().as_slice()
        );
    }

    #[test]
    fn multiple_spaces_test() {
        assert_eq!(
            &[Token::Space],
            tokenize(&Config::new(), "   ").unwrap().as_slice()
        );
    }

    #[test]
    fn string_test() {
        assert_eq!(
            &[Token::String {
                value: String::from("look, a string")
            }],
            tokenize(&Config::new(), "\"look, a string\"")
                .unwrap()
                .as_slice()
        );
    }
    #[test]
    fn zero_test() {
        assert_eq!(
            &[Token::Number { value: 0 }],
            tokenize(&Config::new(), "0").unwrap().as_slice()
        );
    }
    #[test]
    fn number_test() {
        assert_eq!(
            &[Token::Number { value: 10 }],
            tokenize(&Config::new(), "10").unwrap().as_slice()
        );
    }
    #[test]
    fn large_number_test() {
        assert_eq!(
            &[Token::Number { value: 105 }],
            tokenize(&Config::new(), "105").unwrap().as_slice()
        );
    }

    #[test]
    fn end_test() {
        assert_eq!(
            &[Token::End],
            tokenize(&Config::new(), "end").unwrap().as_slice()
        );
    }

    #[test]
    fn pattern_match_test() {
        assert_eq!(
            &[Token::PatternMatch],
            tokenize(&Config::new(), "p").unwrap().as_slice()
        );
    }

    #[test]
    fn lower_case_test() {
        assert_eq!(
            &[Token::LowerCase],
            tokenize(&Config::new(), "lc").unwrap().as_slice()
        );
    }

    #[test]
    fn upper_case_test() {
        assert_eq!(
            &[Token::UpperCase],
            tokenize(&Config::new(), "uc").unwrap().as_slice()
        );
    }

    #[test]
    fn title_case_test() {
        assert_eq!(
            &[Token::TitleCase],
            tokenize(&Config::new(), "tc").unwrap().as_slice()
        );
    }

    #[test]
    fn sentence_case_test() {
        assert_eq!(
            &[Token::SentenceCase],
            tokenize(&Config::new(), "sc").unwrap().as_slice()
        );
    }

    #[test]
    fn camel_case_join_test() {
        assert_eq!(
            &[Token::CamelCaseJoin],
            tokenize(&Config::new(), "ccj").unwrap().as_slice()
        );
    }

    #[test]
    fn camel_case_split_test() {
        assert_eq!(
            &[Token::CamelCaseSplit],
            tokenize(&Config::new(), "ccs").unwrap().as_slice()
        );
    }

    #[test]
    fn extension_add_test() {
        assert_eq!(
            &[Token::ExtensionAdd],
            tokenize(&Config::new(), "ea").unwrap().as_slice()
        );
    }

    #[test]
    fn extension_remove_test() {
        assert_eq!(
            &[Token::ExtensionRemove],
            tokenize(&Config::new(), "er").unwrap().as_slice()
        );
    }

    #[test]
    fn insert_test() {
        assert_eq!(
            &[Token::Insert],
            tokenize(&Config::new(), "i").unwrap().as_slice()
        );
    }

    #[test]
    fn interactive_tokenize_test() {
        assert_eq!(
            &[Token::InteractiveTokenize],
            tokenize(&Config::new(), "it").unwrap().as_slice()
        );
    }

    #[test]
    fn interactive_pattern_match_test() {
        assert_eq!(
            &[Token::InteractivePatternMatch],
            tokenize(&Config::new(), "ip").unwrap().as_slice()
        );
    }

    #[test]
    fn delete_test() {
        assert_eq!(
            &[Token::Delete],
            tokenize(&Config::new(), "d").unwrap().as_slice()
        );
    }

    #[test]
    fn replace_test() {
        assert_eq!(
            &[Token::Replace],
            tokenize(&Config::new(), "r").unwrap().as_slice()
        );
    }

    #[test]
    fn sanitize_test() {
        assert_eq!(
            &[Token::Sanitize],
            tokenize(&Config::new(), "s").unwrap().as_slice()
        );
    }

    #[test]
    fn replace_space_dash_test() {
        assert_eq!(
            &[Token::ReplaceSpaceDash],
            tokenize(&Config::new(), "sd").unwrap().as_slice()
        );
    }

    #[test]
    fn replace_space_period_test() {
        assert_eq!(
            &[Token::ReplaceSpacePeriod],
            tokenize(&Config::new(), "sp").unwrap().as_slice()
        );
    }

    #[test]
    fn replace_space_underscore_test() {
        assert_eq!(
            &[Token::ReplaceSpaceUnder],
            tokenize(&Config::new(), "su").unwrap().as_slice()
        );
    }

    #[test]
    fn replace_dash_space_test() {
        assert_eq!(
            &[Token::ReplaceDashSpace],
            tokenize(&Config::new(), "ds").unwrap().as_slice()
        );
    }

    #[test]
    fn replace_dash_period_test() {
        assert_eq!(
            &[Token::ReplaceDashPeriod],
            tokenize(&Config::new(), "dp").unwrap().as_slice()
        );
    }

    #[test]
    fn replace_dash_under_test() {
        assert_eq!(
            &[Token::ReplaceDashUnder],
            tokenize(&Config::new(), "du").unwrap().as_slice()
        );
    }

    #[test]
    fn replace_period_space_test() {
        assert_eq!(
            &[Token::ReplacePeriodSpace],
            tokenize(&Config::new(), "ps").unwrap().as_slice()
        );
    }

    #[test]
    fn replace_period_dash_test() {
        assert_eq!(
            &[Token::ReplacePeriodDash],
            tokenize(&Config::new(), "pd").unwrap().as_slice()
        );
    }

    #[test]
    fn replace_period_under_test() {
        assert_eq!(
            &[Token::ReplacePeriodUnder],
            tokenize(&Config::new(), "pu").unwrap().as_slice()
        );
    }

    #[test]
    fn replace_under_space_test() {
        assert_eq!(
            &[Token::ReplaceUnderSpace],
            tokenize(&Config::new(), "us").unwrap().as_slice()
        );
    }

    #[test]
    fn replace_under_dash_test() {
        assert_eq!(
            &[Token::ReplaceUnderDash],
            tokenize(&Config::new(), "ud").unwrap().as_slice()
        );
    }

    #[test]
    fn replace_underscore_period_test() {
        assert_eq!(
            &[Token::ReplaceUnderPeriod],
            tokenize(&Config::new(), "up").unwrap().as_slice()
        );
    }

    #[test]
    fn pattern_match_with_pattern_test() {
        assert_eq!(
            &[
                Token::PatternMatch,
                Token::Space,
                Token::String {
                    value: String::from("{#} - {X}")
                },
                Token::Space,
                Token::String {
                    value: String::from("{1}. {2}")
                },
                Token::Comma,
                Token::LowerCase,
            ],
            tokenize(&Config::new(), "p \"{#} - {X}\" \"{1}. {2}\",lc")
                .unwrap()
                .as_slice()
        );
    }

    #[test]
    fn all_case_changes_test() {
        assert_eq!(
            &[
                Token::LowerCase,
                Token::Comma,
                Token::UpperCase,
                Token::Comma,
                Token::TitleCase,
                Token::Comma,
                Token::SentenceCase,
            ],
            tokenize(&Config::new(), "lc,uc,tc,sc").unwrap().as_slice()
        );
    }

    #[test]
    fn all_replace_changes_test() {
        assert_eq!(
            &[
                Token::ReplaceDashPeriod,
                Token::Comma,
                Token::ReplaceDashSpace,
                Token::Comma,
                Token::ReplaceDashUnder,
                Token::Comma,
                Token::ReplacePeriodDash,
                Token::Comma,
                Token::ReplacePeriodSpace,
                Token::Comma,
                Token::ReplacePeriodUnder,
                Token::Comma,
                Token::ReplaceSpaceDash,
                Token::Comma,
                Token::ReplaceSpacePeriod,
                Token::Comma,
                Token::ReplaceSpaceUnder,
                Token::Comma,
                Token::ReplaceUnderDash,
                Token::Comma,
                Token::ReplaceUnderPeriod,
                Token::Comma,
                Token::ReplaceUnderSpace,
            ],
            tokenize(&Config::new(), "dp,ds,du,pd,ps,pu,sd,sp,su,ud,up,us")
                .unwrap()
                .as_slice()
        );
    }

    #[test]
    fn all_extension_changes_test() {
        assert_eq!(
            &[
                Token::ExtensionRemove,
                Token::Comma,
                Token::ExtensionAdd,
                Token::Space,
                Token::String {
                    value: String::from("txt")
                },
            ],
            tokenize(&Config::new(), "er,ea \"txt\"")
                .unwrap()
                .as_slice()
        );
    }

    #[test]
    fn insert_with_pattern_test() {
        assert_eq!(
            &[
                Token::Insert,
                Token::Space,
                Token::String {
                    value: String::from("text")
                },
                Token::Space,
                Token::End,
                Token::Comma,
                Token::Insert,
                Token::Space,
                Token::String {
                    value: String::from("text")
                },
                Token::Space,
                Token::Number { value: 0 }
            ],
            tokenize(&Config::new(), "i \"text\" end,i \"text\" 0")
                .unwrap()
                .as_slice()
        );
    }
}