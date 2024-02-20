enum Publication {
    Book(Book),
    Magazine(Magazine),
}

struct Book {
    title: String,
    author: String,
    page_count: u32,
}

struct Magazine {
    title: String,
    issue: String,
    topic: String,
}
fn main(){

    let book1=Book{
        title: String::from("Babil'in En Zengin Adamı"),
        author: String::from("George"),
        page_count: 146,
    };
    let magazine1= Magazine{
        title: String::from("BABA"),
        issue: String::from("2000 ve sonrası"),
        topic: String::from("yaşam"),
    };

    let publications: Vec<Publication> = vec![
        Publication::Book(book1),
        Publication::Magazine(magazine1),
    ];


    // println!("Book: {} author: {}, {} pages", book1.title, book1.author, book1.page_count);
    // println!("Magazine: {} issue: {}, {} topic", magazine1.title, magazine1.issue, magazine1.topic);

    for pub_item in &publications {
        match pub_item {
            Publication::Book(book) => {
                println!("Book: {} author: {}, {} pages", book.title, book.author, book.page_count);
            }
            Publication::Magazine(magazine) => {
                println!("Magazine: {} issue: {}, {} topic", magazine.title, magazine.issue, magazine.topic);
            }
        }
    }


}
