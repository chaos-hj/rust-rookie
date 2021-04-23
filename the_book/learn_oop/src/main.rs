fn main() {
    display_screen();
}

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(val) => {
                self.update_average();
                Some(val)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
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
    pub label: String,
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button label is {}", self.label);
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox options is {:?}", self.options);
    }
}

pub fn display_screen() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 10,
                height: 20,
                options: vec![String::from("Yes"), String::from("No")],
            }),
            Box::new(Button {
                width: 10,
                height: 20,
                label: String::from("slamdunk"),
            }),
        ],
    };
    screen.run();
}


use blog::Post;

fn post_lifetime() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch");
    
}