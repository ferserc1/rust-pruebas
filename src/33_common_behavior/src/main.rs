pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing '{}' button with width: {}, height: {}", self.label, self.width, self.height)
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing select box with width: {}, height: {}, options: {:?}", self.width, self.height, self.options)
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 100,
                height: 50,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe")
                ]
            }),
            Box::new(Button {
                width: 100,
                height: 50,
                label: String::from("Submit")
            })
        ]
    };

    screen.run();
}
