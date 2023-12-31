use std::{
    fmt::{Debug, Display},
    sync::OnceLock,
};

use anyhow::Error;

pub use rfd;
use rfd::{AsyncMessageDialog, MessageDialog};

/// ダイアログのデフォルトのタイトルです。
pub static DEFAULT_TITLE: OnceLock<String> = OnceLock::new();

/// `get_title`を使って予期せぬエラーのタイトルを取得します。
/// もし設定されていない場合、デフォルトの"Unexpected Error"が取得されます。
pub fn get_title() -> &'static str {
    &DEFAULT_TITLE.get_or_init(|| String::from("Unexpected Error"))
}

pub trait ErrorDialogUnwrapper<T, E = Error>: Sized {
    fn unwrap_or_dialog(self) -> T;
    fn unwrap_or_dialog_with_title(self, title: impl Display) -> T;

    fn ok_unwrap_or_dialog(self) -> Option<T>;
    fn ok_unwrap_or_dialog_with_title(self, title: impl Display) -> Option<T>;
}

fn truncate(text: &str, index: usize) -> &str {
    match text.char_indices().nth(index) {
        Some((index, _)) => &text[..index],
        None => text,
    }
}

pub fn show_error_dialog(title: &str, e: impl Debug, async_: bool) -> (&str, String) {
    let text = format!("{:?}", e);
    let text_for_dialog = format!("{}...", truncate(&text, 253));

    if async_ {
        let dialog = AsyncMessageDialog::new();
        let _ = dialog
            .set_title(title)
            .set_description(&text_for_dialog)
            .show();
    } else {
        let dialog = MessageDialog::new();
        dialog
            .set_title(title)
            .set_description(&text_for_dialog)
            .show();
    };

    (title, text)
}

fn quick_panic((title, text): (&str, String)) -> ! {
    panic!("{}: {}", title, text)
}

impl<T, E: Debug> ErrorDialogUnwrapper<T, E> for Result<T, E> {
    fn unwrap_or_dialog(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => quick_panic(show_error_dialog(get_title(), e, false)),
        }
    }

    fn unwrap_or_dialog_with_title(self, title: impl Display) -> T {
        match self {
            Ok(v) => v,
            Err(e) => quick_panic(show_error_dialog(&format!("{}", title), e, false)),
        }
    }

    fn ok_unwrap_or_dialog(self) -> Option<T> {
        match self {
            Ok(v) => Some(v),
            Err(e) => {
                show_error_dialog(get_title(), e, true);
                None
            }
        }
    }

    fn ok_unwrap_or_dialog_with_title(self, title: impl Display) -> Option<T> {
        match self {
            Ok(v) => Some(v),
            Err(e) => {
                show_error_dialog(&format!("{}", title), e, true);
                None
            }
        }
    }
}

/// 指定されたタイトルと説明でエラー時にダイアログを表示する`unwrap`をラップした関数を生成します。
#[macro_export]
macro_rules! define_unwrapper {
    ( $title:expr, $description:ident ($($arg_name:ident: $arg_type:ty)*) ) => {
        use anyhow::{Context as _, Result};
        use crate::misc::error::ErrorDialogUnwrapper;

        pub fn unwrap_or_dialog<T>(
            target: Result<T> $(, $arg_name: $arg_type)*
        ) -> T {
            target.context($description($($arg_name)*))
                .unwrap_or_dialog_with_title($title)
        }

        pub fn ok_unwrap_or_dialog<T>(
            target: Result<T> $(, $arg_name: $arg_type)*
        ) -> T {
            target.context($description($($arg_name)*))
                .unwrap_or_dialog_with_title($title)
        }
    };
}

pub mod prelude {
    pub use super::ErrorDialogUnwrapper as _;
    pub use crate::define_unwrapper;
    pub use anyhow::{anyhow, bail, Context as _, Error, Result};
}
