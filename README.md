<p align="center">
    <img src="misc/lia.png" width="120px" alt="LiA logo">
</p>

### A transpiled superset of $\text{TeX}$ for writing $\text{LaTeX}$.

<a href="https://github.com/jaspwr/LiA/actions/workflows/rust.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/jaspwr/LiA/rust.yml?branch=main" alt="build">
</a>
<a href="https://aur.archlinux.org/packages/lia-git">
    <img src="https://img.shields.io/aur/version/lia-git" alt="build">
</a>

This project is in development (expect bugs). This is a superset of TeX that extends its syntax adding new several new features. These added features are just designed to make LaTeX code less verbose and faster to write but also just catered more towards my personal preference.

> For a more detailed explanation of the features see the [documentation](docs.md).

> For installation instructions see the [installation](#installation) section.

## Example #1
### LiA code
[COMPILATION_INPUT_START]: <> (Do not remove this line.)
```tex
eq {
    dy/dx = x*(a - b) + [[1, 2], [3, 4]] + sin(x)
}
```
[COMPILATION_INPUT_END]: <> (Do not remove this line.)
### Resulting TeX
[COMPILATION_RESULT_START]: <> (Do not remove this line.)
```tex
\begin{document}
    \begin{equation}
        \frac{dy}{dx} = x \times \left(a - b\right) + \begin{pmatrix} 1 & 2 \\ 3 & 4 \end{pmatrix} +\sin \left(x\right)
    \end{equation}
\end{document}
```
[COMPILATION_RESULT_END]: <> (Do not remove this line.)

## Example #2
### LiA code
[COMPILATION_INPUT_START]: <> (Do not remove this line.)
```tex
use graphicx, [utf8]inputenc
@cat = 🐈
@img = (src, desc) => {
    env center {
        \image{@("images/" + src)}{10cm}
        \linebreak
        **@desc**
    }
}
#* Cool Cat Image
## A subsection
@img(cat.png, Meow @cat!)
```
[COMPILATION_INPUT_END]: <> (Do not remove this line.)
### Resulting TeX
[COMPILATION_RESULT_START]: <> (Do not remove this line.)
```tex
\usepackage{graphicx}
\usepackage[utf8]{inputenc}


\newcommand{\cat}{🐈}
\newcommand{\img}[3]{
    \begin{center}
        \image{#3}{10cm}
        \linebreak
        \textit{#2}
    \end{center}
}


\begin{document}
    \section*{Cool Cat Image}
    \subsection{A subsection}
    \img{cat.png}{Meow \cat!}{images/cat.png}
\end{document}
```
[COMPILATION_RESULT_END]: <> (Do not remove this line.)
## Example #3
### LiA code
[COMPILATION_INPUT_START]: <> (Do not remove this line.)
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
[COMPILATION_INPUT_END]: <> (Do not remove this line.)
### Resulting TeX
[COMPILATION_RESULT_START]: <> (Do not remove this line.)
```tex
\newcommand{\muliplication}[3]{
    $#1 \times #2 = #3$
}


\begin{document}
    \begin{itemize}
        \item I'm a \textit{Markdown} style \textbf{list}
        \item \muliplication{2}{3}{6}
        \item \muliplication{6}{6}{36}
        \item \muliplication{2}{9}{18}
        \begin{itemize}
            \item I'm indented
        \end{itemize}
        \item {I'm a multiline
        list item}
    \end{itemize}
\end{document}
```
[COMPILATION_RESULT_END]: <> (Do not remove this line.)
# Installation

|__OS__|__Instructions__|
|---|---|
|__Arch Linux__| Install from the AUR [https://aur.archlinux.org/packages/lia-git](https://aur.archlinux.org/packages/lia-git). |
|__Other__| Download the latest release from the [releases page](https://github.com/jaspwr/LiA/releases). There is currently no installer so you will need to add the directory to your path manually. |
# Usage
```bash
lia file.lia -w --pdflatex
```
* Run `lia --help` for more information on usage.
* The `-w` flag will tell the compiler to watch the file for changes and recompile it.
* The `--pdflatex` flag runs `pdflatex` on the outputted TeX file.
# Building
Requires [Rust](https://www.rust-lang.org/tools/install) to be installed.
```bash
git clone https://github.com/jaspwr/LiA
cd LiA
cargo build --release
```
Your binary will be in `target/release`.

# VSCode extension
For syntax highlighting and other features in VSCode, you can use the extension. To install it, copy `tooling/vscode/lia-helper` to your VSCode extensions directory (normally `~\.vscode\extensions`) then restart VSCode.


# Contributing
I am planning to do a full rewrite. Please do not contribute right now.
