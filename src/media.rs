mod content;

use content::catalog::Catalog;
use content::media::Media;

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

    let item = catalog.get_by_index(0);

    println!("{:#?}", item.unwrap());

    println!("{:#?}", item.expect("Expected value here"));
}
