use crate::Draw;

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

//
impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!(
            "draw a {} x {} button with label: {}",
            self.width, self.height, self.label
        );
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "draw a {} x {} SelectBox with a options: {}",
            self.width,
            self.height,
            self.options.len()
        )
    }
}
