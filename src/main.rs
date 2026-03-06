#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
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

    print_media(audiobook);
    print_media(good_movie);
    print_media(bad_book);
}
