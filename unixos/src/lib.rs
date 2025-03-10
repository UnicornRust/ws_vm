pub mod components;

pub trait Draw {
    fn draw(&self);
}

// 这里 Box<dyn Draw> 为一个 trait 对象，他是Box 中任何实现了Draw trait
// 的类型的替身, 这样的 trait 对象在运行时可以替代多种具体类型
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// 定义 run 方法对每一个组件调用 draw 方法
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 定义泛型类型的参数
// ----------------------
// 如果定义的时 trait bound 的泛型参数，泛型参数一次只能替代一种具体类型，
// 就是说，每次要么全部是 Button 要么全部是 Text
// -----------------------
// >> 如果只需要同质化的集合，则倾向于使用泛型和trait bound, 因为其定义
//    在编译时采用具体类型进行单态化
//
pub struct Canvas<T: Draw> {
    components: Vec<T>,
}

impl<T> Canvas<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}
