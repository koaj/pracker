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