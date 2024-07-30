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

