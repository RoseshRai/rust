use std::future::Future;
use std::pin::Pin;
use serde_derive::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Debug)]
pub struct BookInfo<T> {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub genre: T,
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct BookSafeDetails<T> {
    pub isbn: String,
    pub title: String,
    pub genre: T,
}

#[derive(PartialEq, Eq, Debug)]
pub struct AuthorInfo {
    pub name: String,
    pub biography: String,
}

#[derive(thiserror::Error, Debug)]
pub enum LibraryError<E> {
    #[error("Unknown book '{0}'")]
    UnknownBook(String),
    #[error("Unknown author '{0}'")]
    UnknownAuthor(String),
    #[error(transparent)]
    Implementation(E),
}

impl<E> From<E> for LibraryError<E> {
    fn from(value: E) -> Self {
        LibraryError::Implementation(value)
    }
}

pub type FutureResult<'a, T, E> = Pin<Box<dyn Future<Output = Result<T, LibraryError<E>>> + 'a>>;


pub trait LibraryManager<T, E> {
    /// Retrieve a book from the library using the book's ISBN.
    ///
    /// Returns either:
    ///
    /// - Ok(BookInfo) if the book exists.
    /// - Error(LibraryError::UnknownBook) if the book does not exist.
    /// - Error(LibraryError::Implementation) for any other issues during the operation.
    fn get_book<'a>(&'a self, isbn: &'a str) -> FutureResult<'a, BookInfo<T>, E>;

    /// Retrieve the list of all books available in the library.
    ///
    /// Returns either:
    ///
    /// - Ok(Vec<BookSafeDetails>) containing details of all books.
    /// - Error(LibraryError::Implementation) if an issue occurs during the operation.
    fn get_books(&self) -> FutureResult<Vec<BookSafeDetails<T>>, E>;

    /// Add a new book to the library database.
    ///
    /// Returns either:
    ///
    /// - Ok(()) if the book has been successfully added.
    /// - Error(LibraryError::Implementation) if any issue occurs during the operation.
    fn add_book<'a>(&'a self, isbn: &'a str, title: &'a str, author: &'a str, genre: T) -> FutureResult<'a, (), E>;

    /// Delete a book from the library database using the book's ISBN.
    ///
    /// Returns either:
    ///
    /// - Ok(()) if the book has been successfully deleted.
    /// - Error(LibraryError::UnknownBook) if the book does not exist.
    /// - Error(LibraryError::Implementation) if any issue occurs during the operation.
    fn delete_book(&self, isbn: String) -> FutureResult<(), E>;

    /// Update the details of an existing book in the library.
    ///
    /// Returns either:
    ///
    /// - Ok(()) if the book details have been successfully updated.
    /// - Error(LibraryError::UnknownBook) if the book does not exist.
    /// - Error(LibraryError::Implementation) if any issue occurs during the operation.
    fn update_book<'a>(&'a self, isbn: &'a str, title: &'a str, author: &'a str, genre: T) -> FutureResult<'a, (), E>;

    /// Retrieve an author from the library using the author's name.
    ///
    /// Returns either:
    ///
    /// - Ok(AuthorInfo) if the author exists.
    /// - Error(LibraryError::UnknownAuthor) if the author does not exist.
    /// - Error(LibraryError::Implementation) if any other issue occurs during the operation.
    fn get_author<'a>(&'a self, name: &'a str) -> FutureResult<'a, AuthorInfo, E>;

    /// Add a new author to the library database.
    ///
    /// Returns either:
    ///
    /// - Ok(()) if the author has been successfully added.
    /// - Error(LibraryError::Implementation) if any issue occurs during the operation.
    fn add_author<'a>(&'a self, name: &'a str, biography: &'a str) -> FutureResult<'a, (), E>;

    /// Delete an author from the library database using the author's name.
    ///
    /// Returns either:
    ///
    /// - Ok(()) if the author has been successfully deleted.
    /// - Error(LibraryError::UnknownAuthor) if the author does not exist.
    /// - Error(LibraryError::Implementation) if any issue occurs during the operation.
    fn delete_author(&self, name: String) -> FutureResult<(), E>;

    /// Update the biography of an existing author in the library.
    ///
    /// Returns either:
    ///
    /// - Ok(()) if the biography has been successfully updated.
    /// - Error(LibraryError::UnknownAuthor) if the author does not exist.
    /// - Error(LibraryError::Implementation) if any issue occurs during the operation.
    fn update_author<'a>(&'a self, name: &'a str, biography: &'a str) -> FutureResult<'a, (), E>;
}
