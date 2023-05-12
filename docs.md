# LiA 0.3.0 Documentation

>⚠️ This is the documentation for version 0.3.0. This is only an early version and is still in development. Do not expect any of these features to stay the same in future versions. Additionally, the compiler has not been thoroughly tested and may not work as expected. If you find any bugs or have any suggestions please open an issue or pull request on the [GitHub repository](https://github.com/jaspwr/LiA).


## Contents
1. [Language features](#language-features)
    1. [Environments](#environments)
    1. [Imports](#imports)
    1. [Italic](#italic)
    1. [Bold](#bold)
    1. [Markdown style sections](#markdown-style-sections)
    1. [Markdown style lists](#markdown-style-lists)
    1. [Markdown style enumerated lists](#markdown-style-enumerated-lists)
    1. [Variables](#variables)
        1. [Referencing variables](#referencing-variables)
        1. [Declaring variables](#declaring-variables)
    1. [Equations](#equations)
        1. [Numbered](#numbered)
        1. [Anonymous](#anonymous)
        1. [General expressions](#general-expressions)
        1. [Expression with grouping](#expression-with-grouping)
        1. [Matrices](#matrices)
        1. [Macros](#macros)
    1. [Explicit version specification](#explicit-version-specification)
1. [Document Structure](#document-structure)


## Language features

Most TeX is valid in LiA so you are able to write LaTeX as normal however with the addition of the features listed below.

### Environments
Environment statements provide a less verbose way to open environments.
#### Lia
[COMPILATION_INPUT_START]: <> (Do not remove this line.)
```tex
env environmenttype {
    content
}
```
[COMPILATION_INPUT_END]: <> (Do not remove this line.)
#### TeX
[COMPILATION_RESULT_START]: <> (Do not remove this line.)
```tex
\begin{environmenttype}
    content
\end{environmenttype}
```
[COMPILATION_RESULT_END]: <> (Do not remove this line.)

-------------------

### Imports
Any line opened with a `use` keyword (excluding whitespace) will be treated as an import statement.

| LiA                     | TeX                       |
|-------------------------|---------------------------|
| `use packagename`       | `\usepackage{packagename}`|
|`use packagename, otherpackage, thirdpackage`| `\usepackage{packagename}`<br>`\usepackage{otherpackage}`<br>`\usepackage{thirdpackage}`|
|`use [option]packagename`| `\usepackage[option]{packagename}`|

Consumes remainder of line.

-------------------

### Italic
This is an alternative syntax for the `textit` command.
| LiA                    | TeX                      |
|------------------------|--------------------------|
| `**Inner text**`       | `\textit{Inner text}`    |
> Note this differs from the single `*` in markdown.
-------------------

### Bold
This is an alternative syntax for the `textbf` command.

| LiA                     | TeX                      |
|-------------------------|--------------------------|
| `***Inner text***`      | `\textbf{Inner text}`    |
> Note this differs from the `**` in markdown.

-------------------

### Markdown style sections

| LiA          | TeX                      |
|--------------|--------------------------|
| `# title`    | `\section{title}`        |
| `## title`   | `\subsection{title}`     |
| `### title`  | `\subsubsection{title}`  |
| `#* title`   | `\section*{title}`       |
| `##* title`  | `\subsection*{title}`    |
| `###* title` | `\subsubsection*{title}` |


Consumes remainder of line. For multiline enclose the section title in `{}`.

-------------------

### Markdown style lists
Any line opened with a `*` will be treated as a list item. You can create nested lists with indentation. In most cases the indentation type will be inferred.
> ⚠️ As of version 0.3.0, indentation types other than 4 spaces are unthoroughly tested and may not work as expected.
#### Lia
[COMPILATION_INPUT_START]: <> (Do not remove this line.)
```tex
* List item.
* List item.
    * Nested item.
        * Double nested item.
* List item.
```
[COMPILATION_INPUT_END]: <> (Do not remove this line.)
#### TeX
[COMPILATION_RESULT_START]: <> (Do not remove this line.)
```tex
\begin{itemize}
    \item List item.
    \item List item.
    \begin{itemize}
        \item Nested item.
        \begin{itemize}
            \item Double nested item.
        \end{itemize}
    \end{itemize}
    \item List item.
\end{itemize}
```
[COMPILATION_RESULT_END]: <> (Do not remove this line.)
 List items consume the remainder of the line. For multiline enclose list item contents in `{}`.

-------------------

### Markdown style enumerated lists
Any line opened with `1.` will be treated as an enumerated list item, this can be any number. You can create nested lists with indentation. In most cases the indentation type will be inferred.
> ⚠️ As of version 0.3.0, indentation types other than 4 spaces are unthoroughly tested and may not work as expected.
#### Lia
[COMPILATION_INPUT_START]: <> (Do not remove this line.)
```tex
1. List item.
1. List item.
    1. Nested item.
        1. Double nested item.
1. List item.
```
[COMPILATION_INPUT_END]: <> (Do not remove this line.)
#### TeX
[COMPILATION_RESULT_START]: <> (Do not remove this line.)
```tex
\begin{enumerate}
    \item List item.
    \item List item.
    \begin{enumerate}
        \item Nested item.
        \begin{enumerate}
            \item Double nested item.
        \end{enumerate}
    \end{enumerate}
    \item List item.
\end{enumerate}
```
[COMPILATION_RESULT_END]: <> (Do not remove this line.)
List items consume the remainder of the line. For multiline enclose list item contents in `{}`.

-------------------

### Variables
LiA variables provide a less verbose way to define new commands.

Any word annotated with a `@` will be treated as a variable.
#### Referencing variables
> ⚠️ As of version 0.3.0, variables with computed arguments can not be used before they are defined. This will be fixed in future versions.

| LiA                      | TeX                      |
|--------------------------|--------------------------|
| `@varname`               | `\varname`               |
| `@varname(arg, otherarg)`| `\varname{arg}{otherarg}` |
#### Declaring variables
##### Constants
| LiA                      | TeX                      |
|--------------------------|--------------------------|
| `@varname = Some content`| `\newcommand{\varname}{Some content}`|
* Consumes remainder of line. For multiline enclose the contents in `{}`.
##### Simple functions
| LiA                      | TeX                      |
|--------------------------|--------------------------|
| `@varname = () => {Some content}`| `\newcommand{\varname}[0]{Some content}`|
|`@varname = (arg, otherarg) => {Hello @arg @otherarg}`|`\newcommand{\varname}[2]{Hello #1 #2}`|

Will consume until unnested `}`.
##### Computed functions
If inside the contents of a function an expression in `@()` is found it will be evaluated and whenever it is referenced the result will be computed and passed in as an additional argument.
```tex
@varname = (a, b) => {
    @(a + b)
}
```
When referenced as `@varname(1,2)` the result will be `\varname{1}{2}{3}`.
* Types currently supported are `Number` and `String`. It is possible to annotate arguments with types which will be checked at compile time.
```tex
@varname = (arg: Number, otherarg: String) => {
    @(arg + 1)
    @(otherarg + "!")
}
```
* Supported operators are for `Number` are `+`, `-`, `*`, `/`, `%` and `^`.
* Supported operators for `String` are `+`.

Will consume until unnested `}`.


-------------------

### Equations
Equation statements provide an alternative and commonly less verbose way to write LaTeX equations.
#### Numbered
Equations formatted like this with `eq` will be transpiled to a regular equation in an `equation` environment.
##### Lia
[COMPILATION_INPUT_START]: <> (Do not remove this line.)
```tex
eq {
    a * b
}
```
[COMPILATION_INPUT_END]: <> (Do not remove this line.)
##### TeX
[COMPILATION_RESULT_START]: <> (Do not remove this line.)
```tex
\begin{equation}
    a \times b
\end{equation}
```
[COMPILATION_RESULT_END]: <> (Do not remove this line.)
#### Anonymous
Equations formatted like this with `eq*` will be transpiled to an anonymous equation in `\[`...`\]`.
##### Lia
[COMPILATION_INPUT_START]: <> (Do not remove this line.)
```tex
eq* {
    a * b
}
```
[COMPILATION_INPUT_END]: <> (Do not remove this line.)
##### TeX
[COMPILATION_RESULT_START]: <> (Do not remove this line.)
```tex
\[
    a \times b
\]
```
[COMPILATION_RESULT_END]: <> (Do not remove this line.)
$a \times b$

The content inside the equation expression uses a separate syntax to more easily
represent mathematical expressions. The content will be parsed and converted to
LaTeX. Most TeX commands should work as normal.
> ⚠️ As of version 0.3.0, TeX commands can be separated from their arguments by fractions. This can be solved by encasing the command in `{}`. This will be fixed in future versions.
#### General expressions
##### Lia
[COMPILATION_INPUT_START]: <> (Do not remove this line.)
```tex
eq* {
    x = (1 / 2 + 2 ^ 3) + \alpha
}
```
[COMPILATION_INPUT_END]: <> (Do not remove this line.)
##### TeX
[COMPILATION_RESULT_START]: <> (Do not remove this line.)
```tex
\[
    x = \left(\frac{1}{2} + 2^3\right) +\alpha
\]
```
[COMPILATION_RESULT_END]: <> (Do not remove this line.)
$x = \left(\frac{1}{2} + 2^3\right) +\alpha$

Operations are grouped by precedence, so `1 + 2 / 3` will be parsed as `1 + (2 / 3)` (it won't literally add brackets). operators are `+`, `-`, `*`, `/`, `%` and `^`. Other symbols such as `=` are treated as regular tokens or replaced if a [macro](#macros).
#### Expression with grouping
##### Lia
[COMPILATION_INPUT_START]: <> (Do not remove this line.)
```tex
eq* {
    f(x) = 1 / {2 + 2 ^ 3}
}
```
[COMPILATION_INPUT_END]: <> (Do not remove this line.)
##### TeX
[COMPILATION_RESULT_START]: <> (Do not remove this line.)
```tex
\[
    f \left(x\right) = \frac{1}{{2 + 2^3}}
\]
```
[COMPILATION_RESULT_END]: <> (Do not remove this line.)
$f \left(x\right) = \frac{1}{{2 + 2^3}}$

Note that tokens are separated by spaces, so `xyz` will be grouped but `x y z` will be separate which differs from pronumerals in LaTeX equations. This saves grouping pronumerals in `{}` in situations like `dy/dx`.
#### Matrices
##### Lia
[COMPILATION_INPUT_START]: <> (Do not remove this line.)
```tex
eq* {
    [[1, 2],
    [3, 4]]
}
```
[COMPILATION_INPUT_END]: <> (Do not remove this line.)
##### TeX
[COMPILATION_RESULT_START]: <> (Do not remove this line.)
```tex
\[
    \begin{pmatrix} 1 & 2 \\ 3 & 4 \end{pmatrix}
\]
```
[COMPILATION_RESULT_END]: <> (Do not remove this line.)
$\begin{pmatrix} 1 & 2 \\ 3 & 4 \end{pmatrix}$

#### Macros
| Token | Replacment | LaTeX |
|-|-|-|
| `<=` | `\le` | $\le$ |
| `>=` | `\ge` | $\ge$ |
| `+-` | `\pm` | $\pm$ |
| `-+` | `\mp` | $\mp$ |
| `=>` | `\implies` | $\implies$ |
| `!=` | `\ne` | $\ne$ |
| `->` | `\rightarrow` | $\rightarrow$ |
| `<-` | `\leftarrow` | $\leftarrow$ |
| `~==` | `\cong` | $\cong$ |
| `~=` | `\simeq` | $\simeq$ |
| `~~` | `\approx` | $\approx$ |
| `inf` | `\infty` | $\infty$ |
| `arcsin` | `\arcsin` | $\arcsin$ |
| `arccos` | `\arccos` | $\arccos$ |
| `arctan` | `\arctan` | $\arctan$ |
| `sinh` | `\sinh` | $\sinh$ |
| `cosh` | `\cosh` | $\cosh$ |
| `tanh` | `\tanh` | $\tanh$ |
| `coth` | `\coth` | $\coth$ |
| `sin` | `\sin` | $\sin$ |
| `cos` | `\cos` | $\cos$ |
| `tan` | `\tan` | $\tan$ |
| `cot` | `\cot` | $\cot$ |
| `sec` | `\sec` | $\sec$ |
| `csc` | `\csc` | $\csc$ |
| `log` | `\log` | $\log$ |
| `ln` | `\ln` | $\ln$ |

> If you don't want a macro to be replaced, you can separate it with spaces e.g. `s i n` will be parsed as the separate pronumerals $s$, $i$ and $n$ and not `\sin`.
-------------------

### Explicit version specification
The variable `@LIAVERSION` is reserved for specifying the version that the document is written in. If you specify a version, the document will be compiled with that version of the compiler otherwise it will use the latest version. It is recommended to specify a version to ensure that your document will compile correctly in the future. Always specify the version as the first line of the document.
```tex
@LIAVERSION = 0.3.0
```

## Document structure
* As LiA is designed for LaTeX documents, all document content will be automatically encased in a `document` environment. If there is a `document` environment annotated it will be ignored.
* Imports will be placed at the top of the document followed by variable declarations and then the document content.
* Things such as `documentclass` will not be automatically added to the document. You will need to add them manually as regular LaTeX commands.
