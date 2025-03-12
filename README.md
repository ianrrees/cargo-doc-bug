# Demonstrate `cargo doc` bug

1. Run `cargo doc` in the `myapp` directory
2. There are two different versions of `mylib`, the docs generated for `myapp`
should contain links in to both versions of `mylib`
3. Actually, the docs only link to the newer version of `mylib`