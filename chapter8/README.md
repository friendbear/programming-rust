# Module

## モジュール

モジュールはRustの名前空間であり、Rustのプログラムやライブラリを構成する関数、型、定数などを収めるコンテナである。
クレートがプロジェクト間のコード共有の為のものにあるのに対し、モジュールはプロジェクト内部のコード構造化の為の機構。

### モジュールの複数ファイルへの分割

 モジュールは次のように書くことができる。

ここではsportesモジュールが、spores.rsという名前のファイルに入っている、とコンパイラに知らせている。

```rust
mod spores;
```

このsporesモジュールはコードが置かれている場所が異なる。

 モジュールをディレクトリで構成することもで出来る。コンパイラは、mod sportes;という宣言を見ると、
spores.rsとspores/mod.rsの両方をチェックする。
両方ともない場合、もしくは両方ともあったばあ愛にはエラーとなる。

### パスとインポート

モジュールの機能にアクセスするには::演算子を用いる。プロジェクト内のコードのどこからでも、任意の標準ライブラリの機能を
**絶対パス**で参照することができる。

モジュールは親モジュールの名前空間を自動的に引き継がない。たとえば、以下のようなproteins/mod.rsを書いた場合

```rust
//proteins/mod.rs
pub enum AminoAcid {...}
pub mod sythesis;
```

こうなっていても、synthesis.rsから、自動的にAminoAcidが見えるようにはならあない。

```rust
// proteins/synthesis.rs
pub fn synthesize(seq: &[AminoAcid]) // error: can't find type `AminoAcid`
```

個々のモジュールは白紙状態で始まるので、利用する名前をインポートする必要がある。

```rust
// proteins/synthesis.rs
use super::AminoAcid; // eexplicitly import form parent

pub fn syntheesizee(seq: &[AminoAacid])
```

 キーワードsuperはインポートの際には特別な意味を持ち、親モジュールのエイリアスになる。
同様にselfは現在のモジュールを指す。

```rust
// in proteins/mod.rs

//import from a submodule
use self::synthesis::synthesize;

// import nnames from an enum
// so we can write `Lys` for lysine, rather then
use self::AminoAcid::*;

```

サブモジュールは、親モジュールのプライベートアイテムにアクセスできるが、個別に名前でインポートされなければならない。
use super::*;としても、パブリックアイテムしかインポートされない。

モジュールとファイルは同じではないが、モジュールと、Unixファイルシステムのシステムのファイルとディレクトリの間には
自然な類似関係がある。useキーワードは、lnコマンドが作るリンクのようなエイリアスを作る。
パスはファイル名と同じように、絶対と相対の形を持つ。selfとsuperは、スペシャルディレクトリ.と..ににている。

### 標準のプレリュード

すべてのモジュールは、インポートされている名前という面では「白紙」の状態からスタートする、が完全に白紙というわけではない。

まず、標準ライブラリstdは自動的にすべてのプロジェクトにリンクされる。これは、lib.rsやmain.rs二以下のような宣言が見えない形で
描かれていると思えば良い。

`extern crate std;`

 さらにVecやRResultなどの、特によく使われているものが、標準のプレリュードに含まれていて、自動的にインポートされる。
Rustコンパイラは、ルートモジュールを含めてすべてんおモジュールの先頭に、下のようなインポート宣言があるかのようんに振る舞う。

`use std::prellude::v1::*;`

stdは含まれていないので、stdを使うには以下のように明示的に書く。

`use std;`

ただし、普通はstdの特定の利用する機能だけインポートした方が良いだろう。


### #[cfg()]

#[cfg]の完全な構文リストはRust言語仕様 <https://doc.rust-lang.org/reference.html#conditional-cocmplication>にある。

| #![cfg()] | 意味 |
| --- | --- |
| test | テストモードでのみコンパイル (--testでコンパイルされた時)|
| debug_assertions | デバッグアサーションが有効な時のみコンパイル |
| unix| macOS, Linux, FreeBSD, などのUnix系OSでのみコンパイル |
| windows | Windowsでのみコンパイル |
| target_pointer_width = "32" | ターゲットのポインタ幅が32ビットの時のみコンパイル |
| target_arch = "x86_64" | ターゲットのアーキテクチャ |
| target_os = "linux" | ターゲットのOS |
| feature = "foo" | fooという機能が有効な時のみコンパイル |
| not(foo) | fooが有効でない時のみコンパイル |
| all(foo, bar) | fooとbarが有効な時のみコンパイル |
| any(foo, bar) | fooかbarが有効な時のみコンパイル |

#### #[inline]

* #[inline(always)] その関数を常にインライン展開する
* #[inline(never)] その関数をインライン展開しない

> [!Tips]
> #[cfg]や#[allow]などの一部の属性は、モジュール全体に付加して、その中のすべてのアイテムに適用することができる。
> #[test]や#[inline]などは、個々のアイテム毎につけなければならない。

属性をクレート全体に付与するには、main.rsファイルもしくはlib.rsファイルの先頭に書く。
この際、#ではなく#!を用いる。


```rust
#[allow(non_camel_case_types)]
pub struct git_revspec {
    ...
}
pub struct git_commit {
    ...
}
```
#!は属性を次のアイテムに付与するのではなく、その行を包含する相手に対して付与することを意味する。
上のケースでは、git_revspecとgit_commitの両方にnon_camel_case_types属性が付与される。

### テストとドキュメント

#### 結合テスト

結合テストは、モジュールの外部インターフェースをテストするためのテストである。


#### ドキュメント

cargo doc usages
```bash
cargo doc --nno-deps --open
```

ドキュメントは、ライブラリのpubの付いたものと、それにつけたドキュメントコメントもしくはドクコメントから生成される。

３つのスラッシュで始まるコメントは、コンパイラによって#[doc]属性として解釈される。

#### バージョン

Semantic Versioning 2.0.0 <https://semver.org/> から採用されている。

#### Cargo.lock

Cargoが新しいバージョンに切り替えるのは、明示的に指示された場合、つまりユーザがCargo .tomlファイルを変更した場合と、
cargo updateコマンドを実行した場合のみである。

```toml
image = { git = "https://github.com/Piston/image.git", branch = "master" }
```

### クレートの公開 crate.io

クレートを公開するには、crates.ioに登録する必要がある。crates.ioに登録するには、crates.ioのアカウントを作成し、
cargo loginコマンドでログインする。

```bash
cargo login
```

cargo packageコマンドで、クレートをパッケージ化する。

```bash

cargo package
```

cargo publishコマンドで、クレートを公開する。

```bash
cargo publish
```

### ワークスペース

プロジェクトが成長するにつれ、多くのクレートが必要になることがある。このような場合、Cargoのワークスペース機能を使うと、
複数のクレートを一つのプロジェクトとして管理することができる。

ワークスペースは、Cargo.tomlファイルの[workspace]セクションで定義される。

```toml
members = [
    "foo",
    "bar",
    "baz",
]
```
