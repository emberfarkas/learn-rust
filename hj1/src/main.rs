// HJ1 字符串最后一个单词的长度
// 计算字符串最后一个单词的长度，单词以空格隔开，字符串长度小于5000。（注：字符串末尾不以空格为结尾）


fn main() {
    let mut input_string = String::new();
     
    std::io::stdin()
    .read_line(&mut input_string)
    .expect("无法读取输入的内容，请重新启动！");

    let input_arr: Vec<&str> = input_string.trim().split(" ").collect();
    let len = input_arr[input_arr.len() - 1].len();

    println!("lenght of len: {}", len);
}
