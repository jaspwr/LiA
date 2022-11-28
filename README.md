# LiA
##### A transpiled superset of TeX because some of TeX's syntax was annoying me.
This is more for my personal use however you're welcome to use it or contribute. These added features are just designed to make LaTeX code less verbose, faster to write but also just catered more towards my personal preference.

## Example #1
### LiA code
```tex
use graphicx, [utf8]inputenc
@cat = 🐈
@img = (src, desc) => {
    env center {
        \image{@("images/" + src), 10cm}
        \linebreak
        \textit{@desc}
    }
}
#* Cool Cat Image
@img(cat.png, Meow @cat!)
```
### Resulting TeX
```tex
\usepackage{graphicx}
\usepackage[utf8]{inputenc}


\newcommand{\cat}{🐈}
\newcommand{\img}[2]{
  \begin{center}
    \image{#1, 10cm}
    \linebreak
    \textit{#2}
  \end{center}
}


\begin{document}
  \section*{Cool Cat Image}
  \img{images/cat.png}{Meow \cat!}
\end{document}
```
## Example #2
### LiA code
```tex
@muliplication = (a: Number, b: Number) => { $@a \times @b = @(a * b)$ }
* I'm a Markdown style list
* @muliplication(2, 3)
* @muliplication(6, 6)
* @muliplication(2, 9)
  * I'm indented
* {I'm a multiline
   list item}
```

### Resulting TeX
```tex
\newcommand{\img}[3]{
  $#1 \times #2 = #3$
}


\begin{document}
  \being{itemize}
    \item I'm a Markdown style list
    \item \muliplication{2, 3, 6}
    \item \muliplication{6, 6, 36}
    \item \muliplication{2, 9, 18}
    \being{itemize}
      \item I'm indented
    \end{itemize}
    \item I'm a multiline
          list item
  \end{itemize}
\end{document}
```
### types:
* `Number` or `num`
* `Text` or `txt`
* `Size` or `sz`
* `Colour` or `Color` or `col`
* `Lambda` or `fn` or `λ`
* `Any` or simply no type annotation
