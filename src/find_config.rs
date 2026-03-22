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
    if let Ok(p) = env::var(&env_key)
        && !p.trim().is_empty()
    {
        v.push(PathBuf::from(p));
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ── to_env_key ────────────────────────────────────────────────────────────

    #[rstest]
    #[case("esa-reader", "ESA_READER")]
    #[case("myapp", "MYAPP")]
    #[case("my_app", "MY_APP")]
    #[case("My App", "MY_APP")]
    #[case("app.name", "APP_NAME")]
    fn test_to_env_key(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(to_env_key(input), expected);
    }

    // ── dedup_paths ───────────────────────────────────────────────────────────

    #[rstest]
    fn test_dedup_paths_removes_exact_duplicates() {
        let mut paths = vec![
            PathBuf::from("/a/b/c"),
            PathBuf::from("/x/y/z"),
            PathBuf::from("/a/b/c"), // duplicate
        ];
        dedup_paths(&mut paths);
        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0], PathBuf::from("/a/b/c"));
        assert_eq!(paths[1], PathBuf::from("/x/y/z"));
    }

    #[rstest]
    fn test_dedup_paths_empty_stays_empty() {
        let mut paths: Vec<PathBuf> = vec![];
        dedup_paths(&mut paths);
        assert!(paths.is_empty());
    }

    #[rstest]
    fn test_dedup_paths_no_duplicates_unchanged() {
        let mut paths = vec![PathBuf::from("/a"), PathBuf::from("/b")];
        dedup_paths(&mut paths);
        assert_eq!(paths.len(), 2);
    }

    // ── find_config_path ──────────────────────────────────────────────────────

    /// When the environment variable `<APP>_CONFIG` is set to a non-empty
    /// string, `find_config_path` should include that path at the top of the
    /// candidate list (it becomes the recommended path).
    #[rstest]
    fn test_find_config_path_env_var_takes_priority() {
        // Use a unique env-var name to avoid interfering with other tests.
        let env_key = "ESA_READER_CONFIG";
        let custom_path = "/tmp/custom-esa-config.toml";
        // SAFETY: test-only env mutation; tests must not be run in parallel
        // within the same process when relying on env vars.
        unsafe { env::set_var(env_key, custom_path) };

        let result = find_config_path("esa-reader", "config.toml").unwrap();

        // The env-var path should be the recommended one (highest priority).
        assert_eq!(result.recommended, PathBuf::from(custom_path));
        assert!(
            result.candidates.contains(&PathBuf::from(custom_path)),
            "env-var path must appear in candidates"
        );

        unsafe { env::remove_var(env_key) };
    }

    /// Without any environment override the recommended path falls back to the
    /// first platform-standard location (XDG or ~/.config).
    #[rstest]
    fn test_find_config_path_recommended_is_first_candidate() {
        // Ensure the env-var is absent.
        let env_key = "ESA_READER_CONFIG";
        unsafe { env::remove_var(env_key) };

        let result = find_config_path("esa-reader", "config.toml").unwrap();

        assert_eq!(
            result.recommended, result.candidates[0],
            "recommended must equal candidates[0]"
        );
    }

    /// `existing` is `None` when no candidate file exists on disk.
    /// (We use a deliberately unlikely app name so no real config is present.)
    #[rstest]
    fn test_find_config_path_existing_is_none_when_absent() {
        let result = find_config_path("esa-reader-nonexistent-xyz-12345", "config.toml").unwrap();
        assert!(
            result.existing.is_none(),
            "no config should exist for a made-up app name"
        );
    }
}
