# Bulk email signature generator

Bulk email signature generator is a tool to generate your HTML signatures from your CSV file and a HTML template file.

This tool supports templating provided by [TinyTemplate](https://github.com/bheisler/TinyTemplate).  

You can use custom CSV columns as well.

## Build

```
cargo build
```

## Test

```
cargo test
```

## CSV

The CSV file must contains the column `file_name` in order to generate the HTML files.  
You can use any columns you want. 

See [contacts.csv](input_example/contacts.csv)

By default the file must be named `contacts.csv`.  
To use a different file name use `bulk-email-signature-generator --csv my_file.csv`

## Template

The HTML template file must reference the same columns as your CSV file.  
If your CSV contains the column `mobile_phone`, you can use it as such in your HTML file: `{ contact.mobile_phone }`

You can use conditionnal statement in your HTML template 

```html
{{- if contact.mobile_phone -}}
<p> { contact.mobile_phone } </p>
{{- endif -}}
```

See [template.html](input_example/template.html)

More info on [TinyTemplate](https://github.com/bheisler/TinyTemplate)

## Run

To generate the HTML signature files you can use

```
cargo run

cargo run -- --csv "my_file.csv" --template "my_template.html" --output "my_folder"
```

## License

Bulk email signature generator is distributed under the terms of both the MIT license
and the Apache License (Version 2.0)

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
