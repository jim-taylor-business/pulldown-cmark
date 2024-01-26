// This file is auto-generated by the build script
// Please, do not modify it manually

use super::test_markdown_html;

#[test]
fn table_test_1() {
    let original = r##"Test header
-----------
"##;
    let expected = r##"<h2>Test header</h2>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_2() {
    let original = r##"Test|Table
----|-----
"##;
    let expected = r##"<table><thead><tr><th>Test</th><th>Table</th></tr></thead><tbody></tbody>
</table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_3() {
    let original = r##"> Test  | Table
> ------|------
> Row 1 | Every
> Row 2 | Day
>
> Paragraph
"##;
    let expected = r##"<blockquote>
<table><thead><tr><th>Test</th><th>Table</th></tr></thead><tbody>
<tr><td>Row 1</td><td>Every</td></tr>
<tr><td>Row 2</td><td>Day</td></tr>
</tbody></table>
<p>Paragraph</p>
</blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_4() {
    let original = r##" 1. First entry
 2. Second entry

    Col 1|Col 2
    -|-
    Row 1|Part 2
    Row 2|Part 2
"##;
    let expected = r##"<ol>
<li>
<p>First entry</p>
</li>
<li>
<p>Second entry</p>
<table><thead><tr><th>Col 1</th><th>Col 2</th></tr></thead><tbody>
<tr><td>Row 1</td><td>Part 2</td></tr>
<tr><td>Row 2</td><td>Part 2</td></tr>
</tbody></table>
</li>
</ol>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_5() {
    let original = r##"|Col 1|Col 2|
|-----|-----|
|R1C1 |R1C2 |
|R2C1 |R2C2 |
"##;
    let expected = r##"<table><thead><tr><th>Col 1</th><th>Col 2</th></tr></thead><tbody>
<tr><td>R1C1</td><td>R1C2</td></tr>
<tr><td>R2C1</td><td>R2C2</td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_6() {
    let original = r##"| Col 1 | Col 2 |
|-------|-------|
|       |       |
|       |       |
"##;
    let expected = r##"<table><thead><tr><th>Col 1</th><th>Col 2</th></tr></thead><tbody>
<tr><td></td><td></td></tr>
<tr><td></td><td></td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_7() {
    let original = r##"| Col 1 | Col 2 |
|-------|-------|
|   x   |       |
|       |    x  |
"##;
    let expected = r##"<table><thead><tr><th>Col 1</th><th>Col 2</th></tr></thead><tbody>
<tr><td>x</td><td></td></tr>
<tr><td></td><td>x</td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_8() {
    let original = r##"|Col 1|Col 2|
|-----|-----|
|✓    |✓    |
|✓    |✓    |
"##;
    let expected = r##"<table><thead><tr><th>Col 1</th><th>Col 2</th></tr></thead><tbody>
<tr><td>✓</td><td>✓</td></tr>
<tr><td>✓</td><td>✓</td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_9() {
    let original = r##"|  Target                       | std |rustc|cargo| notes                      |
|-------------------------------|-----|-----|-----|----------------------------|
| `x86_64-unknown-linux-musl`   |  ✓  |     |     | 64-bit Linux with MUSL     |
| `arm-linux-androideabi`       |  ✓  |     |     | ARM Android                |
| `arm-unknown-linux-gnueabi`   |  ✓  |  ✓  |     | ARM Linux (2.6.18+)        |
| `arm-unknown-linux-gnueabihf` |  ✓  |  ✓  |     | ARM Linux (2.6.18+)        |
| `aarch64-unknown-linux-gnu`   |  ✓  |     |     | ARM64 Linux (2.6.18+)      |
| `mips-unknown-linux-gnu`      |  ✓  |     |     | MIPS Linux (2.6.18+)       |
| `mipsel-unknown-linux-gnu`    |  ✓  |     |     | MIPS (LE) Linux (2.6.18+)  |
"##;
    let expected = r##"<table><thead><tr><th>Target</th><th>std</th><th>rustc</th><th>cargo</th><th>notes</th></tr></thead><tbody>
<tr><td><code>x86_64-unknown-linux-musl</code></td><td>✓</td><td></td><td></td><td>64-bit Linux with MUSL</td></tr>
<tr><td><code>arm-linux-androideabi</code></td><td>✓</td><td></td><td></td><td>ARM Android</td></tr>
<tr><td><code>arm-unknown-linux-gnueabi</code></td><td>✓</td><td>✓</td><td></td><td>ARM Linux (2.6.18+)</td></tr>
<tr><td><code>arm-unknown-linux-gnueabihf</code></td><td>✓</td><td>✓</td><td></td><td>ARM Linux (2.6.18+)</td></tr>
<tr><td><code>aarch64-unknown-linux-gnu</code></td><td>✓</td><td></td><td></td><td>ARM64 Linux (2.6.18+)</td></tr>
<tr><td><code>mips-unknown-linux-gnu</code></td><td>✓</td><td></td><td></td><td>MIPS Linux (2.6.18+)</td></tr>
<tr><td><code>mipsel-unknown-linux-gnu</code></td><td>✓</td><td></td><td></td><td>MIPS (LE) Linux (2.6.18+)</td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_10() {
    let original = r##"|-|-|
|ぃ|い|
"##;
    let expected = r##"<p>|-|-|
|ぃ|い|</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_11() {
    let original = r##"|ぁ|ぃ|
|-|-|
|ぃ|ぃ|
"##;
    let expected = r##"<table><thead><tr><th>ぁ</th><th>ぃ</th></tr></thead><tbody>
<tr><td>ぃ</td><td>ぃ</td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_12() {
    let original = r##"|Колонка 1|Колонка 2|
|---------|---------|
|Ячейка 1 |Ячейка 2 |
"##;
    let expected = r##"<table><thead><tr><th>Колонка 1</th><th>Колонка 2</th></tr></thead><tbody>
<tr><td>Ячейка 1</td><td>Ячейка 2</td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_13() {
    let original = r##"table a
|  a  |  b  |
| --- | --- |
|  c  |  d  |


table b
    |  a  |  b  |
    | --- | --- |
    |  c  |  d  |


table c
 a  |  b
--- | ---
 c  |  d


table d
    a | b
    --|--
    c | d


table e
a | b
--|--
c | d

table f
  |  a  |  b  |
  | --- | --- |
  |  c  |  d  |


table g
   a  |  b
  --- | ---
   c  |  d

table h
a
|-|
b

table i
| a
|-
b

table j
| a
-
b
"##;
    let expected = r##"<p>table a</p>
<table><thead><tr><th>a</th><th>b</th></tr></thead><tbody>
<tr><td>c</td><td>d</td></tr>
</tbody></table>
<p>table b
|  a  |  b  |
| --- | --- |
|  c  |  d  |</p>
<p>table c
a  |  b
--- | ---
c  |  d</p>
<p>table d
a | b
--|--
c | d</p>
<p>table e
a | b
--|--
c | d</p>
<p>table f</p>
<table><thead><tr><th>a</th><th>b</th></tr></thead><tbody>
<tr><td>c</td><td>d</td></tr>
</tbody></table>
<p>table g
a  |  b
--- | ---
c  |  d</p>
<p>table h
a
|-|
b</p>
<p>table i</p>
<table><thead><tr><th>a</th></tr></thead><tbody>
<tr><td>b</td></tr>
</tbody></table>
<h2>table j
| a</h2>
<p>b</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_14() {
    let original = r##"a | b
- | -
1 | 2
"##;
    let expected = r##"<table><thead><tr><th>a</th><th>b</th></tr></thead><tbody>
<tr><td>1</td><td>2</td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_15() {
    let original = r##"a | b\
- | -
1 | 2
"##;
    let expected = r##"<p>a | b\</p>
<ul>
<li>| -
1 | 2</li>
</ul>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_16() {
    let original = r##"a\
| b | c |
|---|---|
| d | e |
"##;
    let expected = r##"<p>a\</p>
<table><thead><tr><th>b</th><th>c</th></tr></thead><tbody>
<tr><td>d</td><td>e</td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_17() {
    let original = r##"| Description | Test case |
|-------------|-----------|
| Single      | `\`       |
| Double      | `\\`      |
| Basic test  | `\|`      |
| Basic test 2| `\|\|\`   |
| Basic test 3| `x\|y\|z\`|
| Not pipe    | `\.`      |
| Combo       | `\.\|\`   |
| Extra       | `\\\.`    |
| Wait, what? | `\\|`     |
| Wait, what? | `\\\|`    |
| Wait, what? | `\\\\|`   |
| Wait, what? | `\\\\\|`  |
| Wait, what? |          \|
| Wait, what? |         \\|
| Wait, what? |        \\\|
| Wait, what?x|          \|x
| Wait, what?x|         \\|x
| Wait, what?x|        \\\|x
| Direct trail|         \.|x
"##;
    let expected = r##"<table><thead><tr><th>Description</th><th>Test case</th></tr></thead><tbody>
<tr><td>Single</td><td><code>\</code></td></tr>
<tr><td>Double</td><td><code>\\</code></td></tr>
<tr><td>Basic test</td><td><code>|</code></td></tr>
<tr><td>Basic test 2</td><td><code>||\</code></td></tr>
<tr><td>Basic test 3</td><td><code>x|y|z\</code></td></tr>
<tr><td>Not pipe</td><td><code>\.</code></td></tr>
<tr><td>Combo</td><td><code>\.|\</code></td></tr>
<tr><td>Extra</td><td><code>\\\.</code></td></tr>
<tr><td>Wait, what?</td><td><code>\|</code></td></tr>
<tr><td>Wait, what?</td><td><code>\\|</code></td></tr>
<tr><td>Wait, what?</td><td><code>\\\|</code></td></tr>
<tr><td>Wait, what?</td><td><code>\\\\|</code></td></tr>
<tr><td>Wait, what?</td><td>|</td></tr>
<tr><td>Wait, what?</td><td>|</td></tr>
<tr><td>Wait, what?</td><td>\|</td></tr>
<tr><td>Wait, what?x</td><td>|x</td></tr>
<tr><td>Wait, what?x</td><td>|x</td></tr>
<tr><td>Wait, what?x</td><td>\|x</td></tr>
<tr><td>Direct trail</td><td>.</td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_18() {
    let original = r##"| Single | `\|` |
|--|--|
| Single | `\|` |


| Double | `\\|` |
|--|--|
| Double | `\\|` |


| Double Twice | `\\|\\|` |
|--|--|
| Double Twice | `\\|\\|` |


| Triple | `\\\|` |
|--|--|
| Triple | `\\\|` |
"##;
    let expected = r##"<table><thead><tr><th>Single</th><th><code>|</code></th></tr></thead><tbody>
<tr><td>Single</td><td><code>|</code></td></tr>
</tbody></table>
<table><thead><tr><th>Double</th><th><code>\|</code></th></tr></thead><tbody>
<tr><td>Double</td><td><code>\|</code></td></tr>
</tbody></table>
<table><thead><tr><th>Double Twice</th><th><code>\|\|</code></th></tr></thead><tbody>
<tr><td>Double Twice</td><td><code>\|\|</code></td></tr>
</tbody></table>
<table><thead><tr><th>Triple</th><th><code>\\|</code></th></tr></thead><tbody>
<tr><td>Triple</td><td><code>\\|</code></td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_19() {
    let original = r##"| Table | Header |
|-------|--------|
| Table | Body   |
|
| Not   | Enough |


| Table | Header |
|-------|--------|
| Table | Body   |
|	
| Not   | Enough |
"##;
    let expected = r##"<table><thead><tr><th>Table</th><th>Header</th></tr></thead><tbody>
<tr><td>Table</td><td>Body</td></tr>
</tbody></table>
<p>|
| Not   | Enough |</p>
<table><thead><tr><th>Table</th><th>Header</th></tr></thead><tbody>
<tr><td>Table</td><td>Body</td></tr>
</tbody></table>
<p>|
| Not   | Enough |</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_20() {
    let original = r##"| Table | Header |
|-------|--------|
|
"##;
    let expected = r##"<table><thead><tr><th>Table</th><th>Header</th></tr></thead><tbody>
</tbody></table>
<p>|</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_21() {
    let original = r##"|
|-------|--------|
| Table | Body   |
"##;
    let expected = r##"<p>|
|-------|--------|
| Table | Body   |</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_22() {
    let original = r##"| Single | [test](first\|second) |
|--|--|

| Double | [test](first\\|second) |
|--|--|

| Triple | [test](first\\\|second) |
|--|--|
"##;
    let expected = r##"<table><thead><tr><th>Single</th><th><a href="first%7Csecond">test</a></th></tr></thead><tbody>
</tbody></table>
<table><thead><tr><th>Double</th><th><a href="first%7Csecond">test</a></th></tr></thead><tbody>
</tbody></table>
<table><thead><tr><th>Triple</th><th><a href="first%5C%7Csecond">test</a></th></tr></thead><tbody>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_23() {
    let original = r##"| Single | [first\|second] |
|--|--|

| Double | [first\\|second] |
|--|--|

| Triple | [first\\\|second] |
|--|--|

[first\|second]: https://rust-lang.org

[first\\|second]: https://docs.rs
"##;
    let expected = r##"<table><thead><tr><th>Single</th><th>[first|second]</th></tr></thead><tbody>
</tbody></table>
<table><thead><tr><th>Double</th><th><a href="https://rust-lang.org">first|second</a></th></tr></thead><tbody>
</tbody></table>
<table><thead><tr><th>Triple</th><th><a href="https://docs.rs">first\|second</a></th></tr></thead><tbody>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_24() {
    let original = r##"Q: Knock knock.
A: Who's there.
Q: Interrupting cow.
A: Interrupting —?
| `Moo\\|ooo` |
|-------------|
| `ooo\\|ooo` |
"##;
    let expected = r##"<p>Q: Knock knock.
A: Who&#39;s there.
Q: Interrupting cow.
A: Interrupting —?</p>
<table><thead><tr><th><code>Moo\|ooo</code></th></tr></thead><tbody>
<tr><td><code>ooo\|ooo</code></td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_25() {
    let original = r##"| ![Moo\\|Moo](image.png) |
|-------------|
| ![Moo\\\|Moo](image.png) |
"##;
    let expected = r##"<table><thead><tr><th><img src="image.png" alt="Moo|Moo" /></th></tr></thead><tbody>
<tr><td><img src="image.png" alt="Moo\|Moo" /></td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_26() {
    let original = r##"| [Moo](https://example.org "Example\\|Link") |
|---------------------------------------------|
| [Moo](https://example.org "Example\\\|Link") |
"##;
    let expected = r##"<table><thead><tr><th><a href="https://example.org" title="Example|Link">Moo</a></th></tr></thead><tbody>
<tr><td><a href="https://example.org" title="Example\|Link">Moo</a></td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_27() {
    let original = r##"moo | moo
----|----
moo | moo
*
"##;
    let expected = r##"<table><thead><tr><th>moo</th><th>moo</th></tr></thead><tbody>
<tr><td>moo</td><td>moo</td></tr>
</tbody></table>
<ul>
<li></li>
</ul>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn table_test_28() {
    let original = r##"moo | moo
----|----
moo | moo
2.
"##;
    let expected = r##"<table><thead><tr><th>moo</th><th>moo</th></tr></thead><tbody>
<tr><td>moo</td><td>moo</td></tr>
</tbody></table>
<ol start="2">
<li></li>
</ol>
"##;

    test_markdown_html(original, expected, false, false, false);
}
