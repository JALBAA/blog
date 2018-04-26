
# CSS编码规范

_本文内容基于[ecomfe(百度EFE团队)/css](https://github.com/ecomfe/spec/blob/master/css-style-guide.md)规范修改得来_


## 1 前言


本文档的目标是使CSS代码风格保持一致，容易被理解和被维护。

虽然本文档是针对CSS设计的，但是在使用各种CSS的预编译器(如less、sass、stylus等)时，适用的部分也应尽量遵循本文档的约定。


## 2 代码风格


### 2.1 文件


#### [建议] `CSS` 文件使用无 `BOM` 的 `UTF-8` 编码。

解释：

UTF-8 编码具有更广泛的适应性。BOM 在使用程序或工具处理文件时可能造成不必要的干扰。

### 2.2 缩进


#### [强制] 使用 `4` 个空格做为一个缩进层级，不允许使用 `2` 个空格 或 `tab` 字符。


示例：

```css
.selector {
    margin: 0;
    padding: 0;
}
```

### 2.3 空格


#### [强制] `选择器` 与 `{` 之间必须包含空格。

示例：

```css
.selector {
}
```

#### [强制] `属性名` 与之后的 `:` 之间不允许包含空格， `:` 与 `属性值` 之间必须包含空格。

示例：

```css
margin: 0;
```

#### [强制] `列表型属性值` 书写在单行时，`,` 后必须跟一个空格。

示例：

```css
font-family: Arial, sans-serif;
```

### 2.4 行长度


#### [强制] 每行不得超过 `120` 个字符，除非单行不可分割。

解释：

常见不可分割的场景为URL超长。


#### [建议] 对于超长的样式，在样式值的 `空格` 处或 `,` 后换行，建议按逻辑分组。

示例：

```css
/* 不同属性值按逻辑分组 */
background:
    transparent url(aVeryVeryVeryLongUrlIsPlacedHere)
    no-repeat 0 0;

/* 可重复多次的属性，每次重复一行 */
background-image:
    url(aVeryVeryVeryLongUrlIsPlacedHere)
    url(anotherVeryVeryVeryLongUrlIsPlacedHere);

/* 类似函数的属性值可以根据函数调用的缩进进行 */
background-image: -webkit-gradient(
    linear,
    left bottom,
    left top,
    color-stop(0.04, rgb(88,94,124)),
    color-stop(0.52, rgb(115,123,162))
);
```

### 2.5 选择器


#### [强制] 当一个 rule 包含多个 selector 时，每个选择器声明必须独占一行。

示例：

```css
/* good */
.post,
.page,
.comment {
    line-height: 1.5;
}

/* bad */
.post, .page, .comment {
    line-height: 1.5;
}
```

#### [强制] `>`、`+`、`~` 选择器的两边各保留一个空格。

示例：

```css
/* good */
main > nav {
    padding: 10px;
}

label + input {
    margin-left: 5px;
}

input:checked ~ button {
    background-color: #69C;
}

/* bad */
main>nav {
    padding: 10px;
}

label+input {
    margin-left: 5px;
}

input:checked~button {
    background-color: #69C;
}
```

#### [强制] 属性选择器中的值必须用双引号包围。

解释：

不允许使用单引号，不允许不使用引号。


示例：

```css
/* good */
article[character="juliet"] {
    voice-family: "Vivien Leigh", victoria, female
}

/* bad */
article[character='juliet'] {
    voice-family: "Vivien Leigh", victoria, female
}
```

### 2.6 属性


#### [强制] 属性定义必须另起一行。

示例：

```css
/* good */
.selector {
    margin: 0;
    padding: 0;
}

/* bad */
.selector { margin: 0; padding: 0; }
```

#### [强制] 属性定义后必须以分号结尾。

示例：

```css
/* good */
.selector {
    margin: 0;
}

/* bad */
.selector {
    margin: 0
}
```






## 3 通用




### 3.1 选择器


#### [强制] 如无必要，不得为 `id`、`class` 选择器添加类型选择器进行限定。

解释：

在性能和维护性上，都有一定的影响。


示例：


```css
/* good */
#error,
.danger-message {
    font-color: #c00;
}

/* bad */
dialog#error,
p.danger-message {
    font-color: #c00;
}
```

#### [建议] 选择器的嵌套层级应不大于 3 级，位置靠后的限定条件应尽可能精确。

示例：

```css
/* good */
#username input {}
.comment .avatar {}

/* bad */
.page .header .login #username input {}
.comment div * {}
```



### 3.2 属性缩写



#### [建议] 在可以使用缩写的情况下，尽量使用属性缩写。

示例：

```css
/* good */
.post {
    font: 12px/1.5 arial, sans-serif;
}

/* bad */
.post {
    font-family: arial, sans-serif;
    font-size: 12px;
    line-height: 1.5;
}
```

#### [建议] 使用 `border` / `margin` / `padding` 等缩写时，应注意隐含值对实际数值的影响，确实需要设置多个方向的值时才使用缩写。

解释：

border / margin / padding 等缩写会同时设置多个属性的值，容易覆盖不需要覆盖的设定。如某些方向需要继承其他声明的值，则应该分开设置。


示例：

```css
/* centering <article class="page"> horizontally and highlight featured ones */
article {
    margin: 5px;
    border: 1px solid #999;
}

/* good */
.page {
    margin-right: auto;
    margin-left: auto;
}

.featured {
    border-color: #69c;
}

/* bad */
.page {
    margin: 5px auto; /* introducing redundancy */
}

.featured {
    border: 1px solid #69c; /* introducing redundancy */
}
```


### 3.3 属性书写顺序


#### [建议] 同一 rule set 下的属性在书写时，应按功能进行分组，并以 **Formatting Model（布局方式、位置） > Box Model（尺寸） > Typographic（文本相关） > Visual（视觉效果）** 的顺序书写，以提高代码的可读性。

解释：

- Formatting Model 相关属性包括：`position` / `top` / `right` / `bottom` / `left` / `float` / `display` / `overflow` 等
- Box Model 相关属性包括：`border` / `margin` / `padding` / `width` / `height` 等
- Typographic 相关属性包括：`font` / `line-height` / `text-align` / `word-wrap` 等
- Visual 相关属性包括：`background` / `color` / `transition` / `list-style` 等

另外，如果包含 `content` 属性，应放在最前面。


示例：

```css
.sidebar {
    /* formatting model: positioning schemes / offsets / z-indexes / display / ...  */
    position: absolute;
    top: 50px;
    left: 0;
    overflow-x: hidden;

    /* box model: sizes / margins / paddings / borders / ...  */
    width: 200px;
    padding: 5px;
    border: 1px solid #ddd;

    /* typographic: font / aligns / text styles / ... */
    font-size: 14px;
    line-height: 20px;

    /* visual: colors / shadows / gradients / ... */
    background: #f5f5f5;
    color: #333;
    -webkit-transition: color 1s;
       -moz-transition: color 1s;
            transition: color 1s;
}
```


### 3.4 清除浮动



#### [建议] 当元素需要撑起高度以包含内部的浮动元素时，通过对伪类设置 `clear` 或触发 `BFC` 的方式进行 `clearfix`。尽量不使用增加空标签的方式。

解释：

触发 BFC 的方式很多，常见的有：

* float 非 none
* position 非 static
* overflow 非 visible

如希望使用更小副作用的清除浮动方法，参见 [A new micro clearfix hack](http://nicolasgallagher.com/micro-clearfix-hack/) 一文。

另需注意，对已经触发 BFC 的元素不需要再进行 clearfix。


### 3.5 !important


#### [建议] 尽量不使用 `!important` 声明。


#### [建议] 当需要强制指定样式且不允许任何场景覆盖时，通过标签内联和 `!important` 定义样式。

解释：

必须注意的是，仅在设计上 `确实不允许任何其它场景覆盖样式` 时，才使用内联的 `!important` 样式。通常在第三方环境的应用中使用这种方案。下面的 z-index 章节是其中一个特殊场景的典型样例。



### 3.6 z-index

#### `z-index` 将进行分层，对文档流外绝对定位元素的视觉层级关系进行管理。

解释：

同层的多个元素，如多个由用户输入触发的 Dialog，在该层级内使用相同的 `z-index` 或递增 `z-index`。

建议每层包含100个 `z-index` 来容纳足够的元素，如果每层元素较多，可以调整这个数值。

#### `z-index` 分层建议(待定)

示例：

1-9999: 应用层
1000-1999: 滚动侧边栏
7999-8999: block遮罩
9000-9999: msg_box
99999: 上限
。。。待定


## 4 值与单位


### 4.1 文本


#### [强制] 文本内容必须用双引号包围。

解释：

文本类型的内容可能在选择器、属性值等内容中。


示例：

```css
/* good */
html[lang|="zh"] q:before {
    font-family: "Microsoft YaHei", sans-serif;
    content: "“";
}

html[lang|="zh"] q:after {
    font-family: "Microsoft YaHei", sans-serif;
    content: "”";
}

/* bad */
html[lang|=zh] q:before {
    font-family: 'Microsoft YaHei', sans-serif;
    content: '“';
}

html[lang|=zh] q:after {
    font-family: "Microsoft YaHei", sans-serif;
    content: "”";
}
```

### 4.2 数值


#### [强制] 当数值为 0 - 1 之间的小数时，省略整数部分的 `0`。

示例：

```css
/* good */
panel {
    opacity: .8
}

/* bad */
panel {
    opacity: 0.8
}
```

### 4.3 url()


#### [强制] `url()` 函数中的路径不加引号。

示例：

```css
body {
    background: url(bg.png);
}
```


#### [建议] `url()` 函数中的绝对路径可省去协议名。


示例：

```css
body {
    background: url(//baidu.com/img/bg.png) no-repeat 0 0;
}
```


### 4.4 长度


#### [强制] 长度为 `0` 时须省略单位。 (也只有长度单位可省)

示例：

```css
/* good */
body {
    padding: 0 5px;
}

/* bad */
body {
    padding: 0px 5px;
}
```


### 4.5 颜色


#### [强制] RGB颜色值必须使用十六进制记号形式 `#rrggbb`。不允许使用 `rgb()`。

解释：

带有alpha的颜色信息可以使用 `rgba()`。使用 `rgba()` 时每个逗号后必须保留一个空格。


示例：

```css
/* good */
.success {
    box-shadow: 0 0 2px rgba(0, 128, 0, .3);
    border-color: #008000;
}

/* bad */
.success {
    box-shadow: 0 0 2px rgba(0,128,0,.3);
    border-color: rgb(0, 128, 0);
}
```

#### [强制] 颜色值可以缩写时，必须使用缩写形式。

示例：

```css
/* good */
.success {
    background-color: #aca;
}

/* bad */
.success {
    background-color: #aaccaa;
}
```

#### [强制] 颜色值不允许使用命名色值。

示例：

```css
/* good */
.success {
    color: #90ee90;
}

/* bad */
.success {
    color: lightgreen;
}
```

#### [建议] 颜色值中的英文字符采用小写。如不用小写也需要保证同一项目内保持大小写一致。


示例：

```css
/* good */
.success {
    background-color: #aca;
    color: #90ee90;
}

/* good */
.success {
    background-color: #ACA;
    color: #90EE90;
}

/* bad */
.success {
    background-color: #ACA;
    color: #90ee90;
}
```


### 4.6 2D 位置


#### [强制] 必须同时给出水平和垂直方向的位置。

解释：

2D 位置初始值为 `0% 0%`，但在只有一个方向的值时，另一个方向的值会被解析为 center。为避免理解上的困扰，应同时给出两个方向的值。[background-position属性值的定义](http://www.w3.org/TR/CSS21/colors.html#propdef-background-position)


示例：

```css
/* good */
body {
    background-position: center top; /* 50% 0% */
}

/* bad */
body {
    background-position: top; /* 50% 0% */
}
```

## 5 变换与动画



#### [强制] 使用 `transition` 时应指定 `transition-property`。

示例：

```css
/* good */
.box {
    transition: color 1s, border-color 1s;
}

/* bad */
.box {
    transition: all 1s;
}
```

## 6 layout.css
为了尽量达到布局语义和实体语义的分离，同时提供统一的便捷布局方式，在svn上的H5/gomeUI/core/css/加入新的顶级css文件layout.css。
layout.css 引入了栅格系统和弹性盒子，在进行布局时，应优先考虑使用公共layout。

## 7. 注释

### 7.1 文件注释

#### [强制] 每个css文件头部应该加入注释
示例：
`[]`中的内容是需要作者填写的内容
```css
/**
 * [file description]
 * @Auhtor: [xxx]
 * @Date: [2015-09-10 13:23:33]
 * @Last Modified By: [xxx]
 * @Last Modified Time: [2015-09-10 13:23:33]
 */
```
### 7.2块注释

####[强制] 根据每个功能模块（组件）书写css代码块。

示例：
```css
/*模块foo*/
/*@Date: 2015-09-10 13:23:33*/
.foo .title {}
.foo .desc {}

/*模块bar*/
/*@Date: 2015-09-10 19:30:00*/
.bar .title{}
.bar .desc{}
```

可选的更严谨的注释：

```css
/*start 模块foo*/
/*@Date: 2015-09-10 12:12:41*/
/*@Last Modified Time: 2015-09-12 12:33:12*/
.foo .title {}
.foo .desc {}
/*end*/
```

# css语义化建议
## css 命名规则
本规范的css依据 **布局类名[block] + 元素类名[element] + 实体语义(\_元素类名)[semantic(\_element)] + 修饰属性名[modifier]** 的规则

#### 名词解释

-  布局类名： 指layout.css中的css类，专门用于布局。
-  元素类名： 指一个页面中抽象出的一些可复用元素关键字，比如.list .item代表一个list的元素、module代表页面中的一个大的模块，content代表正文。
-  实体语义： 指一个模块或元素的带有语义的含义，加入`_元素类名`强调其和某一类元素的关联。
-  修饰属性名： 指一个模块或元素的状态，如激活、禁用、聚焦等，可以用空格隔开描述多个状态。

*_有时候元素类和实体语义的界限比较模糊，需要视具体情况而定_

*_此规则主要约束模块级别的css类，具体深入到模块细节中时，需要灵活使用，避免css类描述过于繁琐,如模块内的小布局，因为其缺少可复用性，可以直接以实体语义代替元素类名。_


####示例
```html
<style>

</style>
<!--一个页面的实体内容-->
<article class="wrapper">
    <!--产品模块-->
    <!--grid 为布局类-->
    <!--module代表抽象的可复用元素-->
    <!--product_module是这个模块的实体语义-->
    <section class="grid module product_module">
        <!--column_3 代表一个栅格中的一个区块，占3列-->
        <!--product1 为一个实体名-->
        <div class="column_3 product1">
            <!--title 为一个元素类关键字-->
            <h3 class="title">商品1的标题</h3>
            <!--desc 为一个元素类关键字-->
            <p class="desc">商品1的描述</p>
        </div>
        <div class="column_9 product2">
            <h3 class="title">商品2的标题</h3>
            <ul class="list">
                <li class="item focus unfold"></li>
                <li class="item"></li>
            </ul>
        </div>
    </section>
    <!--日期模块-->
    <section class="module date_module">
    </section>
    <!--购物车模块-->
    <!--flexbox 一个layout布局类，代表弹性盒子-->
    <!--module 代表这个section是一个大的功能模块，为了同别的模块区分，加入实体名buy_cart-->
    <section class="flexbox module buy_cart_module">
        <!--flex1 布局类，占3/1-->
        <!--btn 代表这个a标签是一个btn元素，为了区分同级的btn，加入实体名add-->
        <!--active 用来修饰它的状态，表示目前处于激活状态-->
        <a class="flex1 btn add_btn active">加入购物车</a>
        <button class="flex2 btn add_btn disable">删除购物车</button>
    </section>
</article>
```

## css 类定义方式

### 推荐的定义方式

推荐css根据html的模块划分
模块充当命名空间，定义模块下的样式时，需要先写出模块，避免对其他模块造成影响

例如需要描述上例中的product模块中的product2的list的item，则按如下方式描述
```css
/*good*/
.product_module .product2 .list .item {}
/*bad*/
.list .item {}
```

### 推荐的css选择器风格
推荐使用多类选择器风格
```css
/*作为命名空间的约束 元素类*/
.module .element {}
/*作为命名空间的约束 元素类且是xx语义*/
.module .element.semantic {}
/*作为命名空间的约束 元素类且是xx状态*/
.module .element.modifier {}
```

## css类元素常用关键字(待定)
**以下关键字，部分会整合到后续组件中**
module      一个大的功能模块
wrapper     一个包裹器
container   一个容器
list        一个列表
item        一个列表项
info        信息
btn         按钮
title       标题
sub_title   副标题
desc        描述
radio       单选框
checkbox    复选框
banner      广告/轮播
swiper      滑动条
font_icon   字体图标
portrait    头像
icon        图标
logo        标志
tag         标签
...待定'
