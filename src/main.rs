
use std::cmp::Ordering;

struct Left {
    val: u32
}

struct Right {
    foo: u32
}

impl PartialEq<Right> for Left {
  fn eq(&self, other: &Right) -> bool {
      return self.val == other.foo;
  }
}

impl PartialEq for Left {
  fn eq(&self, other: &Self) -> bool {
      return self.val == other.val;
  }
}

impl PartialOrd<Right> for Left {
    fn partial_cmp(&self, other: &Right) -> Option<Ordering> {
        if self.val == other.foo {
            return Some(Ordering::Equal);
        } else if self.val > other.foo {
            return Some(Ordering::Greater);
        } else {
            return Some(Ordering::Less);
        }
    }
}

impl PartialOrd for Left {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.val == other.val {
            return Some(Ordering::Equal);
        } else if self.val > other.val {
            return Some(Ordering::Greater);
        } else {
            return Some(Ordering::Less);
        }
    }
}

fn main() {
    let a = Left { val: 100 };
    let b = Right { foo: 200 };
    println!("Ans = {:?}", a < b);
}
