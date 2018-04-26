# Promise讲解

原理实现，参见[http://code.ds.gome.com.cn/gitlab/wireless-h5/gome-polyfill-promise](http://code.ds.gome.com.cn/gitlab/wireless-h5/gome-polyfill-promise)

## 1. Promise 简介

`Promise`回调函数之外的另一种异步操作代码风格, 使用`Promise`可以进行链式调用，防止多次回调造成的n次缩进问题（也称callback hell）

## 2. Promise 定义

`Promise`是一个函数

`new Promise`得到一个函数实例

下文中使用大写P，`Promise`指代**Promise函数**，小写p，`promise`指代`new`出来的**promise实例**

`promise`是一个有一个`then`方法的对象


### 2.1 promise状态、值、原因

#### 2.1.1 一个promise有三个状态


* 初始态：`pending`

* 解决态：`fulfilled` 

* 拒绝态：`rejected`

*例：2.1-1*

```js
const promise = new Promise(function (resolve, reject) {
    
})

// proimse初始状态为pending

```

#### 2.1.2 promise状态变更

* `promise`的状态能且仅能变更一次
* `promise`的状态只能从`pending`转变为
* `fulfilled`，或从`pending`转变为`rejected`
* 解决态的`promise`会具备一个值
* 拒绝态的`promise`会具备一个原因
* `promise`的状态变更是不可逆的

*图 2.1-1*

![promise状态和值](../img/tutorials/promise状态.png)


*例子 2.1-2*

```js
const promise = new Promise(function (resolve, reject) {
    $.get('/any-url', function (data) {
        // 改变实例的状态和值
        resolve(data)
    })
})
// proimse初始状态为pending
// promise初始化的同时发起了一个ajax请求，请求成功之后
// promise的状态变为fufilled，且promise获得值，值为请求结果
```

*例子 2.1-3*

```js
const promise = new Promise(function (resolve, reject) {
    setTimeout(function () {
        reject('fail')
    }, 1000)
})
// proimse初始状态为pending
// promise初始化1s之后，拒绝promise
// promise的状态变为rejected，且promise获得原因，原因为一个字符串'fail'
```


*例子 2.1-4*

```js
const promise = new Promise(function (resolve, reject) {
    rejected(new Error('fail'))
})
// proimse初始化即被拒绝
// 可以理解为promise初始态为promise，原因是一个Error实例，error的message为字符串'fail'
```

### 2.2 Promise函数解析

* `Promise`函数必须接收一个参数`executor`，以下以`executor`指代这个参数
* `executor`必须是一个函数
* `executor`接收两个参数
* `executor`的两个参数都是函数，以下以`resolve`指代第一个参数，以`reject`指代第二个参数
* 执行`resovle`，则将此`promise`实例置为`fufilled`
* `resolve`接收一个参数，可以是任意类型，此参数会作为`promise`实例的值
* 执行`reject`，则将测`promise`实例置为`rejected`
* `reject`接收一个参数，可以是任意类型，此参数会作为`promise`实例的原因


*例子 2.2-1 此处的函数定义只是为了方便理解*
```js
/**
 * @param x {any}
 */
function resolve (x) {

}
/**
 * @param r {any}
 */
function reject (r) {

}

/**
 * @param resolve {Function}
 * @param reject {Function}
 */
function executor (resolve, reject) {
}

const promise = function Promise (excutor) {

}

```


### 2.3 promise.then

#### 2.3.1 

* `then`方法是个函数
* `then`方法返回一个新的`promise`对象`promise2`
* `then`从一个`promise`取值（或原因），也可以理解为`then`是一个取值器
* `then`方法有两个参数
* 使用`onFulfilled`指代第一个参数
* 使用`onRejected`指代第一个参数
* `onFufilled`是当`promise`的状态为`fulfilled`时的回调
* `onRejected`是当`promise`的状态为`rejected`时的回调
* `onFufilled`有一个参数，是promise的值
* `onRejected`有一个参数，是promise的原因

*例子 2.3-1 then回调的执行顺序*
```js
const promise = new Promise(function executor (resolve, reject) {
    setTimeout(function () {
        resolve('foo')
    }, 1000)
})
// promise pending中
const promise2 = promise.then(function onFulfilled (x) {
    // 1s后此函数被触发
    console.log(x) // 'foo'
}, function onRejected (r) {

})
```


#### 2.3.2 promise.then的return值


* `onFulfilled`或`onRejected`可以`return`值
* `return`的值如果是一个`Promise`实例`promise3`，则`then`返回的`promise2`实例继承`promise3`的状态和值（或原因）
* `return`的值如果是一个非`Promise`实例，则`promise2`触发`fufilled`状态，并且获得`return`的结果作为值
* 无论是`onFulfilled`还是`onRejected`，只要`return`值不是一个`rejeced promise`，`promise2`都会被置为`fufilled`

*例子 2.3-2 then的回调返回一个promise对象*
```js
const promise = new Promise(function executor (resolve, reject) {
    setTimeout(function () {
        resolve('foo')
    }, 1000)
})
// promise pending中
const promise2 = promise.then(function onFulfilled (x) {
    console.log(x) // foo
    const promise3 = new Promise(function (resolve, reject) {
        setTimeout(function () {
            resolve('123')
        }, 1000)
    })
    return promise3
}, function onRejected (r) {

})

promise2.then(function (x) {
    // 2s之后触发
    console.log(x)// 123
})
```

*例子 2.3-3 then的回调返回一个非promise对象，无论如何都会将proimse2置为fulfilled*
```js
const promise = new Promise(function executor (resolve, reject) {
    setTimeout(function () {
        reject('foo')
    }, 1000)
})
// promise pending中
const promise2 = promise.then(function onFulfilled (x) {
    
}, function onRejected (r) {
    console.log(r) // foo
    return r + 'bar'
})
promise2.then(function (x) {
    // 1s之后触发
    console.log(x)// 123
})
```

#### 2.3.3 更多的`then`特性


* 基于`then`可以实现链式调用
* 一个`promise`可以被then多次

*例子 2.3-4 链式调用， 同步*

```js

function add(x) {
    return x + 1
}

new Promise(function (resolve, reject) {
    resolve(1)
})
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(function (x) {
    console.log(x) // 12
})

```


*例子 2.3-5 链式调用， 异步*

```js

function add(x) {
    return new Promise(function (resolve, reject) {
        setTimeout(function () {
            resolve(x + 1)
        }, 100)
    })
}

new Promise(function (resolve, reject) {
    resolve(1)
})
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(function (x) {
    // after 1.2s
    console.log(x) // 12
})

```


*例子 2.3-6 链式调用， 结合gome-utils-http做例子， 本例只做示例*

```js
const http = require('gome-utils-http')

http(/*接口A,参数略*/)
.then(x => {
    return http(/*接口B,参数略*/)
})
.then(x => {
    return http(/*接口C,参数略*/)
})
.then(x => {
    return http(/*接口D,参数略*/)
})
.then(x => {
    // 执行业务逻辑
})
```

*例子 2.3-6 一个promise可以被then多次*

```js
const promise = new Promise(function (resolve, reject) {
    resolve(1)
})

const promise2 = promise.then(function (x) {
    return x + 1
})

const promise3 = promise.then(function (x) {
    return x - 1
})

promise2.then(function (x) {
    console.log(x) // 2
})

promise3.then(function (x) {
    console.log(x) // 0
})
```

### 2.4 promise.catch

`catch`即相当于`then`方法第一个参数为null的情况

```js
catch(function () {

})

// 等价于

then(/* 永远不会被调用 */, function () {

})
```

### 2.5 promise错误捕获的穿透性


promise链式调用中，只要有一个错误被捕获（通过`catch`或`then`的`onRejected`回调）即可，所以经常可以做兜底的错误捕获，使错误从调用链条中穿过


*例子 2.3-4 第一个reject会直接穿透到最后一个catch*

```js

function add(x) {
    // 不会被执行到
    return x + 1
}

new Promise(function (resolve, reject) {
    reject('pass')
})
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.then(add)
.catch(function (x) {
    console.log(x) // pass
})

```
## 3. Promise.all

Promise.all(Array)<sup>[1]</sup> 方法返回一个 Promise 实例 

此实例在 array 参数内所有的 promise 都“完成（resolved）”或参数中不包含 promise 时回调完成（resolve）；

如果参数中  promise 有一个失败（rejected），此实例回调失败（rejecte），失败原因的是第一个失败 promise 的结果。


## 4. Promise.race

Promise.race(iterable) 方法返回一个 promise 

并伴随着 promise对象解决的返回值或拒绝的错误原因, 只要 iterable 中有一个 promise 对象"解决(resolve)"或"拒绝(reject)"。



*[1] Promise.all的参数实际上是(iterable)，此处为了方便理解，替换为Array*