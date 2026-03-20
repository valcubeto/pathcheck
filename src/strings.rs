pub const AFTER_HELP: &str = r#"
    The <FORMAT> argument is a string that may contain
        braces where variables are stored, and a formatting
        for those variables is set after the : character.

    Valid variables are:
        line           When the --enumerate flag is present,
                       it's the current line index (starting
                       from 1)
        path           The path text
        status         The status symbol of the path
        description

    Valid formats are:
        <{char}   Pad to left using {char}
        >{char}   Pad to right using {char}
        -{char}   Center text surrounding with {char}
        0         Pad with zeros ({line} only)

    The default {char} for padding is space (\u0020).

    Escapes are allowed: \{  \:  \r  \n  \t

    Example:
        # Display paths by padding numbers to the
        # left and centering the path.
        --format="{line :>}{ path :-}{description}"
"#;

pub const OK_ICON: &str = "\u{f058}";
pub const ERR_ICON: &str = "\u{f530}";

pub const OK_EMOJI: &str = "\u{2705}";
pub const ERR_EMOJI: &str = "\u{274c}";

pub const OK_TEXT: &str = "OK";
pub const ERR_TEXT: &str = "ERR";
