#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }

            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }

            Media::AudioBook { title } => {
                format!("Audiobook: {}", title)
            }
        }
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::AudioBook {
        title: String::from("John"),
    };

    let good_movie = Media::Movie {
        title: String::from("Good movie"),
        director: String::from("Good director"),
    };

    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad author"),
    };

    println!("{}", audiobook.description());
    println!("{}", good_movie.description());
    println!("{}", bad_book.description());
}
