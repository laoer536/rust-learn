fn main() {
    // 整数类型
    let _i8: i8 = -128;
    let _u8: u8 = 255;
    let _i16: i16 = -32768;
    let _u16: u16 = 65535;
    let _i32: i32 = -2_147_483_648;
    let _u32: u32 = 4_294_967_295;
    let _i64: i64 = -9_223_372_036_854_775_808;
    let _u64: u64 = 18_446_744_073_709_551_615;
    let _i128: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;
    let _u128: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    let _isize: isize = -1_isize;
    let _usize: usize = 1_usize;

    // 浮点类型
    let _f32: f32 = std::f32::consts::PI;
    let _f64: f64 = std::f64::consts::PI;

    // 布尔类型
    let _true = true;
    let _false = false;

    // 字符类型
    let _char = 'A';
    let _emoji = '🚀';

    // 元组类型
    let _tuple = (42, 3.14, 'π');

    // 数组类型
    let _array = [1, 2, 3, 4, 5];
    // let _array:[i32;5] = [1, 2, 3, 4, 5]; 类型声明处第一个位置成员类型 第二个位置成员个数
    let _array_all3 = [3;5]; // [3,3,3,3,3] 5个3组成的数组
    
    let index_array = [0,1,2,3,4];
    let index1_of_array = _array[index_array[1]];

    // 切片类型
    let _slice = &_array[1..4];

    // 字符串切片
    let _str_slice = "Hello Rust";

    // String 类型
    let _string = String::from("Hello World");

    // 智能指针
    let _box = Box::new(2024);

    // 单元类型
    let _unit = ();
    
    let (_x, _y, _z) = _tuple; // 通过解构等方式使用值
    let _ = _array[0] + _slice[0];
    let _ = _str_slice.len() + _string.len();
    let _ = *_box;
    
    
    println!("index1_of_array:{}",index1_of_array);
    println!("x: {}, y: {}, z: {}", _tuple.0,_tuple.1, _tuple.2); // 也可以使用dot方式使用元组的值
    println!("_tuple:{:?}",_tuple);
    println!("_array:{:?}",_array);
    println!("_slice:{:?}",_slice);
    println!("_str_slice:{}",_str_slice);
    println!("_string:{}",_string);
    println!("*_box:{}",*_box);
    println!("_array_all3:{:?}",_array_all3);
    println!("_f32:{}",_f32);
    println!("_f64:{}",_f64);
}
