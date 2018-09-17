# ES1规范读书笔记(二)

记法约定

## 1. 句法及词法文法 （Syntactic and Lexical Grammars）

## 1.1 上下文无关文法(Context-Free Grammars)

原文

>A context-free grammar consists of a number of productions. Each production has an abstract symbol called a
nonterminal as its left-hand side, and a sequence of one or more nonterminal and terminal symbols as its righthand
side. For each grammar, the terminal symbols are drawn from a specified alphabet.
Starting from a sentence consisting of a single distinguished nonterminal, called the goal symbol, a given contextfree
grammar specifies a language, namely, the (perhaps infinite) set of possible sequences of terminal symbols
that can result from repeatedly replacing any nonterminal in the sequence with a right-hand side of a production
for which the nonterminal is the left-hand side.

要点：

* 上下文无关文法由若干生成式（production）组成
* 每个生成式（production）的左边都有一个抽象符号，叫做非终结符（nonterminal）
* 右边是由一个或多个非终结符和终结符（terminal symbol）组成的序列
* 对于上下文无关文法，终结符（terminal symbol）是从特定的字母表中选出（drawn）的
* 由可辨识的（distinguished）非终结符组成的句子，的开头，为目标符号（goal symbol）
* 从目标符号开始，一个给定的上下文无关文法描述了这种语言，即是
* 可以通过重复（or递归的？）的把生成式左侧的非终结符替换掉右侧的序列的 一组（或无限的）带有终结符的序列

扩展：

* 上下文无关文法，属于形式文法中的一种
* 形式文法是描述形式语言的一种方法
* 形式语言是用精确的数学或机器可处理的公式定义的语言
* 常见的形式文法由四种：无限制文法、上下文相关文法、上下文无关文法和正规（正则）文法
* 四种文法分别对应四种形式语言：递归可枚举语言、上下文相关语言、上下文无关语言和正规（正则）语言
* [维基百科上对生成式（production）的解释](https://en.wikipedia.org/wiki/Production_(computer_science))

## 1.2  词法文法（The lexical grammar）

A lexical grammar for ECMAScript is given in Section 7. This grammar has as its terminal symbols the
characters of the Unicode character set. It defines a set of productions, starting from the goal symbol Input, that
describe how sequences of Unicode characters are translated into a sequence of input elements.
Input elements other than white space and comments form the terminal symbols for the syntactic grammar for
ECMAScript and are called ECMAScript tokens. These tokens are the reserved words, identifiers, literals, and
punctuators of the ECMAScript language. Moreover, line terminators, although not considered to be tokens, also
become part of the stream of input elements and guide the process of automatic semicolon insertion (see section
7.8). Simple white space and single-line comments are simply discarded and do not appear in the stream of input
elements for the syntactic grammar. A multi-line comment is likewise simply discarded if it contains no line

terminator; but if a multi-line comment contains one or more line terminators, then it is replaced by a single line
terminator, which becomes part of the stream of input elements for the syntactic grammar.
Productions of the lexical grammar are distinguished by having two colons “::” as separating punctuation.

## 1.3  数值字符串文法 (The numeric string grammar)

A second grammar is used for translating strings into numeric values. This grammar is similar to the part of the
lexical grammar having to do with numeric literals and has as its terminal symbols the characters of the Unicode
character set. This grammar appears in section 9.3.1.
Productions of the numeric string grammar are distinguished by having three colons “:::” as punctuation