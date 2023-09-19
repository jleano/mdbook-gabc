use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Result;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CodeBlockKind::*, Event, Options, Parser, Tag};

pub struct Gregorio;

impl Preprocessor for Gregorio {
    fn name(&self) -> &str {
        "gregorio"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let mut res = None;
        book.for_each_mut(|item: &mut BookItem| {
            if let Some(Err(_)) = res {
                return;
            }

            if let BookItem::Chapter(ref mut chapter) = *item {
                res = Some(Gregorio::add_gregorio(chapter).map(|md| {
                    chapter.content = md;
                }));
            }
        });

        res.unwrap_or(Ok(())).map(|_| book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

fn escape_html(s: &str) -> String {
    let mut output = String::new();
    for c in s.chars() {
        match c {
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '&' => output.push_str("&amp;"),
            _ => output.push(c),
        }
    }
    output
}

fn add_gregorio(content: &str) -> Result<String> {
    //let mut gregorio_content = String::new();
    let mut in_gregorio_block = false;

    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    let mut code_span = 0..0;
    let mut start_new_code_span = true;

    let mut gregorio_blocks = vec![];

    let events = Parser::new_ext(content, opts);

    println!("before loop");
    for (e, span) in events.into_offset_iter() {
        println!("in loop, e={:?}, span={:?}", e, span);
        log::debug!("e={:?}, span={:?}", e, span);
        if let Event::Start(Tag::CodeBlock(Fenced(code))) = e.clone() {
            println!("in fenced1. code={:?}", code);
            //if &*code == "" {
            if code.is_empty() {
                println!("in fenced2");
                in_gregorio_block = false;
                //gregorio_content.clear();
            } else {
                in_gregorio_block = true;
            }
            continue;
        }

        if !in_gregorio_block {
            println!("not in block");
            continue;
        }

        // We're in the code block. The text is what we want.
        // Code blocks can come in multiple text events.
        if let Event::Text(_) = e {
            if start_new_code_span {
                code_span = span;
                start_new_code_span = false;
            } else {
                code_span = code_span.start..span.end;
            }

            continue;
        }

        println!("code_span = {:?}", code_span);
        if let Event::End(Tag::CodeBlock(Fenced(code))) = e {
            if in_gregorio_block {
                assert_eq!(
                    "gregorio", &*code,
                    "After an opening gregorio code block we expect it to close again"
                );
            }
            in_gregorio_block = false;

            let gregorio_content = &content[code_span.clone()];
            let gregorio_content = escape_html(gregorio_content);
            let gregorio_content = gregorio_content.replace("\r\n", "\n");
            let gregorio_code = format!(
                "<pre class=\"chant-container\">{}</pre>\n\n",
                gregorio_content
            );
            println!("code={:?}", gregorio_code);
            gregorio_blocks.push((span, gregorio_code));
            start_new_code_span = true;
        }
    }

    let mut content = content.to_string();
    println!("after loop. content={:?}", content);
    for (span, block) in gregorio_blocks.iter().rev() {
        let pre_content = &content[0..span.start];
        let post_content = &content[span.end..];
        println!(
            "block={:?}, pre_content={:?}, post_content={:?}",
            block,
            pre_content.clone(),
            post_content
        );
        content = format!("{}\n{}{}", pre_content, block, post_content);
    }
    println!("after loop1. content={:?}", content);
    Ok(content)
}

impl Gregorio {
    fn add_gregorio(chapter: &mut Chapter) -> Result<String> {
        add_gregorio(&chapter.content)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::add_gregorio;

    #[test]
    fn adds_gregorio() {
        let content = r#"# Chapter

```gregorio
(f3) EC(ce!fg)CE(f) *(,) ad(fe~)vé(f!gwhf)nit(f) (,)
```

Text
"#;

        let expected = r#"# Chapter


<pre class="chant-container">(f3) EC(ce!fg)CE(f) *(,) ad(fe~)vé(f!gwhf)nit(f) (,)
</pre>



Text
"#;

        assert_eq!(expected, add_gregorio(content).unwrap());
    }

    #[test]
    fn leaves_tables_untouched() {
        // Regression test.
        // Previously we forgot to enable the same markdwon extensions as mdbook itself.

        let content = r#"# Heading

| Head 1 | Head 2 |
|--------|--------|
| Row 1  | Row 2  |
"#;

        let expected = r#"# Heading

| Head 1 | Head 2 |
|--------|--------|
| Row 1  | Row 2  |
"#;

        assert_eq!(expected, add_gregorio(content).unwrap());
    }

    #[test]
    fn leaves_html_untouched() {
        // Regression test.
        // Don't remove important newlines for syntax nested inside HTML

        let content = r#"# Heading

<del>

*foo*

</del>
"#;

        let expected = r#"# Heading

<del>

*foo*

</del>
"#;

        assert_eq!(expected, add_gregorio(content).unwrap());
    }

    #[test]
    fn html_in_list() {
        // Regression test.
        // Don't remove important newlines for syntax nested inside HTML

        let content = r#"# Heading

1. paragraph 1
   ```
   code 1
   ```
2. paragraph 2
"#;

        let expected = r#"# Heading

1. paragraph 1
   ```
   code 1
   ```
2. paragraph 2
"#;

        let ret = add_gregorio(content).unwrap();
        println!("bfore assert. ret={:?}", ret);
        assert_eq!(expected, ret);
        //assert_eq!(expected, add_gregorio(content).unwrap());
    }

    #[test]
    fn escape_in_gregorio_block() {
        //TODO may be able to delete this method.
        let _ = env_logger::try_init();
        let content = r#"
```gregorio
(f3) EC(ce!fg)CE(f) *(,) ad(fe~)vé(f!gwhf)nit(f) (,)
```

hello
"#;

        let expected = r#"

<pre class="chant-container">(f3) EC(ce!fg)CE(f) *(,) ad(fe~)vé(f!gwhf)nit(f) (,)
</pre>



hello
"#;

        assert_eq!(expected, add_gregorio(content).unwrap());
    }

    #[test]
    fn more_backticks() {
        let _ = env_logger::try_init();
        let content = r#"# Chapter

````gregorio
(f3) EC(ce!fg)CE(f) *(,) ad(fe~)vé(f!gwhf)nit(f) (,)
````

Text
"#;

        let expected = r#"# Chapter


<pre class="chant-container">(f3) EC(ce!fg)CE(f) *(,) ad(fe~)vé(f!gwhf)nit(f) (,)
</pre>



Text
"#;

        assert_eq!(expected, add_gregorio(content).unwrap());
    }

    #[test]
    fn crlf_line_endings() {
        let _ = env_logger::try_init();
        let content = "# Chapter\r\n\r\n````gregorio\r\n\r\n(f3) EC(ce!fg)CE(f) *(,)\r\nad(fe~)vé(f!gwhf)nit(f) (,)\r\n````";
        let expected =
            "# Chapter\r\n\r\n\n<pre class=\"chant-container\">\n(f3) EC(ce!fg)CE(f) *(,)\nad(fe~)vé(f!gwhf)nit(f) (,)\n</pre>\n\n";

        assert_eq!(expected, add_gregorio(content).unwrap());
    }

    #[test]
    fn test_leaves_nongregorio_untouched() {
        let content = r#"Chapter\nsample program```python\nprint('output')```\nfinished"#;
        assert_eq!(content, add_gregorio(content).unwrap());
    }
}
