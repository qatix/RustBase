use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn read_file() {
    let file_name = String::from("/Users/tang/github/RustBase/start.md");
    let text = fs::read_to_string(file_name).unwrap();
    println!("file content:{}", text);
}

pub fn write_file() -> std::io::Result<()> {
    //打开的文件一定存放在可变的变量中才能使用 File 的方法！
    let file_name = String::from("/Users/tang/github/RustBase/test/ftest.txt");
    let mut file = File::create(file_name).unwrap();
    file.write(b"from rust program").unwrap();

    //追加写
    let mut file2 = OpenOptions::new()
        .append(true)
        .open("/Users/tang/github/RustBase/test/ftest.txt")?;
    file2.write(b"APPEND word")?;


    //读写打开
    let mut file = OpenOptions::new()
        .read(true).write(true).open("/Users/tang/github/RustBase/test/ftest.txt")?;
    file.write(b"COVER")?;

    Ok(()) //不能有分号
}





