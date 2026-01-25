# esa-reader

esa.io の投稿一覧と本文をターミナル上で閲覧する TUI アプリです。設定ファイルにワークスペース（チーム）と API トークン、表示用のビュー（検索クエリ）を定義して利用します。

## できること
- 投稿一覧の表示（スター数付き）
- 投稿本文の表示（Markdown）
- ビュー（クエリ）をタブ切り替えして一覧を絞り込み

## 設定ファイル
TOML で設定します。アプリは `workspaces` の先頭に定義されたワークスペースを使用します。

```toml
[workspaces.default]
team_name = "my_team"
api_endpoint = "https://api.esa.io"
token = "my_token"

[workspaces.default.post_views.all]
title = "All Posts"
query = "sort:updated"
```

### 各項目
- `workspaces.<name>.team_name`: esa のチーム名
- `workspaces.<name>.api_endpoint`: API エンドポイント（現在の実装では未使用）
- `workspaces.<name>.token`: API トークン
- `workspaces.<name>.post_views.<name>.title`: タブに表示される名称
- `workspaces.<name>.post_views.<name>.query`: 一覧取得時の検索クエリ（未指定なら `sort:updated`）

## 設定ファイルのパス
以下の順で探索されます。最初に見つかったファイルが使用されます。

1. `ESA_READER_CONFIG` で指定されたパス
2. `$XDG_CONFIG_HOME/esa-reader/config.toml`
3. `~/.config/esa-reader/config.toml`
4. `~/Library/Application Support/esa-reader/config.toml`
5. `~/.esa-reader/config.toml`

見つからない場合は上記 1〜5 のうち最優先パスが推奨先として表示されます。

## 使い方
```bash
cargo run
```

### キーバインド
- `j` / `↓`: 下へ移動（一覧選択 / 本文スクロール）
- `k` / `↑`: 上へ移動（一覧選択 / 本文スクロール）
- `h` / `←`: 前のビューへ切り替え
- `l` / `→`: 次のビューへ切り替え
- `Enter`: 選択中の投稿を本文表示
- `w`: 選択中の投稿を watch する
- `W`: 選択中の投稿の watch を解除する
- `s`: 選択中の投稿を star する
- `S`: 選択中の投稿の star を解除する
- `q`: 終了
