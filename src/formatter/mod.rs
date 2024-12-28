pub use page::Page;

pub mod title;
pub mod page;

//
pub struct Formatter {
    current: Option<Page>,
}
//
impl Formatter {
    pub fn new(current: Page) -> Self {
        Self { current: Some(current) }
    }
    //
    pub fn print(self) -> String {
        self.current.unwrap().print()
    }
    //
    pub fn add_page(mut self, content: String) -> Self {
        self.current = Some(self.current.take().unwrap().add_next(content));
        self
    }
}