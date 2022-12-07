# LiA
### A transpiled superset of TeX for writing LaTeX.

![status](https://img.shields.io/github/workflow/status/jaspwr/LiA/Rust)

This is more for my personal use however you're welcome to use it or contribute. These added features are just designed to make LaTeX code less verbose, faster to write but also just catered more towards my personal preference.

> For a more detailed explanation of the features see the [documentation](docs.md).

> For installation instructions see the [installation](#installation) section.

## Example #1
### LiA code
```tex
eq {
    dy/dx = x*(a - b) + [[1, 2], [3, 4]]
}
```
### Resulting TeX
```tex
\begin{equation}
    \frac{dy}{dx} = x \times \left(a - b\right) + \begin{pmatrix} 1 & 2 \\ 3 & 4 \end{pmatrix}
\end{equation}
```

## Example #2
### LiA code
```tex
use graphicx, [utf8]inputenc
@cat = 🐈
@img = (src, desc) => {
    env center {
        \image{@("images/" + src), 10cm}
        \linebreak
        **@desc**
    }
}
#* Cool Cat Image
## A subsection
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
  \subsection{A subsection}
  \img{images/cat.png}{Meow \cat!}
\end{document}
```
## Example #3
### LiA code
```tex
@muliplication = (a: Number, b: Number) => { $@a \times @b = @(a * b)$ }
* I'm a **Markdown** style ***list***
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
    \item I'm a \textit{Markdown} style \textbf{list}
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
# Installation

|__OS__|__Instructions__|
|---|---|
|__Windows__| Download the latest release from the [releases page](https://github.com/jaspwr/LiA/releases). There is currently no installer so you will need to add the directory to your path manually. If you have Rust intalled it is recommended that you use the instructions in the [building](#building) section.
|__Arch Linux__| Clone this repository and use the PKGBUILD. I'm working on adding this to the AUR.
|__Other__| Follow the instructions in the [building](#building) section.
# Usage
```bash
lia file.lia
```
* Run `lia --help` for more information on usage.
# Building
Requires [Rust](https://www.rust-lang.org/tools/install) to be installed.
```bash
git clone https://github.com/jaspwr/LiA
cd LiA
cargo install --path
```
