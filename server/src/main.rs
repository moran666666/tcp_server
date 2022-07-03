use std ::net::{TcpListener,TcpStream};
use std::{str,thread,time,io::{self,Read,Write}};

fn handle_client(mut stream: TcpStream) -> io::Result<()>{
    let mut buf = [0;512]; // 创建buf可变数组变量，长度512byte，用来存放消息内容
    loop { // 消息读取循环
        let bytes_read = stream.read(&mut buf)?; // 读取消息内容到buf变量中
        if bytes_read == 0{ // 如果读取内容长度为0(即没有内容)，则不处理立即返回
            return Ok(()); // 返回Result类型的Ok空值
        }
        print!("read form client:{}",str::from_utf8(&buf).unwrap()); // 打印client发过来的消息内容

        stream.write(&buf[..bytes_read])?; // 将读取到的内容值，再发送回给client
        println!("write to client:{}",str::from_utf8(&buf).unwrap()); // 打印发送给client的消息内容

        thread::sleep(time::Duration::from_secs(1)); // 睡眠 1 秒
    }
}

fn main() -> io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();  // 绑定ip及端口创建监听器
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new(); // 创建vector变量，用来存放线程
    
    for stream in listener.incoming() { // 循环接受client进来的消息
        let stream = stream.expect("failed"); // 解包消息
 
        let handle = thread::spawn(move || { // 新建线程
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}",error)) // 处理client发过来的消息
        });

        thread_vec.push(handle); // 将线程放进vector变量thread_vec中
    }
 
    for handle in thread_vec { // 遍历读取thread_vec变量中存放的所有线程
        handle.join().unwrap(); // 将线程加入处理完成等待队列中
    }
    Ok(()) // 返回Result类型的Ok空值
}