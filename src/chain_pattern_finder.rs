#[derive(Debug)]
pub struct Chain {
    vec: Vec<String>,
    start_watch: Option<String>,
}

impl Chain {
    pub fn new(vec: Vec<String>) -> Chain {
        Self {
            vec,
            start_watch: None,
        }
    }

    pub fn push(&mut self, element: String) -> Option<(usize, usize)> {
        self.vec.push(element.clone());

        if self.start_watch.is_some() {
            let x = self.start_watch.clone().unwrap();
            if x == element {
                let start = self.vec.iter().position(|s| *s == x).unwrap();
                let end = self.vec.iter().rposition(|s| *s == x).unwrap();

                let slice = &self.vec[start + 1..end + 1];

                for i in 0..end {
                    let another_slice = &self.vec[i..i + slice.len()];

                    if slice == another_slice {
                        return Some((i, i + slice.len() - 1));
                    }
                }

                panic!("?");
            }
        }

        None
    }

    pub fn last(&self) -> &String {
        self.vec.last().unwrap()
    }

    pub fn push_and_start_watch(&mut self, element: String) {
        self.push(element.clone());
        self.start_watch = Some(element.clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::chain_pattern_finder::{Chain};

    #[test]
    fn test_two_element() {
        let mut chain = Chain::new(vec!["11A".to_string()]);
        chain.push("11B".to_string());
        chain.push_and_start_watch("11Z".to_string());

        let t = chain.push("11B".to_string());
        assert!(t.is_none());

        let t = chain.push("11Z".to_string());
        assert_eq!(Some((1, 2)), t);
    }

    #[test]
    fn test_three_element() {
        let mut chain = Chain::new(vec!["22A".to_string()]);

        chain.push("22B".to_string());
        chain.push("22C".to_string());
        chain.push_and_start_watch("22Z".to_string());

        let t = chain.push("22B".to_string());
        assert!(t.is_none());

        let t = chain.push("22C".to_string());
        assert!(t.is_none());

        let t = chain.push("22Z".to_string());
        assert_eq!(Some((1, 3)), t);
    }
}