pub struct AveragedCollection {
    list: Vec<i32>,
    average: Option<f64>,
}

impl AveragedCollection {
    pub fn new() -> Self {
        return Self {
            list: vec![],
            average: None
        };
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                return Some(value);
            }
            None => None,
        }
    }

    pub fn get_average(&self) -> f64 {
        return self.average.unwrap();
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = Some(total as f64 / self.list.len() as f64);
    }
}

pub trait Draw {
    fn draw(&self);
}

struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
