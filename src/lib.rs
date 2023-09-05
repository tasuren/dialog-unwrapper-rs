use std::fmt::{Display, Debug};

use anyhow::Error;

use rfd::{ MessageDialog, AsyncMessageDialog };
use rust_i18n::t;


pub trait ErrorDialogUnwrapper<T, E = Error>: Sized {
    fn unwrap_or_dialog(self) -> T;
    fn unwrap_or_dialog_with_title(self, title: impl Display) -> T;

    fn ok_unwrap_or_dialog(self) -> Option<T>;
    fn ok_unwrap_or_dialog_with_title(self, title: impl Display) -> Option<T>;
}


pub fn show_error_dialog(
    title: String,
    e: impl Debug,
    async_: bool
) -> (String, String) {
    let text = format!("{:?}", e);

    if async_ {
        let dialog = AsyncMessageDialog::new();
        let _ = dialog.set_title(&title)
            .set_description(&text)
            .show();
    } else {
        let dialog = MessageDialog::new();
        dialog.set_title(&title)
            .set_description(&text)
            .show();
    };

    (title, text)
}

fn quick_panic((title, text): (String, String)) -> ! {
    panic!("{}: {}", title, text)
}

impl<T, E: Debug> ErrorDialogUnwrapper<T, E> for Result<T, E> {
    fn unwrap_or_dialog(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => quick_panic(show_error_dialog(
                get_unexpected_error(), e, false
            ))
        }
    }

    fn unwrap_or_dialog_with_title(self, title: impl Display) -> T {
        match self {
            Ok(v) => v,
            Err(e) => quick_panic(show_error_dialog(
                format!("{}", title), e, false
            ))
        }
    }

    fn ok_unwrap_or_dialog(self) -> Option<T> {
        match self {
            Ok(v) => Some(v),
            Err(e) => {
                show_error_dialog(get_unexpected_error(), e, true);
                None
            }
        }
    }

    fn ok_unwrap_or_dialog_with_title(self, title: impl Display) -> Option<T> {
        match self {
            Ok(v) => Some(v),
            Err(e) => {
                show_error_dialog(format!("{}", title), e, true);
                None
            }
        }
    }
}


pub fn get_unexpected_error() -> String {
    t!("common.error.unexpected")
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
    pub use anyhow::{Result, Error, Context as _, Ok, anyhow, bail};
    pub use super::ErrorDialogUnwrapper as _;
    pub use crate::define_unwrapper;
}