# History

## Todos


## 26-02

- 26-02-14 (26.2.5+self.14.1):
  - versionを更に改良。日付があったら便利と思って+14.1見たいのをつけることにした
    - version は `SemVer` という形式が必要らしく `n.n.n[-pre][+meta]` という形式とのこと
      - `n` の部分は 数値かつ 0スタート不可
      - `-pre`, `+meta` はオプションで `[0-9] [A-Z] [a-z] -` の範囲でフリーフォーマット
        - ただし `-pre` はバージョン管理に含まれ、よくあるパターンが `-alpha, -beta, -preview, -rc.1` など
          - rc: Release Candidate (ほぼ完成)
        - 更に `-pre` を記載した場合 厳密チェックが走り`+meta`も含んだチェックになるとのこと。併用は避けたほうがいい
  - map_err_str.rsを追加。
    - 主には tauriの返却用で `anyhow::Result` を `Result<T, STring>` に変換する際に `map_err(|e| e.to_string())` と変換するのを避けるための処置
    - traitを利用し様々な型に対応できそうなのでmapとして追加
- 26-02-11 (26.2.2-4):
  - remove sqlite. we can use library like `diesel`
  - sqlite version down for diesel
  - add `read_dir_deep()`
- 26-02-09 (26.1.5-26.1.6):
  - FileMetaをoptionにし、FileInfono生成コストを低減
    - 具体的には read_dir で取得できる DirEntry からは is_dir, is_file, is_symlink が 取得できるため、meta取得を行わないようにした

## 26-01

- 26-01-14 (26.1.4):
  - FileInfoの更新 meta内に is_image, is_movieフラグを移動, path_string() を追加
- 26-01-13 (26.1.3):
  - add `insert_row_id` to `sqlite_gateway` that return autoincrement id after insert.
- 26-01-11 (26.1.2):
  - sqlite select_one's result modify to `Result<Option<T>>`
- 26-01-10 (26.1.1):
  - mod implementation back from my_mod.rs to my_mod/mod.rs
  - version の管理を日付ベースに変更 (year.month.version)
  - tauri 系の処理を削除 (a2-rust-tauri-src へ移動)
- 26-01-04 (0.1.45-0.1.50):
  - AppHandle を参照で受け取るように変更(Command は参照だとコンパイルエラー)
  - sqlite の open/close でも borrow/move が発生しないように修正
  - sqlite の Mutex 利用周りの見直しと簡単なリファクタ
  - remove job_executor(move to other repository) because that use tauri crate
  - job_executor の追加 (同時実行数制限付きで複数ジョブ実行)
    - added tokio:`cargo add tokio --features full`
- 26-01-03 (0.1.44):
  - refactor FileMeta and FileInfo convert
- 26-01-02 (0.1.41-0.1.43):
  - fix permission
  - file のリファクタリング, is_dir や is_file に IO コストが掛かっているということで、meta から取得するように変更
    - is_dir, is_file は実態チェックだから、meta だと一括チェックしている
    - またそれに合わせて混同しないように,FileMeta も新設して変更
- 26-01-01 (0.1.40):
  - add from to file_info
  - file info 内でのエラー解決をリファクタ
    - および TimeStamp を struct で管理
- 25-12-30 (0.1.39):
  - fix pathbuf unwarap 親ディレクトリが取れないときにエラーを出してた
- 25-12-30 (0.1.37 - 0.1.38):
  - add save_jpeg_80 for quick save
  - add resize_aspect_ratio, save_image to images
- 25-12-29 (0.1.36):
  - add display::size
- 0.1.35:
  - remove dhildern to FileInfo
  - add `FileEntry<T>` for manage addtional type manage
- 0.1.34:
  - add children to FileInfo
- 0.1.33:
  - remove tauri/filer that only can implment in srt-tauri
- 0.1.32:
  - add tauri tauri/command to use from app
- 0.1.31:
  - add open_filer to tauri
  - small fix use and mod files
