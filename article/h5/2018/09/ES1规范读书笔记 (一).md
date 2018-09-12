# ES1规范读书笔记 (一)

概览

## 1. 语言概览

原文：

>Language Overview
>
>The following is an informal overview of ECMAScript—not all parts of the language are described. This overview
>is not part of the standard proper.
>ECMAScript is object-based: basic language and host facilities are provided by objects, and an ECMAScript
>program is a cluster of communicating objects. An ECMAScript object is an unordered collection of properties each
>with 0 or more attributes which determine how each property can be used—for example, when the ReadOnly
>attribute for a property is set to true, any attempt by executed ECMAScript code to change the value of the property
>has no effect. Properties are containers that hold other objects, primitive values, or methods. A primitive value is a
>member of one of the following built-in types: Undefined, Null, Boolean, Number, and String; an object is a
>member of the remaining built-in type Object; and a method is a function associated with an object via a property.
>ECMAScript defines a collection of built-in objects which round out the definition of ECMAScript entities. These
>built-in objects include the Global object, the Object object, the Function object, the Array object, the String
>object, the Boolean object, the Number object, the Math object, and the Date object.
>ECMAScript also defines a set of built-in operators which may not be, strictly speaking, functions or methods.
>ECMAScript operators include various unary operations, multiplicative operators, additive operators, bitwise shift
>operators, relational operators, equality operators, binary bitwise operators, binary logical operators, assignment
>operators, and the comma operator.
>ECMAScript syntax intentionally resembles Java syntax. ECMAScript syntax is relaxed to enable it to serve as an
>easy-to-use scripting language. For example, a variable is not required to have its type declared nor are types
>associated with properties, and defined functions are not required to have their declarations appear textually before
>calls to them

要点：

* ES基于对象（object-based），或者说一切皆对象
* ES对象(object)是一堆属性（property）的无序集合
* 属性（property）具有特性（property），特性决定属性（property）如何使用。
* 属性（property）又是对象（object）、原语值（primitive value）、方法（method）的容器。
* 一个原语值（primitive value）是5种内置类型（built-in type）之一（除了Object）
* 一个对象（object）是内置类型（Object）的成员
* 内置类型（built-in type）为 Undefined, Null, Boolean, Number, String, Object
* 方法（method）是一个关联到一个对象（object）的属性（property）上的函数（function）
* 内置对象（built-in objects）包括the `Global(window)` object, the `Object` object, the `Function` object, the `Array` object, the `String` object, the `Boolean` object, the `Number` object, the `Math` object, and the `Date` object.
* 内置操作符（built-in operators）严格来说不是方法或函数，用来进行各种操作
* 内置操作符（built-in operators）的操作有一元、乘法、加法、位、关联、相等性、逻辑与、逻辑异或、赋值（assignment）、逗号操作
* ES语法有意的模仿java语法，但语法松散，以便不懂编程的人也能使用


## 1. 对象（Objects）概念简介

原文

>ECMAScript does not contain proper classes such as those in C++, Smalltalk, or Java, but rather, supports
>constructors which create objects by executing code that allocates storage for the objects and initializes all or
>part of them by assigning initial values to their properties. All functions including constructors are objects, but
>not all objects are constructors. Each constructor has a Prototype property which is used to implement
>prototype-based inheritance and shared properties. Objects are created by using constructors in new
>expressions, for example, new String(“A String”) creates a new string object. Invoking a constructor
>without using new has consequences that depend on the constructor. For example, String(“A String”)
>produces a primitive string, not an object.
>ECMAScript supports prototype-based inheritance. Every constructor has an associated prototype, and every
>object created by that constructor has an implicit reference to the prototype (called the object’s prototype)
>associated with its constructor. Furthermore, a prototype may have a non-null implicit reference to its prototype,
>and so on; this is called the prototype chain. When a reference is made to a property in an object, that reference
>is to the property of that name in the first object in the prototype chain that contains a property of that name. In
>other words, first the object mentioned directly is examined for such a property; if that object contains the named
>property, that is the property to which the reference refers; if that object does not contain the named property, the
>prototype for that object is examined next; and so on.
>In a class-based object-oriented language, in general, state is carried by instances, methods are carried by classes,
>and inheritance is only of structure and behavior. In ECMAScript, the state and methods are carried by objects,
>and structure, behavior, and state are all inherited.
>All objects that do not directly contain a particular property that their prototype contains share that property and
>its value. The following diagram may illustrate this discussion:
>
>

要点：

* ES没有类的概念，但是支持通过构造函数（constructor）创建对象
* 构造函数（constructor）为对象（object）创建内存空间，同时为对象（object）初始化全部或部分的属性（property）
* 所有函数（function），包括构造函数（constructor），都是对象（object）
* 不是所有对象（object）都是构造函数（constructor）
* 每个构造函数（constructor）都有一个名为`Prototype`的属性（property）
* `Prototype`属性（property）用来实现基于原型的继承（prototype-based inheritance）个特性共享(shared properties)
* 对象（object）是通过使用`new表达式`执行构造函数（constructor）进行创建的，例：`typeof String("foo") == 'string'` , `typeof new String('foo') == 'object'`
* ES支持原型继承（prototype-based inheritance）
* 每个构造函数（constructor）都有一个关联的属性（property）`Prototype`，每个通过这个构造函数创建的对象，都对他的构造函数（constructor）的`Prototype`有一个隐式（implicit）的引用。
* 一个`prototype`也可能有一个`非null`的引用，然后再引用、引用、等等，形成一个链条，即原型链（prototype chain）。
* 如果有对某一个对象(object)的一个属性（property）有一个引用(renference)，先找原型链(prototype)上距离这个物体最近的同名引用。
* 基于类的面向对象语言中，实例(instance)搭载状态(state)，类（class）搭载方法（method），只有结构和行为能够被继承。
* ES中，状态(state)和方法(methods)都挂载在对象（object）上，结构、行为、**状态**也都是可继承的。
* 如果有一些对象，他们本身没有某个属性（property），但是他们的原型的原型链上，有某个对象（或这说原型）持有这个属性（property），那么这些对象，分享这个属性（property）和他的值。详见例图。

原文-原型链例图：

>![例图](/static/images/blog/es1-4.2.1.png)
>
>CF is a constructor (and also an object). Five objects have been created by using new expressions: cf1, cf2, cf3,
>cf4, and cf5.Each of these objects contains properties named q1 and q2. The dashed lines represent the implicit
>prototype relationship; so, for example, cf3’s prototype is CFp. The constructor, CF, has two properties itself,
>named P1 and P2, which are not visible to CFp, cf1, cf2, cf3, cf4, or cf5. The property named CFP1 in CFp is
>shared by: cf1, cf2, cf3, cf4, and cf5, as are any properties found in CFp’s implicit prototype chain which are not
>named q1, q2, or CFP1. Notice that there is no implicit prototype link between CFp and CF.
>Unlike class-based object languages, properties can be added to objects on the fly simply by assigning values to
>them. That is, constructors are not required to name or assign values to all or any of its properties. In the above
>diagram, one could add a new shared property for cf1, cf2, cf3, cf4, and cf5 by assigning a new value to the
>property in CFp.


要点-图解：

* CF是构造函数（当然也是个对象）。
* cf1-cf5是通过`new`表达式创建出来的对象。
* CF和CFP不是隐式的关联关系，CFP是CF的prototype属性承载的对象，及CF的prototype是CFP的一个引用(reference)。
* cf1-cf5和CFP有一个隐式的关联关系，也就是原型链。
* cf1-cf5共享CFP的CFP1属性（property）。
* CF的p1、p2不能被共享
* cf1-cf5各自的q1、q2不共享
* 不同于基于类的OO语言，属性property可以直接挂载到某个对象上，而不需要一定通过构造函数（constructor）创建。

## 3. 关键概念定义


#### Type（类型）

>A type is a set of data values. In general, the correct functioning of a program is not affected if different data
>values of the same type are substituted for others.

* 一个类型就是一组数据值的一种统称。通常来讲，相同的类型下的不同的数据值互相替换时，程序不会出错。

#### Primitive value（原语值）

>A primitive value is a member of one of the types Undefined, Null, Boolean, Number, or String. A primitive
>value is a datum which is represented directly at the lowest level of the language implementation.

* 一个原语值就是括号中5种类型（`Undefined`, `Null`, `Boolean`, `Number`, `String`）的其中一种类型的值。
* 一个原语值就是一个能直接体现语言最底层实现的数据。



#### Object（对象）

>An object is a member of the type Object. It is an unordered collection of properties which contain primitive
>values, objects, or functions. A function stored in the property of an object is called a method.

* 一个对象就是`Object`类型的一个值。
* 对象是一组属性（properties）的无序集合
* 属性（property）是一个引用（也可以理解成容器），可以指向原语值（primitive value）、对象（object）或函数（function）。
* 如果一个属性（property）引用的是一个函数（function），则这个函数也被称为一个方法（method）

#### Constructor（构造函数）

>A constructor is a function object which creates and initializes objects. Each constructor has an associated
>prototype object which is used to implement inheritance and shared properties.

* 构造函数是函数，函数是对象。
* 构造函数是用来创建和初始化对象的。
* 每一个构造函数都关联着一个用来实现继承和共享属性的原型对象。

#### Prototype（原型）

>A prototype is an object used to implement structure, state, and behavior inheritance in ECMAScript. When a
>constructor creates an object, that object implicitly references the constructor’s associated prototype for the
>purpose of resolving property references. The constructor’s associated prototype can be referenced by the
>program expression constructor.prototype, and properties added to an object’s prototype are shared,
>through inheritance, by all objects sharing the prototype.

* 原型是一个用来实现结构、状态、表现的继承能力的对象。
* 构造函数创建的对象，隐式的引用了构造函数的关联原型对象。
* 构造函数的关联原型对象，可以通过`constructor.prototype`来获得其引用。
* 原型链上的上次原型对象的属性（property）一旦出现变动，下游的对象也会共享这些变动。

#### Native object（本地对象、原生对象）

>A native object is any object supplied by an ECMAScript implementation independent of the host environment.
>Standard native objects are defined in this specification. Some native objects are built-in; others may be
>constructed during the course of execution of an ECMAScript program.

* 本地对象是ES规范中提及的对象。
* 和宿主环境无关
* 可能是内置对象，也可能是在程序运行时通过内置对象创建的新对象。

#### Built-in object（内置对象）

>A built-in object is any object supplied by an ECMAScript implementation, independent of the host
>environment, that is present at the start of the execution of an ECMAScript program. Standard built-in objects are
>defined in this specification, and the ECMAScript implementation may specify and define others. Every built-in
>object is a native object.

* 内置对象也是ES规范中提及的对象。
* 和宿主环境无关
* 内置对象在程序执行的最开始阶段即暴露并可用
* 标准内置对象是指的规范中提及的这些，但是有些ES的实现，可能会自定义新的内置对象。
* 内置对象也属于本地对象

#### Host object（宿主对象）
  
>A host object is any object supplied by the host environment to complete the execution environment of
>ECMAScript. Any object that is not native is a host object.

* 宿主环境提供的对象
* 比如window下面的属性（property）所引用的对象
* 对象不是本地对象就是宿主对象

#### Undefined（undefined值）

>Undefined is a primitive value used when a variable has not been assigned a value.

* undefined是一个原语值
* 当一个变量没有被赋值时，变量的值为undefined

#### Undefined type（Undefined类型）

>The type Undefined has exactly one value, called undefined.

* Undefined类型只有一个值，就是undefined

#### Null（null值）

>Null is a primitive value that represents the null, empty, or nonexistent reference.

* null是一个原语值，表示空，不存在的引用等概念

#### Null type（Null类型）

>The type Null has exactly one value, called null.

* Null类型只有一个值，是null

#### Boolean value（布尔值）

>A boolean value is a member of the type Boolean and is one of either two unique values, true and false.

* 布尔值是Boolean类型的值，只有两个值，不是`true`就是`false`

#### Boolean type（布尔类型）

>The type Boolean represents a logical entity and consists of exactly two unique values. One is called true and the
>other is called false.

* 布尔类型表示了一个与非逻辑
* 只有两个值，`true` `false`

#### Boolean object（布尔对象）

>   A Boolean object is a member of the type Object and is an instance of the Boolean object which is a constructor.
    That is, a boolean object is created by using the Boolean constructor in a new expression, supplying a boolean as
    an argument. The resulting object has an implicit (unnamed) property which is the boolean. A boolean object can
    be coerced to a boolean value. A boolean object can be used anywhere a boolean value is expected.
    This is an example of one of the conveniences built into ECMAScript—in this case it is to accommodate
    programmers of varying backgrounds. Those familiar with imperative or procedural programming languages may
    find number values more natural, while those familiar with object-oriented languages may find number objects
    more intuitive

* Boolean对象是一个Object类型的成员（值、实例）
* Boolean是一个构造函数
* 对Boolean使用new表达式能创建一个对象（后文用小b boolean表示），new的时候可以传入boolean值
* 创建出来的这个对象有一个隐式的（没有名称的）属性（property），这个属性是一个boolean值
* 一个boolean对象可以被当做一个boolean值强制使用，即boolean值和boolean对象可以交叉使用
* 这种情况是为了适应不同背景的程序员的，比如熟悉过程语言的会更习惯boolean是一个值，而熟悉OO语言的会更习惯boolean是一个对象

#### String value（字符串值）

>A string value is a member of the type String and is the set of all finite ordered sequences of zero or more
Unicode characters.

* String类型的一个成员（值）
* 有限、有序、可以为0个字符的的unicode字符序列

#### String type（字符串类型）

>The type String is the set of all finite ordered sequences of zero or more Unicode characters.

* 所有有限、有序、可以为0个字符的的unicode字符序列都是String类型的

#### String object（字符串对象）

>A string object is a member of the type Object and is an instance of the String object which is a constructor. That
is, a string object is created by using the String constructor in a new expression, supplying a string as an
argument. The resulting object has an implicit (unnamed) property which is the string. A string object can be
coerced to a string value. A string object can be used anywhere a string value is expected.

* String对象Object类型的一个成员（值、实例）
* String对象是一个构造函数
* new String可以创建一个对象（后文用小S string对象表示）。
* String构造函数接受String类型的参数
* string对象隐式的有一个属性（property），属性是一个String值
* string对象可以和string值交叉使用。
* 这种情况是为了适应不同背景的程序员的，比如熟悉过程语言的会更习惯string是一个值，而熟悉OO语言的会更习惯string是一个对象

#### Number value （数值）

>A number value a member of the type Number and is a direct representation of a number.

* 数值，是Number类型的一员
* 数值直接表示一个数，数学意义的数

#### Number type （数类型）

>The type Number is a set of values representing numbers. In ECMAScript the set of values represent the doubleprecision
64-bit format IEEE 754 value along with a special “Not-a-Number” (NaN) value, positive infinity, and
negative infinity.

* 数类型是表示一类可以直接表示为数值的值
* ES规范中，数值为双精度64位浮点数
* 数类型中有几个特殊值，遵循IEEE 754格式定义的值NaN(“Not-a-Number”)、正无穷、负无穷

#### Number object （数对象）

>A number object is a member of the type Object and is an instance of the Number object which is a constructor.
That is, a number object is created by using the Number constructor in a new expression, supplying a number as
an argument. The resulting object has an implicit (unnamed) property which is the number. A number object can
be coerced to a number value. A number object can be used anywhere a number value is expected. Note that a
number object can have shared properties by adding them to the Number prototype.

* Number对象Object类型的一个成员（值、实例）
* Number对象是一个构造函数
* new Number可以创建一个对象（后文用小S number对象表示）。
* Number构造函数接受String类型的参数
* number对象隐式的有一个属性（property），属性是一个Number值
* number对象可以和number值交叉使用。
* 这种情况是为了适应不同背景的程序员的，比如熟悉过程语言的会更习惯number是一个值，而熟悉OO语言的会更习惯number是一个对象

#### Infinity

>The primitive value Infinity represents the positive infinite number value.

* Infinity是一个原语值
* Infinity表示一个正无穷的数值

#### NaN

>The primitive value NaN represents the set of IEEE Standard “Not-a-Number” values.

* NaN是一个原语值
* 表示不是一个数值



## 3. 笔记整理

概述了ES语言的原理，如基于原型的继承、原型链，明确了ES的一些关键概念，包括类型、对象、值等。