use std::{
    env, fs,
    path::{Path, PathBuf},
};

/// 設定ファイル探索の結果
#[derive(Debug, Clone)]
pub struct ConfigPathResult {
    /// 実際に見つかった設定ファイル（無ければ None）
    pub existing: Option<PathBuf>,
    /// 無かった場合に「ここに作るのが筋」な推奨パス
    pub recommended: PathBuf,
    /// 探索した候補（デバッグ用）
    #[allow(dead_code)]
    pub candidates: Vec<PathBuf>,
}

pub fn find_config_path(app_name: &str, file_name: &str) -> anyhow::Result<ConfigPathResult> {
    let candidates = candidate_config_paths(app_name, file_name)?;
    let existing = candidates.iter().find(|p| p.is_file()).cloned();

    // 推奨パス = 候補の先頭（優先度最高の場所）
    let recommended = candidates
        .first()
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("no candidates generated"))?;

    Ok(ConfigPathResult {
        existing,
        recommended,
        candidates,
    })
}

/// macOS / Linux で標準的な置き場所を生成
fn candidate_config_paths(app_name: &str, file_name: &str) -> anyhow::Result<Vec<PathBuf>> {
    let mut v = Vec::new();

    // 0) 明示指定があれば最優先（例: ESA_TUI_CONFIG=/path/to/config.toml）
    // アプリ名に応じて env var 名を変えてもOK
    let env_key = format!("{}_CONFIG", to_env_key(app_name));
    if let Ok(p) = env::var(&env_key) {
        if !p.trim().is_empty() {
            v.push(PathBuf::from(p));
        }
    }

    // 1) XDG_CONFIG_HOME（Linux で一般的、mac でも使う人がいる）
    if let Some(p) = env::var_os("XDG_CONFIG_HOME") {
        v.push(PathBuf::from(p).join(app_name).join(file_name));
    }

    // 2) ~/.config/<app>/<file>（Linux の定番）
    if let Some(home) = home_dir()? {
        v.push(home.join(".config").join(app_name).join(file_name));
    }

    // 3) macOS: ~/Library/Application Support/<app>/<file>
    // macOS 以外では存在しないことが多いが、候補に入れても害は少ない
    if let Some(home) = home_dir()? {
        v.push(
            home.join("Library")
                .join("Application Support")
                .join(app_name)
                .join(file_name),
        );
    }

    // 4) 旧来の置き方: ~/.<app>/config.toml
    if let Some(home) = home_dir()? {
        v.push(home.join(format!(".{app_name}")).join(file_name));
    }

    // 重複除去（同じパスが複数ルールで出ることがある）
    dedup_paths(&mut v);

    Ok(v)
}

/// 推奨パスへ保存する（必要なら親ディレクトリ作成）
pub fn ensure_parent_dir(path: &Path) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

/// 環境変数用に APP_NAME -> APP_NAME（大文字 + 非英数を _）へ変換
fn to_env_key(app_name: &str) -> String {
    app_name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect()
}

fn dedup_paths(v: &mut Vec<PathBuf>) {
    let mut out: Vec<PathBuf> = Vec::with_capacity(v.len());
    for p in v.drain(..) {
        if !out.iter().any(|x| same_path(x, &p)) {
            out.push(p);
        }
    }
    *v = out;
}

/// 文字列表現での簡易比較（OS 依存の厳密さが必要なら canonicalize を検討）
fn same_path(a: &Path, b: &Path) -> bool {
    a.as_os_str() == b.as_os_str()
}

/// HOME からホームディレクトリを取る（外部クレート無し版）
fn home_dir() -> anyhow::Result<Option<PathBuf>> {
    Ok(env::var_os("HOME").map(PathBuf::from))
}
