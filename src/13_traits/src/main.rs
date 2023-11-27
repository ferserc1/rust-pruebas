use traits::{Summary, Tweet, NewsArticle, WeatherForecast, notify};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };
    notify(&tweet);

    let news = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL.")
    };
    notify(&news);

    let forecast = WeatherForecast {
        high_temp: 64.0,
        low_temp: 42.0,
        chance_of_precipitation: 0.4
    };
    notify(&forecast);
}
