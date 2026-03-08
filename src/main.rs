#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
    Podcast(u32),
    Placeholder,
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

            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            }

            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media)
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
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

    let podcast = Media::Podcast(10);

    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    // let item = catalog.get_by_index(40);

    // println!("{:#?}", item)

    match catalog.get_by_index(0) {
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("No value here");
        }
    }
}
