# ES1规范读书笔记 (一)

## 1. 语言概览

原文：

    Language Overview

    The following is an informal overview of ECMAScript—not all parts of the language are described. This overview
    is not part of the standard proper.
    ECMAScript is object-based: basic language and host facilities are provided by objects, and an ECMAScript
    program is a cluster of communicating objects. An ECMAScript object is an unordered collection of properties each
    with 0 or more attributes which determine how each property can be used—for example, when the ReadOnly
    attribute for a property is set to true, any attempt by executed ECMAScript code to change the value of the property
    has no effect. Properties are containers that hold other objects, primitive values, or methods. A primitive value is a
    member of one of the following built-in types: Undefined, Null, Boolean, Number, and String; an object is a
    member of the remaining built-in type Object; and a method is a function associated with an object via a property.
    ECMAScript defines a collection of built-in objects which round out the definition of ECMAScript entities. These
    built-in objects include the Global object, the Object object, the Function object, the Array object, the String
    object, the Boolean object, the Number object, the Math object, and the Date object.
    ECMAScript also defines a set of built-in operators which may not be, strictly speaking, functions or methods.
    ECMAScript operators include various unary operations, multiplicative operators, additive operators, bitwise shift
    operators, relational operators, equality operators, binary bitwise operators, binary logical operators, assignment
    operators, and the comma operator.
    ECMAScript syntax intentionally resembles Java syntax. ECMAScript syntax is relaxed to enable it to serve as an
    easy-to-use scripting language. For example, a variable is not required to have its type declared nor are types
    associated with properties, and defined functions are not required to have their declarations appear textually before
    calls to them

要点：

* ES基于对象（object-based），或者说一切皆对象
* ES对象(object)是一堆特性（property）的无序集合
* 特性（property）具有属性（attribute），属性决定特性（property）如何使用。
* 特性（property）是对象（object）、原语值（primitive value）、方法（method）的容器。
* 一个原语值（primitive value）是5种内置类型（built-in type）之一（除了Object）
* 一个对象（object）是内置类型（Object）的成员
* 内置类型（built-in type）为 Undefined, Null, Boolean, Number, String, Object
* 方法（method）是一个关联到一个对象（object）的特性（property）上的函数（function）
* 内置对象（built-in objects）包括the `Global(window)` object, the `Object` object, the `Function` object, the `Array` object, the `String` object, the `Boolean` object, the `Number` object, the `Math` object, and the `Date` object.
* 内置操作符（built-in operators）严格来说不是方法或函数，用来进行各种操作
* 内置操作符（built-in operators）的操作有一元、乘法、加法、位、关联、相等性、逻辑与、逻辑异或、赋值（assignment）、逗号操作
* ES语法有意的模仿java语法，但语法松散，以便不懂编程的人也能使用
