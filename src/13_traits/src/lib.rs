pub trait Summary {
    fn summarize_title(&self) -> String;

    fn summarize(&self) -> String {
        format!("{}", self.summarize_title())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize_title(&self) -> String {
        format!("{}", self.headline)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize_title(&self) -> String {
        format!("@{}", self.content)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct WeatherForecast {
    pub high_temp: f64,
    pub low_temp: f64,
    pub chance_of_precipitation: f64
}

impl Summary for WeatherForecast {
    fn summarize_title(&self) -> String {
        format!("The chance of precipitation is {}%.", self.chance_of_precipitation * 100.0)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}