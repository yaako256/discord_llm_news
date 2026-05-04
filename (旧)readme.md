# メモ
parameterの頭文字検索がちょっとおかしい部分があるのでそれを修正する。(hで-hが出るとか)
string型指定が第1パラを奪う問題が解決してない。今は定義順で解決してる。
helpをコマンドのインターフェースに含めるか否か問題。
エラーログをデバックレスポンスじゃなくしたい。ちゃんとHTTPコード使う？
---
---

# プロジェクト概要
## プロジェクト名
`1_command_and_ui`

## プロジェクトの目的
```
初めてTauriなどのGUIツールを使わずにプロジェクトを作る。
このプロジェクトでは、簡単なコマンドシステムの実装・検証を目的とする。
今回はバックエンド側がメイン
フロントエンド側は、入力用input_fieldと出力用text_boxのみのような簡易な作りにする。
```


## プロジェクト構成
本プロジェクトは、フロントエンドとバックエンドでディレクトリを分けて管理している
- `frontend/`: React (Vite)
- `backend/`: Rust (Cargo)
---
---
# ビルド&実行の備忘録
フロントエンド側とバックエンド側は完全に分かれている。
それぞれでビルドし、それぞれを実行する形になっている。

## 1. フロントエンド側 (Frontend)
ちゃんとFrontendフォルダに行って実行する。
基本的に開発用を使ってればok
```bash
# 開発中に使うコマンド
# 起動は早いが最適化が行われない
npm run dev

# 本番用で使うコマンド
# ファイルが最適化(ビルド)される
npm run build

# プレビュー
# ビルドしたものが正しく動作するかの確認
# ローカルでdistフォルダを確認できるのが存在意義
npm run preview
```

## 2. バックエンド側 (Backend)
ちゃんとBackendフォルダに行って実行する。
基本的に開発用を使ってればok
```bash
# 開発中に使うコマンド
cargo run

# ビルドだけしたいとき
# 後で立ち上げたいとき
cargo build

# 本番用(リリースビルド)
# 最適化を最大限に効かせる
cargo build --release
```

## 3. 同時に実行
フロントエンドをバックエンドを同時に実行するときのコマンド。<br>
セットアップ備忘録に後述するdev.batを実行する。<br>
dev.batがあるルートディレクトリで実行すること。
```bash
# dev.batが実行される
.\dev.bat
```

---
---
# セットアップ備忘録
プロジェクト作成の手順と選択したオプションの記録。<br>
フロントエンドとバックエンドでそれぞれプロジェクトを作った。

## 1. フロントエンド側 (Frontend)
npmコマンドを使って既存のディレクトリ（`.`）に対してプロジェクトを展開した。

**実行コマンド:**
```bash
npm create vite@latest .
```

**選択オプション:**
| 項目 | 選択内容 |
| :--- | :--- |
| **Select a framework** | React |
| **Select a variant** | TypeScript + React + Compiler |
| **Install with npm and start now?** | Yes |
<br>
**`.`（カレントディレクトリ）指定による挙動のメモ:**
```
・Project name: コマンドを実行したフォルダ名が自動採用された。(多分)
・Package name: 同上。
・展開場所: そのフォルダ直下にファイル群が展開された。
・補足: プロジェクト名を手動で設定するステップはスキップされた。
```

---

## 2. バックエンド側 (Backend)

RustのパッケージマネージャであるCargoを使用。<br>
今回は既存の `backend` ディレクトリをそのままプロジェクト化した。

**実行コマンド:**
```bash
# 既存ディレクトリをプロジェクト化する場合
cargo init

# (参考)新規ディレクトリを作成して開始する場合
# cargo new <Project_name>
```

## 3. ルート側(root)
### 3.1 テスト関係
フロントエンドとバックエンドを同時に実行させるためにbatファイルを作る。<br>
batファイルはwindowsしか対応してないから注意。<br>
また、batファイルに日本語を含めるとエラーが出るため注意。
新たに2つのコマンドプロンプトが開かれるのは面倒であるため、バックエンドの処理はvscodeのターミナルで行わせる。
```bat
:: Function to run both frontend and backend at the same time
@echo off

:: Displaying a message (log)
echo Starting both frontend and backend development servers...

:: [Run frontend]
start cmd /c "cd frontend && npm run dev"


:: [Run backend]
:: When using a new command prompt
:: start cmd /c "cd backend && cargo run"

:: When using the VS Code terminal
cd backend && cargo run
```
**要素の解説:**
| 要素 | 動作内容 |
| :--- | :--- |
| @echo off | ターミナルにコマンドを表示しないようにする |
| echo | ターミナルに文字列を出力する |
| start | 新しいコマンドプロンプトを開く |
| cmd /c | 実行が終了したらコマンドプロンプトを閉じる |

### 3.2 Github関係
Githubにリポジトリとして認識させる。
プロジェクトのルートフォルダで行う。

**実行コマンド:**
```bash
git init
```

---
---

## 開発スタック
- **Frontend:** React, TypeScript, Vite (React Compiler有効)
- **Backend:** Rust (Cargo)
- **Package Manager:** npm
