use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn spawn_function() {
    for i in 0..5 {
        println!("spawned thread print:{}", i);
        thread::sleep(Duration::from_millis(10));
    }
}

pub fn thread_test() {
    thread::spawn(spawn_function);

    //推荐写法：使用闭包
    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("closure spawned thread print {}", i);
            thread::sleep(Duration::from_millis(200));
        }
    });

    for i in 0..3 {
        println!("main thread print:{}", i);
        thread::sleep(Duration::from_millis(50));
    }

    handle.join().unwrap();

    //所有权转移
    let s = "hello";
    let handle2 = thread::spawn(move || {
        println!("s in spawned thread:{}", s);
    });
    handle2.join().unwrap();


    //消息传递
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got:{}", received);
}