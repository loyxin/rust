
enum List<T>{
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> List<T>{
    fn new() -> List<T> {
        // if add a line use List::*
        // rewrite as Nil
        List::Nil
    }

    fn prepend(self, elem: T) -> List<T> {
        List::Cons(elem, Box::new(self))
    }

    fn len(&self)-> u32{
        match *self {
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }
}

fn main(){
    let mut list = List::<i32>::new();
    list = list.prepend(2);
    list = list.prepend(2);
    list = list.prepend(2);
    println!("length:{}", list.len());
	// xiahuaxian  _ no unused variable warning
    let _x =4;
    let _x = 3.214;
let mut _mutable_integer = 7i32;

    {
        // 被不可变的 `_mutable_integer` 遮蔽
        let  _mutable_integer = _mutable_integer;

        // 报错！`_mutable_integer` 在本作用域被冻结
        _mutable_integer = 50;
        // 改正 ^ 注释掉上面这行 or change mut

        // `_mutable_integer` 离开作用域
    }

    // 正常运行！ `_mutable_integer` 在这个作用域没有冻结
    _mutable_integer = 3;

}
