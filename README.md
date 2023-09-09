Rust製アプリのための、簡単にResult型をunwrapし、エラーであればダイアログでそれを伝えるためのクレートです。  
そんな規模が大きくないのでcrates.ioには公開していませんが、要望があればそうします。

## 使い方
まず最初にAnyHowの`.context`メソッドでエラーダイアログの説明を設定します。  
そのあと、トレイト`ErrorDialogUnwrapper`をインポートし、`unwrap_or_dialog`を使います。  
使うトレイトはpreludeモジュールにあるもので全て揃います。（これは追加でanyhowのbailといったマクロもインポートします。）
### 例
```rust
use dialog_unwrapper::prelude::*;

fn calculate() -> Result<usize> {
    unimplemented!();
}

fn main() {
    println!(
        "{}",
        calculate()
            .context("とあるプロセスが異常終了しました。")
            .unwrap_or_dialog();
    )
}
```
