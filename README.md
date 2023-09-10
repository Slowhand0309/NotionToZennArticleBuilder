# Notion To Zenn ArticleBuilder
Notionからエクスポートした記事をZennのフォーマットに変換するツールです。

## 機能
- 参照している画像を `image{x}` にリネームして指定された記事IDへ移動
  - x 部分はインクリメントされた値が入る (例: `image1`, `image2`, ...)
- 記事内の画像リンクを移動したリンクへ置換

## 使い方
1. Notionからエクスポートしたzipファイルを解凍
2. res ディレクトリに解凍したファイルを配置
3. `cargo run -- --filename="{記事名}.md" --article-id={記事ID}` で実行
4. `out` ディレクトリに変換後のファイルが出力される