
// // Los rasgos en Rust sirven para definir un comportamiento que un tipo debe implementar.
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Podemos pasar un rasgo como argumento de una función.
fn print(summarizable: &impl Summary) {
    println!("{}", summarizable.summarize());
}

// Esto es equivalente a la función anterior, pero con tipos genéricos
fn print2<T: Summary>(summarizable: &T) {
    println!("{}", summarizable.summarize());
}

// La notación de genéricos se utiliza también para definir tiempos de vida. La siguiente función devuelve una referencia al slice de string más largo que se le pasa
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
     if x.len() > y.len() {
        x
     } else {
        y
     }
}


fn main() {
    let article = NewsArticle {
        headline: String::from("Rust Mola"),
        location: String::from("Spain"),
        author: String::from("Fernando Serrano Carpena"),
        content: String::from("Rust es el nuevo lenguaje de moda, porque tiene características increíbles, como la seguridad y la velocidad."),
    };

    print(&article);
    print2(&article);

    let s1 = "Hola";
    let s2 = "Mundo";
    let result = longest(s1, s2);
    println!("The longest string is {}", result);

    let s3 = String::from("Hola Mundo");

    let result = longest(s3.as_str(), s2);
    println!("The longest string is {}", result);
}
