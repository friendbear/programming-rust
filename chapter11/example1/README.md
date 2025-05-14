# トレイトとジェネリクス

## トレイトの使い方

トレイトは、任意の型がサポートすることができる性質。
多くの場合トレイトは何らかの機能、つまりその型ができることを表す。

* std::io::Writeを実装した値には、バイト列を書き込むことができる。
* std::iter::Iteratorを実装した値は、値の列を生成することができる。
* std::clone::Cloneを実装した値は、自分のクローンをメモリ上に作成することができる。
* std::fmt::Debugを実装した値は、デバッグ表示を生成することができる。

これらのトレイトは全て、Rustの標準ライブラリに含まれている。

* std::fs::Fileはstd::io::Writeを実装している。
* Range<i32> (0..10の型)hはIteratorを実装している。スライスやハッシュテーブルに関するイテレータも同様。
* 標準ライブラリのほとんどの型はCloneを実装している。TcpStreamのように、メモリ内のデータいが愛のものを表している型は例外だ。
* 標準ライブラリのほとんどの型はDebugを実装している。

### トレイトオブジェクト

トレイトを使って多様性のあるコードを書く方法は２つある。トレイトオブジェクトとジェネリクスだ。

RustではWrite型の変数を持つことができない。
Write型の変数を持つことができるのは、Writeトレイトを実装した具体的な型だけだ。

```rust
use std::io::Write;
let mut buf: Vec<u8> = vec![];
let mut writer: Write = buf; // エラー；Writeはサイズが不定
```

変数のサイズはコンパイル時に、決まってなければならない。しかし、Writeはどんな方でも実装できるので、サイズが不定である。

Rustでは、参照だということを明示しなければならない。
```rust
use std::io::Write;
let mut buf: Vec<u8> = vec![];
let mut writer: &mut Write = &mut buf; // ok
```

上のwriterのようなトレイト型への参照を、トレイトオブジェクトっと呼ぶ。他の参照と同様に、トレイトオブジェクトは、何らかの値
を指し、生存期間を持ち、mutか共有可能かのどちらかだ。

トレイトオブジェクトが特殊なのは、コンパイル時に参照先の実際の値がわからないことだ。そこで、トレイトオブジェクトには参照先の型に
関する情報が追加されている。この情報は、Rustが舞台裏でだけ使う。writer.write(data)が呼び出されると、*writerの型に応じた
正しいwriteメソッドを動的に呼び出すために、型の情報が必要になる。直接型情報を問いあ合わせることはないし、&mut Writeから、
例えばVec<u8>のような実際の型にダウンキャストすることもできない。

```rust
use std::io::Write;

fn write_all<W: Write>(writer: &mut W, buf: &[u8]) {
    writer.write_all(buf).unwrap();
}
```

### トレイトオブジェクトのメモリ配置

メモリ上ではトレイトオブジェクトはファットポインタで、値へのポインタと型情報へのポインタを持っている。
個々のトレイトオブジェクトは、２ワード長ということになる。

Rustは、通常の参照を必要に応じて自動的にトレイトオブジェクトに変換する。この例では &mut local_fileをsay_helloに渡すことができるのは
このおかげである。
```rust
let mut local_file = File::create("hello.txt)?;
say_hello(&mut loca_file)?;
```

&mut local_fileの型は&mut Fileで、say_helloの引数の型は&mut Writeだ。FileはWriteの一部なので、通常の参照からトレイトオブジェクト
への変換は自動的に行われる。
 同様にBox<File>かあらBox<Write>への変換も可能だ。ヒープ上にあるWriteを実装した型の値を所有する値となる。

```rust
let w: Box<Write> = Box::new(local_file);
```

```rust
let rc: Rc<Write> = Rc::new(local_file);
```

### ジェネリック関数

say_hello関数をジェネリック関数で書き直すと以下の通り。
```rust
/// generic function
fn say_hello<W: Write>say_hello(out: &mut W) -> io::Result<()> {
    out.write_all(b"Hello, world!\n");
    out.flush();
}
```
変わったのはシグネチャ部分だけで、関数の本体は変わっていない。ジェネリック関数は、トレイトオブジェクトを使う場合には、
トレイトオブジェクトを引数として受け取ることができない。トレイトオブジェクトは、参照でなければならないためだ。
```rust
/// plain function
fn say_hello(out: &mut Write) -> io::Result<()> {
    out.write_all(b"Hello, world!\n");
    out.flush();
}
```

<W: Write> の部分により、関数がジェネリックになる。この部分が**型パラメータ**で、関数のボディー部全体を通してWはWriteトレイトを実装した
何らかの型だ、ということを示している。この関数は、どんな型の引数も受け取ることができるが、その型はWriteトレイトを実装していなければならない。
&mut local_fileを渡すと、say_helllo<File>()を呼び出したことになる。
&mut bytesを渡すと、say_hello<Vec<u8>>()を呼び出したことになる。

いずれの場合も、コンパイラはWを引数の型から推論する。次のように、型パラメータを明示的に指定することもできる。
```rust
say_hello::<File>(&mut local_file)?;
```

ヒントになるような引数を持たないジェネリック関数の場合には明示的に書く必要がある。

```rust
/// calling a generic method colllect<C>() that takes no arguments
let v1 =(0..1000).collect; // エラー
let v2 = (0..1000).collect::<Vec<i32>>(); // ok
```

型パラメータに複数の機能を要求したい場合がある。例えば、ベクタの中から頻度の高い値のTop10を表示したいとしよう。
その場合は表示可能である必要がある。

code top_ten

次の例が表すように、制約は簡単には理解できないほど長くなることがある。そこで、Rustにはキーワードwhereがある。
whereキーワードを使うと、型パラメータの制約を関数のシグネチャから分離することができる。

```rust
fn run_query<M, R>(query: &str) -> Result<R, M::Error>
where
    M: Database,
    R: FromRow,
{
    let mut conn = M::connect("localhost")?;
    let result = conn.run_query(query)?;
    Ok(R::from_row(result))
}
```

whereキーワードを使うと、型パラメータの制約を関数のシグネチャから分離することができる。この例では、MはDatabaseを実装し、RはFromRowを実装している必要がある。

生存期間パラメータの構文は、ジェネリック関数は、生存期間パラメータと型パラメータの両方を持つことができる。

```rust
/// Return a reference to the point in `candidates` that`s
/// closet to the `target` point.
fn nearest<'t, 'c, P>(target: &'t P, candidates: &'c [P]) -> &'c P
where
  P: MeasureDiistance
{
    ...
}
```

### どちらを使うべきか

トレイトオブジェクトとジェネリック関数のどちらを使うかは、微妙な問題だ。どちらもトレイトに基づいているので、多くの点が共通している。
複数の方が入り混じっているコレクションを扱う場合には、トレイトオブジェクトを使うのが正しい。
ジェネリックなサラダを作ることも技術的には可能だ。


```rust
trait Vegetable {

}
struct Salad<V: Vegettable> {
    veggies: Vec<V>,
}
```
しかしこれは厳しい、個々のサラダは常に同じ種類の野菜を持つことになる。ジェネリックなサラダは、異なる種類の野菜を混ぜることができない。
トレイトオブジェクトを使うと、異なる種類の野菜を混ぜることができる。

```rust
struct Salad {
    veggies: Vec<Box<Vegetable>>,
}
```

## トレイトの定義と実装

トレイトの定義は簡単だ。トレイトの名前と、トレイトが提供するメソッドのシグネチャを書くだけである。

```rust
trait Animal {
    fn noise(&self) -> &'static str;
}
```

トレイトを実装するには、implブロックを使う。implブロックは、トレイトの名前と、トレイトを実装する型の名前を指定する。

```rust
struct Dog;
impl Animal for Dog {
    fn noise(&self) -> &'static str {
        "bark"
    }
}
```

### トレイトと他人の定義した型

任意の型に対して任意のトレイトを実装することができる。ただし、トレイトか型のどちらかがそのスコープで新たに導入されている場合に限る。

```rust
trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        ...
    }
}
```

#### 拡張トレイト

ジェネリックImplブロックを書いて、様々な型に対して一気に拡張トレイトを追加することもできる。
以下に示す拡張トレイトは、Rustの全てのWriteを実装した型に対してメソッドを追加する。

```rust
use std::io::{self, Write}

/// Trait for values to which you can send HTML.
trait WriteHtml {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()>;
}
impl<W: Write> WrittHtml for W {
    fn write_html(&mut self, html: &HtmlaDocument) -> io::Result<()> {
        ...
    }
}

### 型と型の関係を定義するトレイト

これまで見てきたトレイトは全て独立しいていた。型が実装するメソッドの集合を定義するトレイトだ。
トレイトは、服数の型が連携して動作するような場合にも利用できる。
トレイトで型の関係をk記述する。

* std::iter::Iteratorトレイトは、個々のイテレータ型と生成される値の型を関連付ける。
* std::opt::Mulトレイトは、掛け合わせる２つの型を関連付ける。a * bのような式があった場合、値aとbは同じ型でも良いし別の型でも良い。
* randクレートには、乱数を生成するトレイト(rand::Rng)と、乱数として生成されることの出来る型を示すトレイト(rand::Rand)が含まれている。
  これらのトレイトは、これらの型がどのように連携できるかを定義している。

#### 関連型: イテレータはどう機能するのか
