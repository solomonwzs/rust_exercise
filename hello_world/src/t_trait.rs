use std::clone::Clone;

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button: w: {}, h: {}, l: {}",
                 self.width, self.height, self.label);
    }
}

impl Clone for Button {
    fn clone(&self) -> Self {
        Button {
            width: self.width,
            height: self.height,
            label: self.label.clone(),
        }
    }

    fn clone_from(&mut self, src: &Self) {
        self.width = src.width;
        self.height = src.height;
        self.label = src.label.clone();
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw select box: w: {}, h: {}, opt: {:?}",
                 self.width, self.height, self.options);
    }
}

fn test_trait() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_work() {
        println!(">>>");
        test_trait();
    }
}
