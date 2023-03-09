# LiA Helper

This extension provides VS Code support for the LiA language.
This plugin currently only supports syntax highlighting however more features such as linting are planned.

## Installation
Copy the `lia-helper` folder into your VS Code extensions folder. This is usually `~\.vscode\extensions`.

Also make sure to add the following to your `settings.json` file:
```json
"[lia]": {
    "editor.wordWrap": "bounded",
}
```