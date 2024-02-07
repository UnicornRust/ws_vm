
use unix::components::ui;


fn main() {
    win_lib();
    unix_gui();
}

// 加载 win 平台函数
fn win_lib() {
    println!("Hello, world!");
    let label = win::add(2, 3);
    println!("{}", label);
}

// 调用 unix 平台绘制库绘制gui
fn unix_gui() {
    let screen = unix::Screen {
        components: vec![
            Box::new(ui::SelectBox{
                width: 100,
                height: 200,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ]
            }),
            Box::new(ui::Button {
                width: 40,
                height: 60,
                label: String::from("submit"),
            }),
        ],
    };
    screen.run();
}
