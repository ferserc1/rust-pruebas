
#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: Vec::new(),
            average: 0.0
        }
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
                Some(value)
            }
            None => None
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

fn main() {
    let mut col = AveragedCollection::new();

    col.add(1);
    col.add(2);
    col.add(3);
    col.add(4);
    col.add(10);
    col.add(9);
    col.add(8);
    col.add(7);
    col.add(6);
    col.add(5);

    println!("Average: {}", col.average());
    println!("List: {:?}", col);
}
