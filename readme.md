# Description

This repo is inspired by [actix-sockets](https://github.com/antholeole/actix-sockets).

# Run

Server:
```
cargo run
```

Client 1:
```
$ npx wscat -c 127.0.0.1:8080/c05554ae-b4ee-4976-ac05-97aaf3c98a23

Connected (press CTRL+C to quit)
< your id is be593422-6da6-47cf-bdac-a31c9906b734
< 42079b77-da24-4e86-ad6a-ac9bf768ef5b just joined!
< 1
> 2
< 2
```

Client 2:
```
$ npx wscat -c 127.0.0.1:8080/c05554ae-b4ee-4976-ac05-97aaf3c98a23

Connected (press CTRL+C to quit)
< your id is 42079b77-da24-4e86-ad6a-ac9bf768ef5b
> 1
< 1
< 2
```
