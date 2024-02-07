// 智能指针, box 是最基础的智能指针，它的作用仅仅是在 heap 上存储数据
//
#[allow(dead_code)]
struct Cat {
    children: Box<Cat>,
}

#[allow(dead_code)]
struct Animal {}

pub fn add(left: usize, right: usize) -> String {
    let mut s = String::new();
    s.push_str("<");
    s.push_str((left + right).to_string().as_str());
    s.push_str(">");
    return s;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, "<4>");
    }
}
