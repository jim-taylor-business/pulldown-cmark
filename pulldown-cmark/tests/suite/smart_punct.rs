// This file is auto-generated by the build script
// Please, do not modify it manually

use super::test_markdown_html;

#[test]
fn smart_punct_test_1() {
    let original = r##""Hello," said the spider.
"'Shelob' is my name."
"##;
    let expected = r##"<p>“Hello,” said the spider.
“‘Shelob’ is my name.”</p>
"##;

    test_markdown_html(original, expected, true, false, false, false, false, false);
}

#[test]
fn smart_punct_test_2() {
    let original = r##"'A', 'B', and 'C' are letters.
"##;
    let expected = r##"<p>‘A’, ‘B’, and ‘C’ are letters.</p>
"##;

    test_markdown_html(original, expected, true, false, false, false, false, false);
}

#[test]
fn smart_punct_test_3() {
    let original = r##"'Oak,' 'elm,' and 'beech' are names of trees.
So is 'pine.'
"##;
    let expected = r##"<p>‘Oak,’ ‘elm,’ and ‘beech’ are names of trees.
So is ‘pine.’</p>
"##;

    test_markdown_html(original, expected, true, false, false, false, false, false);
}

#[test]
fn smart_punct_test_4() {
    let original = r##"'He said, "I want to go."'
"##;
    let expected = r##"<p>‘He said, “I want to go.”’</p>
"##;

    test_markdown_html(original, expected, true, false, false, false, false, false);
}

#[test]
fn smart_punct_test_5() {
    let original = r##"Were you alive in the 70's?
"##;
    let expected = r##"<p>Were you alive in the 70’s?</p>
"##;

    test_markdown_html(original, expected, true, false, false, false, false, false);
}

#[test]
fn smart_punct_test_6() {
    let original = r##"Here is some quoted '`code`' and a "[quoted link](url)".
"##;
    let expected = r##"<p>Here is some quoted ‘<code>code</code>’ and a “<a href="url">quoted link</a>”.</p>
"##;

    test_markdown_html(original, expected, true, false, false, false, false, false);
}

#[test]
fn smart_punct_test_7() {
    let original = r##"'tis the season to be 'jolly'
"##;
    let expected = r##"<p>’tis the season to be ‘jolly’</p>
"##;

    test_markdown_html(original, expected, true, false, false, false, false, false);
}

#[test]
fn smart_punct_test_8() {
    let original = r##"'We'll use Jane's boat and John's truck,' Jenna said.
"##;
    let expected = r##"<p>‘We’ll use Jane’s boat and John’s truck,’ Jenna said.</p>
"##;

    test_markdown_html(original, expected, true, false, false, false, false, false);
}

#[test]
fn smart_punct_test_9() {
    let original = r##""A paragraph with no closing quote.

"Second paragraph by same speaker, in fiction."
"##;
    let expected = r##"<p>“A paragraph with no closing quote.</p>
<p>“Second paragraph by same speaker, in fiction.”</p>
"##;

    test_markdown_html(original, expected, true, false, false, false, false, false);
}

#[test]
fn smart_punct_test_10() {
    let original = r##"[a]'s b'
"##;
    let expected = r##"<p>[a]’s b’</p>
"##;

    test_markdown_html(original, expected, true, false, false, false, false, false);
}

#[test]
fn smart_punct_test_11() {
    let original = r##"\"This is not smart.\"
This isn\'t either.
5\'8\"
"##;
    let expected = r##"<p>"This is not smart."
This isn't either.
5'8"</p>
"##;

    test_markdown_html(original, expected, true, false, false, false, false, false);
}

#[test]
fn smart_punct_test_12() {
    let original = r##"Some dashes:  em---em
en--en
em --- em
en -- en
2--3
"##;
    let expected = r##"<p>Some dashes:  em—em
en–en
em — em
en – en
2–3</p>
"##;

    test_markdown_html(original, expected, true, false, false, false, false, false);
}

#[test]
fn smart_punct_test_13() {
    let original = r##"one-
two--
three---
four----
five-----
six------
seven-------
eight--------
nine---------
thirteen-------------.
"##;
    let expected = r##"<p>one-
two–
three—
four––
five—–
six——
seven—––
eight––––
nine———
thirteen———––.</p>
"##;

    test_markdown_html(original, expected, true, false, false, false, false, false);
}

#[test]
fn smart_punct_test_14() {
    let original = r##"Escaped hyphens: \-- \-\-\-.
"##;
    let expected = r##"<p>Escaped hyphens: -- ---.</p>
"##;

    test_markdown_html(original, expected, true, false, false, false, false, false);
}

#[test]
fn smart_punct_test_15() {
    let original = r##"Ellipses...and...and....
"##;
    let expected = r##"<p>Ellipses…and…and….</p>
"##;

    test_markdown_html(original, expected, true, false, false, false, false, false);
}

#[test]
fn smart_punct_test_16() {
    let original = r##"No ellipses\.\.\.
"##;
    let expected = r##"<p>No ellipses...</p>
"##;

    test_markdown_html(original, expected, true, false, false, false, false, false);
}
