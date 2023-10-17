struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold_smaller(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
     
        assert!(larger.can_hold_smaller(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold_smaller(&larger))
    }
}

