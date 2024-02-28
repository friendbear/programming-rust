use chapter13::Application;

fn drop_application() {
    let mut a = Application {
        name: "Zeus".to_string(),
        nickname: vec![
            "cloud collector".to_string(),
            "king of the gods".to_string(),
        ],
    };

    println!("Before assignment");
    a = Application {
        name: "Hera".to_string(),
        nickname: vec![],
    };
    println!("at end of block");

}
fn main() {
    // Add your code here
    drop_application();
}
