# markdownit cheatsheet
## Block level elements
1. Headings 👍
   a. # h1      d. #### h4
   b. ## h2     e. ##### h5
   c. ### h3    f. ###### h6
2. Horizontal rule 👍
   a. simple    ___
   b. bold      ____
3. Paragraph 👍
   \n\n

4. Blockquote 👍
   > quote

5. Codeblock
   ```
   code here
   ``` 
6. Alignment 👍
   a. |< Align left       c. |> Align right
   b. |= Align center     d. |- Justify

7. Forms
   ::form_tag space separated attributes
   ::form method="post"
   ::password name="input_name" label="input label" place_holder="input placeholder"
   ::submit value="Login"
   ::endform

8. Tables
   ::table headers="Name,Age,Country"
   ::row Aine Dixon,25,Uganda
   ::row Elon Musk,52,Mars
   ::endtable


9. Lists
   1. Number list       -[ ] Uncheked list      1tab/4space indent Nested list
   i) Roman list        -[x] Checked list       - List item
   a. alphabetic        -(shape) Bulleted list

## Inline elements
1. Text formating
   a. *Bold*               d. ``Inline code``      g.  :emoji:
   b. _Italic_             e. ~Strike through~     h.  ==(color)Highligting==
   c.  __Underline__       f. **Caption**          i.  =(size, color, weight, family)font styling=
2. Links
   a. [External link](url) 
   b. [On page refer][id-ref]
   d. [alt text(src)](h, w, border-style, border-color)
   e. [
         [alt text(src)](h, w, border-style, border-color),
         [alt text(src)](h, w, border-style, border-color)
      ] Multi column image definition

3. Letter casing
   a. [-Uppercase]      d. (-Superscript)
   b. [~Capitalized]    e. (_subscript)
   c. [_Lowercase]

4. Escape syntax
   \[ Escape casing start
   \_ Escape italic
   \* Escape bold
 