# VSCode Configuration

```json
{
    "recommendations": [
        "EditorConfig.EditorConfig",
        // Rust extensions
        "rust-lang.rust-analyzer",
        "tamasfe.even-better-toml",
        // Web extensions
        "esbenp.prettier-vscode",
        "dbaeumer.vscode-eslint"
    ],
    "search.exclude": {
        // Rust paths
        "**/target/**": true,
        // Web paths
        "**/node_modules/**": true,
        "**/dist/**": true
    },
    // Rust extensions settings
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer",
        "editor.formatOnSave": true
    },
    "rust-analyzer.cargo.extraEnv": { "DATABASE_URL": "sqlite://database.db" },
    "rust-analyzer.rustfmt.extraArgs": ["+nightly"],
    "rust-analyzer.check.command": "clippy",
    // Web extensions settings
    "[javascript]": {
        "editor.defaultFormatter": "esbenp.prettier-vscode",
        "editor.formatOnSave": true
    },
    "[typescript]": {
        "editor.defaultFormatter": "esbenp.prettier-vscode",
        "editor.formatOnSave": true
    },
    "[typescriptreact]": {
        "editor.defaultFormatter": "esbenp.prettier-vscode",
        "editor.formatOnSave": true
    },
    "[html]": {
        "editor.defaultFormatter": "esbenp.prettier-vscode",
        "editor.formatOnSave": true
    },
    "[scss]": {
        "editor.defaultFormatter": "esbenp.prettier-vscode",
        "editor.formatOnSave": true
    },
    "[json]": {
        "editor.defaultFormatter": "esbenp.prettier-vscode",
        "editor.formatOnSave": true
    }
}
```
