pub struct Table {
    header: Vec<String>,
    content: Vec<Vec<String>>,
}
//
impl Table {
    //
    pub fn new(
        header: &[&str],
        content: Vec<Vec<String>>,
    ) -> Self {
        Self {
            header: header.iter().map(|s| s.to_string()).collect(),
            content,

        }
    }
    //
    pub fn to_string(self) -> Result<String, crate::error::Error> {
        let mut string = self
            .header
            .iter()
            .map(|s| format!("|{s}"))
            .collect::<String>()
            + "|\n"
            + &(0..self.header.len()).map(|_| "|---").collect::<String>()
            + "|\n";
        self.content.iter().for_each(|v| string += &(v.iter().map(|v| format!("|{v}")).collect::<String>() + "|\n") );
        Ok(string + "  \n")
    }
}
