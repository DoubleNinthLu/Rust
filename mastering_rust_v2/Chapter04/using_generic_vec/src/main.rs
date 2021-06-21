fn main() {
    // 提供一种类型
    let v1: Vec<u8> = Vec::new();

    // 或者调用某个方法
    let mut v2 = Vec::new();
    v2.push(2);     // 现在 v2 的类型是 Vec<i32>

    // 或者使用 turbofish 符号
    let v3 = Vec::<u8>::new();
}
