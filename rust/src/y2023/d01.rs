use crate::Day;

pub fn solvers() -> Day {
    (Some(p01),
     Some(p02),
    )
}

enum ParserState {
    BeforeAnyDigits(u32),
    AfterDigit(u32,u32),
}

pub fn p01(input: &String) -> String {
    let end_state = input.as_bytes().iter().fold(ParserState::BeforeAnyDigits(0), |acc, x| {
        let c = *x as u32;

        if c == '\n' as u32 {
            match acc {
                ParserState::BeforeAnyDigits(_) => acc,
                ParserState::AfterDigit(sum, digit) => ParserState::BeforeAnyDigits(sum + digit),
            }
        } else if c >= '0' as u32 && c <= '9' as u32 {
            let d = c - '0' as u32;
            match acc {
                ParserState::BeforeAnyDigits(sum) => ParserState::AfterDigit(sum + d * 10, d),
                ParserState::AfterDigit(sum, _) => ParserState::AfterDigit(sum, d),
            }
        } else {
            acc
        }
    });

    match end_state {
        ParserState::BeforeAnyDigits(sum) => sum,
        ParserState::AfterDigit(sum, digit) => sum + digit,
    }.to_string()
}

enum LexerState {
    Ignore,
    NewLine,
    Digit(u32, u8),

    E,
    Ei,
    Eig,
    Eigh,
    F,
    Fi,
    Fiv,
    Fo,
    Fou,
    N,
    Ni,
    Nin,
    O,
    On,
    S,
    Se,
    Sev,
    Seve,
    Si,
    T,
    Th,
    Thr,
    Thre,
    Tw,
}

fn lexer(state: LexerState, c: &u8) -> LexerState {
    if *c == '\n' as u8 {
        return LexerState::NewLine
    } else if *c >= '0' as u8 && *c <= '9' as u8 {
        return LexerState::Digit((*c - '0' as u8) as u32, ' ' as u8)
    } else if let LexerState::Digit(_, last_c) = state {
        return lexer(lexer(LexerState::Ignore, &last_c), c)
    }

    match state {
        LexerState::Ignore if *c == 'e' as u8 => LexerState::E,
        LexerState::Ignore if *c == 'f' as u8 => LexerState::F,
        LexerState::Ignore if *c == 'n' as u8 => LexerState::N,
        LexerState::Ignore if *c == 'o' as u8 => LexerState::O,
        LexerState::Ignore if *c == 's' as u8 => LexerState::S,
        LexerState::Ignore if *c == 't' as u8 => LexerState::T,
        LexerState::Ignore                    => LexerState::Ignore,

        LexerState::E      if *c == 'i' as u8 => LexerState::Ei,
        LexerState::Ei     if *c == 'g' as u8 => LexerState::Eig,
        LexerState::Eig    if *c == 'h' as u8 => LexerState::Eigh,
        LexerState::Eigh   if *c == 't' as u8 => LexerState::Digit(8, *c),
        LexerState::F      if *c == 'i' as u8 => LexerState::Fi,
        LexerState::F      if *c == 'o' as u8 => LexerState::Fo,
        LexerState::Fi     if *c == 'v' as u8 => LexerState::Fiv,
        LexerState::Fiv    if *c == 'e' as u8 => LexerState::Digit(5, *c),
        LexerState::Fo     if *c == 'u' as u8 => LexerState::Fou,
        LexerState::Fo                        => lexer(LexerState::O, c),
        LexerState::Fou    if *c == 'r' as u8 => LexerState::Digit(4, *c),
        LexerState::N      if *c == 'i' as u8 => LexerState::Ni,
        LexerState::Ni     if *c == 'n' as u8 => LexerState::Nin,
        LexerState::Nin    if *c == 'e' as u8 => LexerState::Digit(9, *c),
        LexerState::Nin                       => lexer(LexerState::N, c),
        LexerState::O      if *c == 'n' as u8 => LexerState::On,
        LexerState::On     if *c == 'e' as u8 => LexerState::Digit(1, *c),
        LexerState::On                        => lexer(LexerState::N, c),
        LexerState::S      if *c == 'e' as u8 => LexerState::Se,
        LexerState::S      if *c == 'i' as u8 => LexerState::Si,
        LexerState::Se     if *c == 'v' as u8 => LexerState::Sev,
        LexerState::Se                        => lexer(LexerState::E, c),
        LexerState::Sev    if *c == 'e' as u8 => LexerState::Seve,
        LexerState::Seve   if *c == 'n' as u8 => LexerState::Digit(7, *c),
        LexerState::Seve                      => lexer(LexerState::E, c),
        LexerState::Si     if *c == 'x' as u8 => LexerState::Digit(6, *c),
        LexerState::T      if *c == 'h' as u8 => LexerState::Th,
        LexerState::T      if *c == 'w' as u8 => LexerState::Tw,
        LexerState::Th     if *c == 'r' as u8 => LexerState::Thr,
        LexerState::Thr    if *c == 'e' as u8 => LexerState::Thre,
        LexerState::Thre   if *c == 'e' as u8 => LexerState::Digit(3, *c),
        LexerState::Thre                      => lexer(LexerState::E, c),
        LexerState::Tw     if *c == 'o' as u8 => LexerState::Digit(2, *c),
        _                                     => lexer(LexerState::Ignore, c),
    }
}

pub fn p02(input: &String) -> String {
    let (end_state, _) = input.as_bytes().iter().fold((ParserState::BeforeAnyDigits(0), LexerState::Ignore), |(p, l), c| {
        match lexer(l, c) {
            LexerState::NewLine => {
                match p {
                    ParserState::BeforeAnyDigits(sum) => (ParserState::BeforeAnyDigits(sum), LexerState::NewLine),
                    ParserState::AfterDigit(sum, digit) => (ParserState::BeforeAnyDigits(sum + digit), LexerState::NewLine),
                }
            },
            LexerState::Digit(digit, last_c) => {
                match p {
                    ParserState::BeforeAnyDigits(sum) => (ParserState::AfterDigit(sum + digit * 10, digit), LexerState::Digit(digit, last_c)),
                    ParserState::AfterDigit(sum, _) => (ParserState::AfterDigit(sum, digit), LexerState::Digit(digit, last_c)),
                }
            },
            l => (p, l)
        }
    });

    match end_state {
        ParserState::BeforeAnyDigits(sum) => sum,
        ParserState::AfterDigit(sum, digit) => sum + digit,
    }.to_string()
}
