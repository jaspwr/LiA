# LiA

>⚠️This is the documentation for version 0.1.0. This is only an early version and is still in development. Do not expect any of these features to stay the same in future versions. Additionally, the compiler has not been thoroughly tested and may not work as expected. If you find any bugs or have any suggestions please open an issue or pull request on the [GitHub repository](https://github.com/jaspwr/LiA). 

Most TeX is valid in LiA so you are able to write LaTeX as normal however with the addition of the features listed below.

## Language features

### Environments
#### Lia
```tex
env environmenttype {
    content
}
```
#### TeX
```tex
\begin{environmenttype}
    content
\end{environmenttype}
```

-------------------

### Imports
* Any row opened with a `use` keyword (excluding whitespace) will be treated as an import statement.

| LiA                     | TeX                       |
|-------------------------|---------------------------|
| `use packagename`       | `\usepackage{packagename}`|
|`use packagename, otherpackage, thirdpackage`| `\usepackage{packagename}`<br>`\usepackage{otherpackage}`<br>`\usepackage{thirdpackage}`|
|`use [option]packagename`| `\usepackage[option]{packagename}`|

* Consumes remainder of line.

-------------------

### Italic
| LiA                    | TeX                      |
|------------------------|--------------------------|
| `**Inner text**`       | `\textit{Inner text}`    |
* Note this differs from the single `*` in markdown.
-------------------

### Bold
| LiA                     | TeX                      |
|-------------------------|--------------------------|
| `***Inner text***`      | `\textbf{Inner text}`    |
* Note this differs from the `**` in markdown.

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
* Consumes remainder of line. For multiline enclose the section title in `{}`.

-------------------

### Markdown style lists
* Any row opened with a `*` will be treated as a list item. You can create nested lists with indentation. In most cases the indentation type will be inferred. 
#### Lia
```tex
* List item.
* List item.
    * Nested item.
        * Double Nested item.
* List item.
```
#### TeX
```tex
\begin{itemize}
    \item List item.
    \item List item.
    \begin{itemize}
        \item Nested item.
        \begin{itemize}
            \item Double Nested item.
        \end{itemize}
    \end{itemize}
    \item List item.
\end{itemize}
```
* List items consume the remainder of the line. For multiline enclose list item contents in `{}`.

-------------------

### Variables

* Any word annotated with a `@` will be treated as a variable.
#### Referencing variables
> ⚠️ As of version 0.1.0, variables with computed arguments can not be used before they are defined. This will be fixed in future versions.

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
* Will consume until unnested `}`.
##### Computed functions
* If inside the contents of a function an expression in `@()` is found it will be evaluated and whenever it is referenced the result will be computed and passed in as an additional argument.
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
* Supported operators are for `Number` are `+`, `-`, `*`, `/`, and `%`.
* Supported operators for `String` are `+`.
> ⚠️ As of version 0.1.0, there are no unary operators. This will be added in a future version.
* Will consume until unnested `}`.


-------------------

### Equations
> ⚠️ As of version 0.1.0, this feature is still in development. Eventually it will be possible to use a more ergonomic syntax for equations inside of the `{}` in equation statements however in it's current state it will just treat the contents the same as the rest of the document.
#### Numbered
##### Lia
```tex
eq {
    content
}
```
##### TeX
```tex
\begin{equation}
    content
\end{equation}
```
#### Anonymous
##### Lia

```tex
eq* {
    content
}
```
##### TeX
```tex
\[
    content
\]
```

## Document structure
* As LiA is designed for LaTeX documents, all document content will be automatically encased in a `document` environment. If there is a `document` environment annotated it will be ignored.
* Imports will be placed at the top of the document followed by variable declarations and then the document content.
