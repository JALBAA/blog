# wiki类型系统笔记


>In programming languages, a type system is a set of rules that assigns a property called type to the various constructs of a computer program, such as variables, expressions, functions or modules.[1] These types formalize and enforce the otherwise implicit categories the programmer uses for algebraic data types, data structures, or other components (e.g. "string", "array of float", "function returning boolean"). The main purpose of a type system is to reduce possibilities for bugs in computer programs[2] by defining interfaces between different parts of a computer program, and then checking that the parts have been connected in a consistent way. This checking can happen statically (at compile time), dynamically (at run time), or as a combination of static and dynamic checking. Type systems have other purposes as well, such as expressing business rules, enabling certain compiler optimizations, allowing for multiple dispatch, providing a form of documentation, etc.


编程语言中，类型系统是一组规则，这组规则给计算机程序中的各种各样的数据结构（如变量、表达式、函数、模块）指定了一个名为类型的属性。这些类型强制其他的一些隐含的数据种类，如抽象数据类型、数据结构或其他组件（如字符串、保存浮点数的数组，返回布尔值的函数）得以形式化的呈现。类型系统的主要目的是，通过给程序的不同部分定义接口，然后检查这些部分的接口是否以一致的类型关联起来，从而减少程序的bug发生率。类型检查可以静态的进行（编译时），也可以动态的进行（运行时），也可以静态动态联合检查。类型系统通常也有其他目的，如规范业务规则，方便特定的编译器优化，提供多派发能力，方便输出格式化文档。


>A type system associates a type with each computed value and, by examining the flow of these values, attempts to ensure or prove that no type errors can occur. The given type system in question determines exactly what constitutes a type error, but in general the aim is to prevent operations expecting a certain kind of value from being used with values for which that operation does not make sense (logic errors). Type systems are often specified as part of programming languages, and built into the interpreters and compilers for them; although the type system of a language can be extended by optional tools that perform added kinds of checks using the language's original type syntax and grammar.


在类型系统中，每一个值都关联一个类型，并且通过检查这些值的控制流，类型系统会尝试保证或证明没有类型错误会发生。类型系统决定了一个类型错误都由什么构成，但是通常来讲，类型系统的目标是阻止一个具有确定类型的值被错误的操作（即逻辑错误）。类型系统通常是程序语言的一个组成部分，并且被集成在编译器或者解释器当中，然而一个语言的类型系统也可以通过某些工具进行拓展，这些工具对语言的原生类型和语法进行更多的类型检查。


>#### Usage overview


>An example of a simple type system is that of the C language. The portions of a C program are the function definitions. One function is invoked by another function. The interface of a function states the name of the function and a list of values that are passed to the function's code. The code of an invoking function states the name of the invoked, along with the names of variables that hold values to pass to it. During execution, the values are placed into temporary storage, then execution jumps to the code of the invoked function. The invoked function's code accesses the values and makes use of them. If the instructions inside the function are written with the assumption of receiving an integer value, but the calling code passed a floating-point value, then the wrong result will be computed by the invoked function. The C compiler checks the type declared for each variable sent, against the type declared for each variable in the interface of the invoked function. If the types do not match, the compiler throws a compile-time error.

C语言是一个类型系统的简单案例。函数定义是C语言的一部分。一个函数被另一个函数调用。一个函数的接口规定了函数的名字和参数列表。一个执行中的函数代码规定了被调用者的名字，并搭载了值的形参变量的名字。被调用函数代码接收并且使用这些值。如果函数规定接收一个整数值，而调用者传入了一个浮点数值，则被调用的函数会产生错误的结果。被调用函数的声明了每个变量的类型，C编译器检查每个传给这个函数的变量的类型声明，如果类型不匹配，编译器会抛出编译时错误。

>A compiler may also use the static type of a value to optimize the storage it needs and the choice of algorithms for operations on the value. In many C compilers the float data type, for example, is represented in 32 bits, in accord with the IEEE specification for single-precision floating point numbers. They will thus use floating-point-specific microprocessor operations on those values (floating-point addition, multiplication, etc.).

编译器通常通过值的静态类型来优化内存存储，还会选择合适的算法去计算这个值。比如，在很多C编译器中，浮点数类型代表了IEEE规定的32位单精度浮点数类型，这样编译器就可以启用浮点数专用的cpu操作。

>The depth of type constraints and the manner of their evaluation affect the typing of the language. A programming language may further associate an operation with various resolutions for each type, in the case of type polymorphism. Type theory is the study of type systems. The concrete types of some programming languages, such as integers and strings, depend on practical issues of computer architecture, compiler implementation, and language design.

类型约束的深度和类型评估的方式，影响着语言的分类。在类型多态性中，编程语言可能对每种类型的每一种特定计算结果关联着一个操作。类型系统是类型理论的研究对象，实际上某些语言的类型系统，比如整数和字符串，依赖于计算机体系结构中的实际问题，以及编译器实现、语言设计。


>#### Fundamentals (基本原则)



>Formally, type theory studies type systems. A programming language must have occurrence to type check using the type system whether at compile time or runtime, manually annotated or automatically inferred. As Mark Manasse concisely put it:[3]

正式的讲，类型理论研究类型系统，编程语言必然会在编译期或运行时使用类型系统进行出现类型检查，无论是通过手动注解类型还是自动推断类型。
正像Mark Manasse说的那样：

>>The fundamental problem addressed by a type theory is to ensure that programs have meaning. The fundamental problem caused by a type theory is that meaningful programs may not have meanings ascribed to them. The quest for richer type systems results from this tension.

>类型理论的基础问题是保证程序有意义（语义？可读性？）。由类型理论产生的基础问题是，本应有意义的程序却有可能没法让他们产生意义。富类型系统正式由于这种矛盾而产生，富类型系统的任务就是为了解决这个矛盾。

>Assigning a data type, termed typing, gives meaning to a sequence of bits such as a value in memory or some object such as a variable. The hardware of a general purpose computer is unable to discriminate between for example a memory address and an instruction code, or between a character, an integer, or a floating-point number, because it makes no intrinsic distinction between any of the possible values that a sequence of bits might mean.[note 1] Associating a sequence of bits with a type conveys that meaning to the programmable hardware to form a symbolic system composed of that hardware and some program.

给一个数据指定一个类型，称作归类（typing），即给一段字节序列（如一个内存中的值）或一些对象（如变量）赋予意义。计算机硬件无法分辨内存地址和代码结构，也无法分辨一个字符和一个整数、或一个浮点数，因为一段字节序列本来就没有任何意义。关联一段字节序列和一个类型, 意味着把可编程硬件变成一个硬件和程序组合而成的一个符号系统。

>A program associates each value with at least one specific type, but it also can occur that one value is associated with many subtypes. Other entities, such as objects, modules, communication channels, and dependencies can become associated with a type. Even a type can become associated with a type. An implementation of a type system could in theory associate identifications called data type (a type of a value), class (a type of an object), and kind (a type of a type, or metatype). These are the abstractions that typing can go through, on a hierarchy of levels contained in a system.

程序中每个值至少关联一个类型，但是很多时候一个值也会关联很多子类型。除值外的其他实体，如对象、模块、通信频道、依赖，也可以和类型关联。甚至类型也可以和类型关联。一个类型系统的实现，理论上可以将数据类型（身份）、类型（class）、元类型（metatype type of type）关联起来。也就是说，归类（typing）这种抽象概念，在一个类型系统的各个层次中都可以使用。

>When a programming language evolves a more elaborate type system, it gains a more finely grained rule set than basic type checking, but this comes at a price when the type inferences (and other properties) become undecidable, and when more attention must be paid by the programmer to annotate code or to consider computer-related operations and functioning. It is challenging to find a sufficiently expressive type system that satisfies all programming practices in a type safe manner.

如果编程语言有一个很精确细腻的类型系统，那么他的类型检查机制会更加精细，但是当类型推断难以判断的时候会花费更大代价，而且开发者也需要在代码类型注解、计算关联操作的思考上，花费更大的精力。在保证类型安全的情况下，定义一个充分的、传神的，能满足所有的编程实践类型系统充满了挑战。

>The more type restrictions that are imposed by the compiler, the more strongly typed a programming language is. Strongly typed languages often require the programmer to make explicit conversions in contexts where an implicit conversion would cause no harm. Pascal's type system has been described as "too strong" because, for example, the size of an array or string is part of its type, making some programming tasks difficult.[4][5] Haskell is also strongly typed but its types are automatically inferred so that explicit conversions are often (but not always) unnecessary.


编译器的类型检测约严格，语言的类型机制也越强。强类型语言经常要去开发者精确定义会话（coversation，人机会话）的上下文，以保证一些隐晦的上下文不会出错。Pascal的类型系统被形容为过强，比如数组或字符串的类型，要求获知他们的长度，这增加了开发的难度。Haskell也是强类型语言，但是他的类型可以被自动的推断出来，由此精确的定义会话就不再是必要的了。


>A programming language compiler can also implement a dependent type or an effect system, which enables even more program specifications to be verified by a type checker. Beyond simple value-type pairs, a virtual "region" of code is associated with an "effect" component describing what is being done with what, and enabling for example to "throw" an error report. Thus the symbolic system may be a type and effect system, which endows it with more safety checking than type checking alone.

编译器通常会实现依赖类型（dependent type）或影响系统（effect system），使得类型检查器更容易验证语言的规范。可视化的代码“域”（region of code）和“影响”组件（effect component）关联，描述了正在发生什么，并且具备了一些额外的能力，比如抛出（throw）一个异常报告。如此，符号系统也可以是一个类型及影响系统，使得这个系统具备比单纯的类型检查具备了更加安全的类型检查机制。

>Whether automated by the compiler or specified by a programmer, a type system makes program behavior illegal if outside the type-system rules. Advantages provided by programmer-specified type systems include:

当超出了类型系统的规则时，无论是自动化的编译器还是某个程序员，类型系统都能使程序的表现非法。对于程序员来说，类型系统的优势如下：

>Abstraction (or modularity) – Types enable programmers to think at a higher level than the bit or byte, not bothering with low-level implementation. For example, programmers can begin to think of a string as a set of character values instead of as a mere array of bytes. Higher still, types enable programmers to think about and express interfaces between two of any-sized subsystems. This enables more levels of localization so that the definitions required for interoperability of the subsystems remain consistent when those two subsystems communicate.

* 抽象（或模块性）- 类型允许程序员从更高层次思考位和字节，不用背底层实现困扰。比如程序员不用再把字符串当成一小组有序字节，而是当成一组字符值的集合去思考。还有，类型允许程序员去思考如何表示两个任意大小的子系统的接口。这使得局部化（定域）可以有更多的表示层次，从而当两个子系统交互的时候，子系统的定义还能具有互通性，使得子系统之间仍保留一致性。

>Documentation – In more expressive type systems, types can serve as a form of documentation clarifying the intent of the programmer. For example, if a programmer declares a function as returning a timestamp type, this documents the function when the timestamp type can be explicitly declared deeper in the code to be an integer type.
Advantages provided by compiler-specified type systems include:

* 文档 -

>Optimization – Static type-checking may provide useful compile-time information. For example, if a type requires that a value must align in memory at a multiple of four bytes, the compiler may be able to use more efficient machine instructions.
Safety – A type system enables the compiler to detect meaningless or probably invalid code. For example, we can identify an expression 3 / "Hello, World" as invalid, when the rules do not specify how to divide an integer by a string. Strong typing offers more safety, but cannot guarantee complete type safety.

* 优化 -