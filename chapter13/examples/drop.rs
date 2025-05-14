
/// この関数は `Application` 構造体のインスタンスを作成し、
/// そのインスタンスを別のインスタンスで上書きすることで、
/// 古いインスタンスがドロップされる様子を示します。
///
/// # 機能
/// - `Application` 構造体のインスタンスを作成します。
/// - インスタンスのフィールドに値を設定します。
/// - インスタンスを別のインスタンスで上書きし、古いインスタンスがドロップされることを確認します。
///
/// # 使用例
/// ```
/// drop_application();
/// ```
///
/// この関数は `main` 関数内で呼び出され、実行されます。
use chapter13::drop::Application;

fn drop_application() {
    let mut a = Application {
        name: "Zeus".to_string(),
        nickname: vec![
            "cloud collector".to_string(),
            "king of the gods".to_string(),
        ],
    };

    println!("Before assignment");
    a = Application { // drop here
        name: "Hera".to_string(),
        nickname: vec![],
    };
    println!("at end of block");
}

fn main() {
    // Add your code here
    drop_application();
}
