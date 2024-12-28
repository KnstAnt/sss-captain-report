//
pub struct Page {
    content: String,
    number: Option<usize>,
}
//
impl Page {
    pub fn new(content: String, number: Option<usize>) -> Self {
        Self { content, number }
    }
    //
    pub fn add_next(self, content: String) -> Self {
        let number = Some(self.number.unwrap_or(0) + 1);
        Page::new(self.print() + &content, number)
    }
    //
    pub fn print(self) -> String {
        let mut res = self.content;
        if let Some(number) = self.number {
            res += &format!("<HTML>

<BODY>

<HR SIZE=4 WIDTH=100% NOSHADE ALIGN=CENTER>

<P> <CENTER> {} </CENTER> </P>

</BODY>

</HTML>

<div style=\"page-break-after: always;\"></div>

", number);
        }
        res
    } 
}
