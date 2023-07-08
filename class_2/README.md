# Rust 语言中的所有权

Rust 中的所有权是个非常独特的性质，值得我们好好理解。Rust 通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查，不会影响运行时的效率。

## 所有权的基本规则
Rust 文档里面说明了所有权的[基本规则](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules)。

- Rust 中的每一个值都有一个所有者；
- 每个值在一个时刻只有一个所有者；
- 当所有者离开作用域，这个值将被丢弃。

## 变量作用域

变量的从它的声明到当前作用域结束时是有效的，作用域用 `{ }` 标记，例如

```
{
    let a = 10u32;
    println!("{a}");
}
```

变量 `a` 只在 `{ }` 范围内有效。

## 拷贝 (Copy)、转移 (Move) 和克隆 (Clone)

### 栈和堆

在理解拷贝和转移之前，建议大家先了解 [栈 (Stack) 和堆 (Heap) ](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap)。简言之，栈和堆都是程序运行可以使用的内存，区别在于：

- 栈内存具有后进先出 (LIFO) 的特性，大小在编译期确定，分配速度快，变量的生命周期只在当前函数栈有效；
- 堆内存则是动态分配，容量远大于栈内存，一般由程序员负责分配和释放来管理变量的生命周期。

不同语言对它们的使用方法有所不同，大家可以自行学习。

### RAII

如果大家从 C++ 的世界过来的话可以去了解一下 [RAII(Resource Acquisition Is Initialization)](https://en.cppreference.com/w/cpp/language/raii)，它可以简单理解为使用局部对象来管理资源。

Rust 和 C++ 对 RAII 的支持不同在于：
- C++ 中 RAII 是可选的，程序员可以自行选择是否使用；
- 在 Rust 中，RAII 升级成为了语言级别的默认模式。

### 拷贝 (Copy)

对栈上的数据对象进行赋值操作时，Rust 会使用**拷贝**，包括如下数据类型：
- 所有整数类型，比如 u32；
- 布尔类型，bool，它的值是 true 和 false；
- 所有浮点数类型，比如 f64；
- 字符类型，char；
- 实现了 Copy [Trait](https://doc.rust-lang.org/book/ch10-02-traits.html) 的类型；
- 包含以上类型的元组。

在对此类对象进行赋值操作是，Rust 会复制另一份数据对象。

例如如下代码，能正确编译。

```
{
    let a = 10u32;
    let b = a;

    println!("{a}");
    println!("{b}");
}
```
因为变量 `a` 所持有的的数据的所有权并没有转移到变量 `b`，而是进行了拷贝，也就是说 `a` 和 `b` 持有各自对应的数据。

### 转移 (Move)

对非以上数据类型（一般会是持有堆上数据的对象）进行赋值操作时，Rust 则使用**转移**，此时数据所有权发生转移。

一个典型的例子就是 String 类型，如下示例代码就不能正确编译。

```
{
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}");
    println!("{s2}");
}
```

原因是由于变量 `s1` 所持有的 String 对象的所有权转移给了变量 `s2`，则后面不能继续使用 `s1`。

### 克隆 (Clone)

如果对于默认发生所有权转移的对象，需要进行数据复制（也就是我们常说的深拷贝），在Rust中可以使用[克隆](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-clone)方法。

所以上述示例可以做如下修改：

```
{
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{s1}");
    println!("{s2}");
}
```

## 引用 (Reference) 和借用 (Borrow)

前面我们了解到关于 String 对象，赋值会让它的所有权发生转移，这就会带来如下代码所描述的一个问题：

```
fn main() {
    let s = String::from("hello");
    takes_ownership(s);

    println!("{}", s);
}

fn takes_ownership(local_string: String) {
    println!("{}", local_string);
}
```

这段代码不能通过编译，是因为：
1. 字符串 `s` 的所有权在调用函数 `takes_ownership()` 的时候从变量 `s` 转移给了局部变量 `local_string`；
2. 在函数调用结束的时候，`local_string` 的作用域结束、然后被释放；
3. 此时 `println!()` 再尝试访问 `s` 就会报错。

而如上场景是我们在编程时经常要用到的场景，我们可以通过将 `local_string` 返回来绕过该问题，但这不是个好的解决办法（代码也不简洁干净、也会多出一些无谓的浅拷贝）。因此 Rust 引入了引用和借用来解决这个问题。

大家可以想想为什么如下代码不会有 `takes_ownership()` 同样的问题。

```
{
    let mut s = String::from("hello");
    println!("{}", s);
    
    s.push_str(" world.");
}
```

为什么在调用了 `println!` 之后，还可以使用 `s` ？大家可以先思考下，我们会在稍后 [println!](#println) 进行具体分析。

### 引用和借用

**引用**类似于 C++ 里的指针（当然 C++ 也有引用），因为都是指向一个地址。通过引用我们可以访问该地址所指向的内存地址里的数据，但并不拥有其所有权。

**借用**则可以理解为创建引用的这个行为，它和引用可以认为是一件事情的两个方面，总是会成对出现。

引用的作用域：
- 从引用定义直到它最后一次使用；
- 另外需要牢记的是引用的作用域不会长于、也不能长于所指向数据的作用域，否则就会出现悬垂引用 ([Dangling References](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references))。  
在 C、C++ 中很容易产生悬垂指针，大量的工作都用来处理类似的问题；而 Rust 则从编译器层次解决这个问题，而不是遗留到运行时，这应该是 Rust 一个很优秀的地方。

以下是一个创建引用的示例：

```
{
    let s = String::from("hello");
    let r = &s;

    println!("{}", r);
}
```

例中创建了一个不可变引用 `r` 指向变量 `s` 所持有的字符串，在后续的代码中可以通过 `r` 来访问对应的字符串；我们也可以说 `r` 从 `s` 借用了字符串。


到这里如何解决前面 `takes_ownership()` 所面临的问题也很直观了，我们可以重写代码如下：

```
fn main() {
    let s = String::from("hello");
    takes_ownership(&s);

    println!("{}", s);
}

fn takes_ownership(local_string: &String) {
    println!("{}", local_string);
}
```

通过在函数声明和调用的地方都使用引用，我们很优雅的解决了这个问题。

### 不可变引用 & 可变引用

**不可变引用**就是告诉编译期不能通过该引用去修改所指向的数据，所以它可以同时创建很多个。

**可变引用**则是创建了一个可以用于修改所指向数据的引用，这样我们不需要获得数据的所有权、同时又可以修改数据。如下就是一个如何创建并使用可变引用的例子。

```
{
    let mut s = String::from("hello");
    
    let r = &mut s;
    r.push_str(" world.");
    
    println!("{}", r);
}
```

Rust 里面只允许同一时刻只有一个对某个变量的可变引用，这样做最大的好处就是可以在编译期就检测到可能会发生的数据竞争 (Data Racing)，从而避免并发带来的不可预期的错误。这样我们也很容易推导出可变引用不能被复制、只能转移，否则就会出现对一个变量的多个可变引用。

因此，如下所示的代码是不能通过编译的。

```
{
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
```

另外，Rust 对可变引用和不可变引用的作用域也会有些要求：
1. 它们的作用域都是从引用定义直到它最后一次使用；
2. 不可变引用可以同时存在多个；
3. 可变引用同一时刻只允许存在一个；
4. 同一资源的可变引用和其它引用（包括可变、不可变引用）的作用域不能有重叠 (overlap)。（其实这个隐含了3）

### println!

这里我们分析下为什么以下代码不会出现 `s` 在 `println!` 中被释放的问题。

```
{
    let mut s = String::from("hello");
    println!("{}", s);
    
    s.push_str(" world.");
}
```

根本原因是 `println!` 不是一个函数，而是一个宏 ([Macro](https://doc.rust-lang.org/book/ch19-06-macros.html#macros))。同许多其他语言一样，Rust 的宏是在编译期处理、并被替换为相应的代码。所以这里 `println!` 并不会触发一次函数调用，也就不会有函数栈，继而不会有 `s` 的所有权被转移给函数栈上的局部变量，也不会导致 `s` 指向的数据被释放。另外透露下，其实 `println` 中使用了对 `s` 的一个不可变引用。
