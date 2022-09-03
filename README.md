How does it work?
-----------------
Pracker working based on the wordpress rest api `wp-json/wp/v2/posts/`. Please make sure that your wordpress site has enabled the rest api. You can add `wp-json/wp/v2/posts/` to the end of your wordpress site url to check if it is enabled. If you see a json response, then it is enabled. If you see a 404 error, then it is not enabled. You can enable it by installing the [WP REST API](https://wordpress.org/plugins/rest-api/) plugin.


How to run wordpress content crawler:


```bash
cargo run -- -s https://wordpres-site-example.com/
```



Output:
```json
Post {
    title: "Post title",
    content: "<b>Post content</b>",
    url: "wordpres-site-example.com/?p=1",
    id: 1
}
```



**Plain text**:

```bash
cargo run -- -s https://wordpres-site-example.com/ -p
```
Output:
```json
Post {
    title: "Post title",
    content: "Post content",
    url: "wordpres-site-example.com/?p=1",
    id: 1
}
```


##Store the output into the DB:

Create a database in postgresql:
```sql
CREATE DATABASE pracker;
```

Run migration:
```bash
diesel migration run
```


```bash
mv env.sample .env
```


If you are looking for more options, please run `cargo run -- --help` to see all the options.