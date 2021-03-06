# My Pipe

**Usage**

> Following CLI are here to try to **fix a problem with WSL error permission.**

> However, **errors** could be **encountered...**

```shell
sudo cargo build --target-dir mypype_test
```

```shell
. /mypype_test/debug/inandout --in fortune --out cowsay
```

> Also, you **could try** to enter the classique command in your prefered terminal.

```shell
$ mypipe --in fortune --out cowsay
```

-----------------------------------------------------------------------------

You have to recode a small pipe-like program, working like this:

```shell
$ mypipe --in fortune --out cowsay
```

```
 _______________________________________
/ Q: What's tiny and yellow and very,   \
| very, dangerous? A: A canary with the |
\ super-user password.                  /
 ---------------------------------------
          \   ^__^
           \  (oo)\_______
              (__)\       )\/\
                  ||----w |
                  ||     ||
```

You can use <https://clap.rs> to parse the command-line arguments, and also follow the guide <https://rust-lang-nursery.github.io/cli-wg/>
