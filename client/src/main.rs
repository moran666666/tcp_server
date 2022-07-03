use std::io::{self,prelude::*,BufReader,Write};
use std::net::TcpStream;
use std::str;

fn main() -> io::Result<( )> {
    let mut stream = TcpStream::connect("127.0.0.1:1234")?; // 连接到tcp server端
    loop { // client与server的交互循环
        print!("please input: "); // 打印提示标准输入
        io::stdout().flush().unwrap(); // 刷新上面标准输入(标准输出为缓存输出，需要\n才会强制输出，否则需要强制刷新输出)

        let mut input = String::new(); // 创建String可变变量input
        io::stdin().read_line(&mut input).expect("Failed to read"); // 读取标准输入到变量input中
        stream.write(input.as_bytes()).expect("failed to write"); // 将读取到input变量中的内容写入到stream
 
        let mut reader =BufReader::new(&stream); // 创建缓存读取变量reader
        let mut buffer: Vec<u8> = Vec::new(); // 创建vector可变变量buffer

        reader.read_until(b'\n',&mut buffer)?; // 读取消息内容到buffer变量中
        println!("read form server:{}",str::from_utf8(&buffer).unwrap()); // 打印server端发过来的内容
    }
}