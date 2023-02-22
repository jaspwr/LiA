# Planned features
- [ ] Add codeblocks and inline codeblocks. Possible use https://ctan.org/tex-archive/macros/latex/contrib/listings/ for syntax highlighting.
- [ ] Labels for equation statements (easy and important).
- [ ] Fix everything that has a warning in the documentation.
- [ ] Add some kind of style system.
- [ ] Add an alternative syntax for tables.
- [ ] Easier hyperlinks.
- [ ] Vim syntax highlighting.
- [ ] Create linter. Should also show LaTeX errors.
- [ ] Add shorthands for Greek letters.
- [ ] Configure automatic builds and releases on GH actions and set up a proper changelog file.
- [ ] Compile multiple files at once.

# Known issues
- [ ] Comments don't really work. `%` should open comments like in TeX which prevent things from being transpiled.
- [ ] Comments are very broken in the syntax highlighter.
- [ ] `eq` and `eq*` are broken in the syntax highlighter.
- [ ] Commas break in equation statements. They should be normal outside of square brackets.
- [ ] Command chaining is just broken in general.

# Other
- [ ] Refactor all the ugly parts.
- [ ] Improve line/ column references in error messages. Basically every error should have one. Internal syntax of equations statements and @() just reference the start of the statement and should be improved.
- [ ] Write documentation in LiA once codeblocks are supported.