/*

Rust のプログラムでは、データを保持するために次の3種類のメモリ空間を持っています。

データメモリ 
- 固定長もしくは スタティック (例: プログラムのライフサイクルで常に存在するもの)なデータ。 
  プログラム内の文字列(例: ‘Hello World’), この文字列のキャラクタは読み取りにしか使えないため、
  この領域に入ります。 コンパイラはこういったデータに対してチューニングをしており、
  メモリ上の位置はすでに知られていてかつ固定であるため、非常に速く使うことができます。

スタックメモリ 
- 関数内で宣言された変数。 関数が呼び出されている間は、メモリ上の位置は変更されることがないため、
  コンパイラからするとチューニングができるので、スタックメモリも非常に速くデータにアクセスできます。

ヒープメモリ 
- プログラムの実行時に作られるデータ。 このメモリにあるデータは追加、移動、削除、
  サイズの調節などの操作が許されています。動的であるため、遅いと思われがちですが、 
  これによりメモリの使い方に柔軟性を生み出すことができます。
  データをヒープメモリに入れることをアロケーション(allocation)といい、
  データをヒープメモリから削除することはディアロケーション(deallocation)と言います。
*/
