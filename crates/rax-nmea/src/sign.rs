use std::sync::LazyLock;

use rax_parser::str_parser::rules::{Char, Until};

pub static CHAR_COMMA: LazyLock<Char> = LazyLock::new(|| Char(&','));
pub static CHAR_NEW_LINE: LazyLock<Char> = LazyLock::new(|| Char(&'\n'));
pub static CHAR_M: LazyLock<Char> = LazyLock::new(|| Char(&'M'));

pub static UNTIL_COMMA: LazyLock<Until> = LazyLock::new(|| Until(","));
pub static UNTIL_STAR: LazyLock<Until> = LazyLock::new(|| Until("*"));
pub static UNTIL_NEW_LINE: LazyLock<Until> = LazyLock::new(|| Until("\n"));
