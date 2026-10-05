#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
    }
    }
}

pub trait Delimiter {
    fn find_next(&self, s:&str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where 
    D: Delimiter
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?; 
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder){
                let until_delimiter = &remainder[..delim_start];
                *remainder = &remainder[delim_end..];
                Some(until_delimiter)
        } else {
             self.remainder.take()
            }
        } 
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices().find(|(_, c)| c == self).map(|(start,_)| (start, self.len_utf8() + 1))
    }
}

pub fn until_char(s: &str, c: char) -> &'_ str {
    StrSplit::new(s, c).next().expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_works() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "1 2 3 4 5";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["1", "2", "3", "4", "5"].into_iter()));
}
