## Introduction to Markdown
Markdown is a lightweight markup language that allows you to format text easily using plain text syntax. It’s widely used in documentation, blogging, and even writing code comments. This guide will introduce the basic features of Markdown and how you can use them in your documents.

### Basic Syntax
- Headings
- Lists
- Blockquote
- Tables, etc

### Details
When we talk about the headings, these start from H1 to H6 annotated as; 
- # for H1
- ## for H2
- ### for H3 upto ###### for H6
____

> A blockquote is anything similar to this here as you can see below the horizontal line
It can be continued as multi paragraph by shitfting to the next line

Skip a line to break from a blockquote
Let's take a sample codeblock below;
```
let name = "Aine";
```
Empty one;
```
```

### Testing numbered lists
1. Item 1
2. Item 2
3. Item 3

4. Item 4
5. Item 5

a. Alph 1
B. Alpha 2

i) Roman 1
ii) Roman 2
iii) Roman 3

-[x] Item Checked 1
-[X] Item Checked 2
-[ ] Item Unchecked 3

-[ ] Item Unchecked 4

_Hello there *Mr. Aine.*_ Thanks for the day_
``Hey``

# Welcome to the Markdown Test

## Introduction
This document is designed to stress test your Markdown parser. It includes various elements such as:
- Headers
- Paragraphs
- Lists (Ordered & Unordered)
- Blockquotes
- Code Blocks
- Inline Formatting
- Nested Elements

___

## Headers Test
# Header 1
## Header 2
### Header 3
#### Header 4
##### Header 5
###### Header 6

## Paragraphs & Line Breaks
This is a paragraph with some text.
This is another paragraph.
This paragraph contains _italic,_ *bold,* __underline,__ and nested *bold-*_italic_ formatting.

___

## Lists Test
### Unordered List
-(disc) Item 1
    -(circle) Subitem 1
        -(disc) Subsubitem 1
-(disc) Item 2
    -(circle) Subitem 2

### Ordered List
1. First item
    1. Nested first item
    2. Nested second item
2. Second item
    -(disc) Mixed bullet inside ordered list

___

## Blockquote Test
> This is a single-level blockquote.
> This is a nested blockquote.
Blockquote with a list
Another list item
___

## Code Block Test
### Inline Code
Use ``print!("Hello, world!");`` in Rust.

## Multi-line Code Block
```
rust
fn main() {
    println!("Hello, Markdown!");
}
```
## hy
___

## Horizontal Rule
___

## Nested Elements
- *Bold List Item*
    - _Italic Nested Item_
        - ``Code inside list``

> *Bold in Blockquote*
Item inside blockquote
_Italic inside blockquote_
___

## Highlight Test
This is ==highlighted text==.
___

## Large Content Stress Test
Below is a repeated paragraph to test performance.
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.
Sed cursus ante dapibus diam. _Sed nisi_. Nulla quis sem at nibh elementum imperdiet. Duis
sagittis ipsum. Praesent mauris. *Fusce nec tellus __sed augue semper porta__*. ~Mauris massa~.
Vestibulum lacinia arcu eget nulla. Class aptent taciti sociosqu ad litora torquent per
conubia nostra, per inceptos himenaeos. Curabitur sodales ligula in libero.
Repeat ~the above~, paragraph 5000 times for extreme testing.