# RustBase



# Tips
## 数组打印
```
println!("d_arr={:?}",arr);
```

# 命名
使用下划线驼峰，不使用大小写


# 注释
```
// 这是第一种注释方式

/* 这是第二种注释方式 */

/*
 * 多行注释
 * 多行注释
 * 多行注释
 */
```


# 所有权
https://www.runoob.com/rust/rust-ownership.html

所有权有以下三条规则：
```
Rust 中的每个值都有一个变量，称为其所有者。
一次只能有一个所有者。
当所有者不在程序运行范围时，该值将被删除。
```
变量范围
```
{
    // 在声明以前，变量 s 无效
    let s = "runoob";
    // 这里是变量 s 的可用范围
}
// 变量范围已经结束，变量 s 无效
```

# 模块组织
```
mod nation {
    mod government {
        fn govern() {}
    }
    mod congress {
        fn legislate() {}
    }
    mod court {
        fn judicial() {}
    }
}
```

# 闭包
```
|参数1, 参数2, ...| -> 返回值类型 {
    // 函数体
}

fn main() {
    let inc = |num: i32| -> i32 {
        num + 1
    };
    println!("inc(5) = {}", inc(5));
}
```