#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rectangle<T> {
    pub left: T,
    pub right: T,
    pub top: T,
    pub bottom: T,
}

impl Rectangle<i32> {
    pub fn into_pointy_rectangle(self) -> [i32; 4] {
        [-self.left, self.right, -self.top, self.bottom]
    }
}

impl<T: Copy> From<[T; 4]> for Rectangle<T> {
    fn from(value: [T; 4]) -> Self {
        Rectangle {
            left: value[0],
            right: value[1],
            top: value[2],
            bottom: value[3],
        }
    }
}
