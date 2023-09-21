# mdbook-gabc

A preprocessor for [mdbook][] to add gabc support.

[mdbook]: https://github.com/rust-lang-nursery/mdBook
[gabc]: https://gregorio-project.github.io/gabc/

It turns this:

~~~
```gabc
(f3) EC(ce!fg)CE(f) *(,) ad(fe~)vé(f!gwhf)nit(f) (,)
```
~~~

into this:

![Simple Chant](simple-chant.png)

in your book.

## Installation

If you want to use only this preprocessor, install the tool:

```
cargo install mdbook-gabc
```

Then let `mdbook-gabc` add the required files and configuration:

```
mdbook-gabc install path/to/your/book
```


This will add the following configuration to your `book.toml`:

```toml
[preprocessor.gabc]
command = "mdbook-gabc"

[output.html]
additional-js = ["exsurge.min.js", "exsurge-init.js"]
```

It will skip any unnecessary changes and detect if `mdbook-gabc` was already configured.

Additionally it copies the files `exsurge.min.js` and  `exsurge-init.js` into your book's directory.
You find these files in the [`src/bin/assets`](src/bin/assets) directory.

Finally, build your book:

```
mdbook path/to/book
```

## License

MPL. See [LICENSE](LICENSE).  
Copyright (c) 2023 Joseph Leano <josephleano@gmail.com>

Exsurge is [MIT licensed](https://github.com/frmatthew/exsurge/blob/master/LICENSE).
The bundled assets (`exsurge.min.js`) are MIT licensed.
