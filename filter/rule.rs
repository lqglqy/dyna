pub struct Rule {
    pub keyword: String
}

impl Rule {
    pub fn get_keyword(self: &Rule) -> String {
        return self.keyword.clone();
    }
}