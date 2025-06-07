Given the following tree of files:

```shell
drwxr-xr-x - sam wheel  7 Jun 15:32 dest
drwxr-xr-x - sam wheel  7 Jun 14:35 src
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[85].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[468].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[547].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[758].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[1343].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[3462].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[6532].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[24546].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[654345].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── pref[11]suf.txt
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── pref[12]suf.txt
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── pref[14]suf.txt
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── pref[52]suf.txt
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── pref[74]suf.txt
.rw-r--r-- 0 sam wheel  7 Jun 14:57 └── pref[458]suf.txt
```

Running

```
mvp src/ dest/ --id-regex '\[(\d\d+)\]'
```

<details>

<summary>produces this output</summary>

```
drwxr-xr-x - sam wheel  7 Jun 15:41 dest
drwxr-xr-x - sam wheel  7 Jun 15:41 ├── 1
drwxr-xr-x - sam wheel  7 Jun 15:41 │   ├── 1
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │   │   └── pref[11]suf.txt
drwxr-xr-x - sam wheel  7 Jun 15:41 │   ├── 2
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │   │   └── pref[12]suf.txt
drwxr-xr-x - sam wheel  7 Jun 15:41 │   ├── 3
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │   │   └── 'other prefix[1343].txt'
drwxr-xr-x - sam wheel  7 Jun 15:41 │   └── 4
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │       └── pref[14]suf.txt
drwxr-xr-x - sam wheel  7 Jun 15:41 ├── 2
drwxr-xr-x - sam wheel  7 Jun 15:41 │   └── 4
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │       └── 'other prefix[24546].txt'
drwxr-xr-x - sam wheel  7 Jun 15:41 ├── 3
drwxr-xr-x - sam wheel  7 Jun 15:41 │   └── 4
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │       └── 'other prefix[3462].txt'
drwxr-xr-x - sam wheel  7 Jun 15:41 ├── 4
drwxr-xr-x - sam wheel  7 Jun 15:41 │   ├── 5
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │   │   └── pref[458]suf.txt
drwxr-xr-x - sam wheel  7 Jun 15:41 │   └── 6
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │       └── 'other prefix[468].txt'
drwxr-xr-x - sam wheel  7 Jun 15:41 ├── 5
drwxr-xr-x - sam wheel  7 Jun 15:41 │   ├── 2
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │   │   └── pref[52]suf.txt
drwxr-xr-x - sam wheel  7 Jun 15:41 │   └── 4
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │       └── 'other prefix[547].txt'
drwxr-xr-x - sam wheel  7 Jun 15:41 ├── 6
drwxr-xr-x - sam wheel  7 Jun 15:41 │   └── 5
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │       ├── 'other prefix[6532].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │       └── 'other prefix[654345].txt'
drwxr-xr-x - sam wheel  7 Jun 15:41 ├── 7
drwxr-xr-x - sam wheel  7 Jun 15:41 │   ├── 4
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │   │   └── pref[74]suf.txt
drwxr-xr-x - sam wheel  7 Jun 15:41 │   └── 5
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │       └── 'other prefix[758].txt'
drwxr-xr-x - sam wheel  7 Jun 15:41 └── 8
drwxr-xr-x - sam wheel  7 Jun 15:41     └── 5
.rw-r--r-- 0 sam wheel  7 Jun 14:57         └── 'other prefix[85].txt'
drwxr-xr-x - sam wheel  7 Jun 15:41 src
```

</details>

Running

```
cpp src/ dest/ --id-regex '\[(\d\d+)\]'
```

<details>

<summary>produces this output</summary>

```
drwxr-xr-x - sam wheel  7 Jun 15:38 dest
drwxr-xr-x - sam wheel  7 Jun 15:38 ├── 1
drwxr-xr-x - sam wheel  7 Jun 15:38 │   ├── 1
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │   │   └── pref[11]suf.txt
drwxr-xr-x - sam wheel  7 Jun 15:38 │   ├── 2
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │   │   └── pref[12]suf.txt
drwxr-xr-x - sam wheel  7 Jun 15:38 │   ├── 3
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │   │   └── 'other prefix[1343].txt'
drwxr-xr-x - sam wheel  7 Jun 15:38 │   └── 4
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │       └── pref[14]suf.txt
drwxr-xr-x - sam wheel  7 Jun 15:38 ├── 2
drwxr-xr-x - sam wheel  7 Jun 15:38 │   └── 4
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │       └── 'other prefix[24546].txt'
drwxr-xr-x - sam wheel  7 Jun 15:38 ├── 3
drwxr-xr-x - sam wheel  7 Jun 15:38 │   └── 4
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │       └── 'other prefix[3462].txt'
drwxr-xr-x - sam wheel  7 Jun 15:38 ├── 4
drwxr-xr-x - sam wheel  7 Jun 15:38 │   ├── 5
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │   │   └── pref[458]suf.txt
drwxr-xr-x - sam wheel  7 Jun 15:38 │   └── 6
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │       └── 'other prefix[468].txt'
drwxr-xr-x - sam wheel  7 Jun 15:38 ├── 5
drwxr-xr-x - sam wheel  7 Jun 15:38 │   ├── 2
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │   │   └── pref[52]suf.txt
drwxr-xr-x - sam wheel  7 Jun 15:38 │   └── 4
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │       └── 'other prefix[547].txt'
drwxr-xr-x - sam wheel  7 Jun 15:38 ├── 6
drwxr-xr-x - sam wheel  7 Jun 15:38 │   └── 5
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │       ├── 'other prefix[6532].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │       └── 'other prefix[654345].txt'
drwxr-xr-x - sam wheel  7 Jun 15:38 ├── 7
drwxr-xr-x - sam wheel  7 Jun 15:38 │   ├── 4
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │   │   └── pref[74]suf.txt
drwxr-xr-x - sam wheel  7 Jun 15:38 │   └── 5
.rw-r--r-- 0 sam wheel  7 Jun 14:57 │       └── 'other prefix[758].txt'
drwxr-xr-x - sam wheel  7 Jun 15:38 └── 8
drwxr-xr-x - sam wheel  7 Jun 15:38     └── 5
.rw-r--r-- 0 sam wheel  7 Jun 14:57         └── 'other prefix[85].txt'
drwxr-xr-x - sam wheel  7 Jun 14:35 src
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[85].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[468].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[547].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[758].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[1343].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[3462].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[6532].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[24546].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[654345].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── pref[11]suf.txt
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── pref[12]suf.txt
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── pref[14]suf.txt
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── pref[52]suf.txt
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── pref[74]suf.txt
.rw-r--r-- 0 sam wheel  7 Jun 14:57 └── pref[458]suf.txt
```

</details>

and running

```
lnp src/ dest/ --id-regex '\[(\d\d+)\]'
```

<details>

<summary>produces this output</summary>

```
drwxr-xr-x - sam wheel  7 Jun 15:40 dest
drwxr-xr-x - sam wheel  7 Jun 15:40 ├── 1
drwxr-xr-x - sam wheel  7 Jun 15:40 │   ├── 1
lrwxr-xr-x - sam wheel  7 Jun 15:40 │   │   └── pref[11]suf.txt -> /private/tmp/src/pref[11]suf.txt
drwxr-xr-x - sam wheel  7 Jun 15:40 │   ├── 2
lrwxr-xr-x - sam wheel  7 Jun 15:40 │   │   └── pref[12]suf.txt -> /private/tmp/src/pref[12]suf.txt
drwxr-xr-x - sam wheel  7 Jun 15:40 │   ├── 3
lrwxr-xr-x - sam wheel  7 Jun 15:40 │   │   └── 'other prefix[1343].txt' -> /private/tmp/src/'other prefix[1343].txt'
drwxr-xr-x - sam wheel  7 Jun 15:40 │   └── 4
lrwxr-xr-x - sam wheel  7 Jun 15:40 │       └── pref[14]suf.txt -> /private/tmp/src/pref[14]suf.txt
drwxr-xr-x - sam wheel  7 Jun 15:40 ├── 2
drwxr-xr-x - sam wheel  7 Jun 15:40 │   └── 4
lrwxr-xr-x - sam wheel  7 Jun 15:40 │       └── 'other prefix[24546].txt' -> /private/tmp/src/'other prefix[24546].txt'
drwxr-xr-x - sam wheel  7 Jun 15:40 ├── 3
drwxr-xr-x - sam wheel  7 Jun 15:40 │   └── 4
lrwxr-xr-x - sam wheel  7 Jun 15:40 │       └── 'other prefix[3462].txt' -> /private/tmp/src/'other prefix[3462].txt'
drwxr-xr-x - sam wheel  7 Jun 15:40 ├── 4
drwxr-xr-x - sam wheel  7 Jun 15:40 │   ├── 5
lrwxr-xr-x - sam wheel  7 Jun 15:40 │   │   └── pref[458]suf.txt -> /private/tmp/src/pref[458]suf.txt
drwxr-xr-x - sam wheel  7 Jun 15:40 │   └── 6
lrwxr-xr-x - sam wheel  7 Jun 15:40 │       └── 'other prefix[468].txt' -> /private/tmp/src/'other prefix[468].txt'
drwxr-xr-x - sam wheel  7 Jun 15:40 ├── 5
drwxr-xr-x - sam wheel  7 Jun 15:40 │   ├── 2
lrwxr-xr-x - sam wheel  7 Jun 15:40 │   │   └── pref[52]suf.txt -> /private/tmp/src/pref[52]suf.txt
drwxr-xr-x - sam wheel  7 Jun 15:40 │   └── 4
lrwxr-xr-x - sam wheel  7 Jun 15:40 │       └── 'other prefix[547].txt' -> /private/tmp/src/'other prefix[547].txt'
drwxr-xr-x - sam wheel  7 Jun 15:40 ├── 6
drwxr-xr-x - sam wheel  7 Jun 15:40 │   └── 5
lrwxr-xr-x - sam wheel  7 Jun 15:40 │       ├── 'other prefix[6532].txt' -> /private/tmp/src/'other prefix[6532].txt'
lrwxr-xr-x - sam wheel  7 Jun 15:40 │       └── 'other prefix[654345].txt' -> /private/tmp/src/'other prefix[654345].txt'
drwxr-xr-x - sam wheel  7 Jun 15:40 ├── 7
drwxr-xr-x - sam wheel  7 Jun 15:40 │   ├── 4
lrwxr-xr-x - sam wheel  7 Jun 15:40 │   │   └── pref[74]suf.txt -> /private/tmp/src/pref[74]suf.txt
drwxr-xr-x - sam wheel  7 Jun 15:40 │   └── 5
lrwxr-xr-x - sam wheel  7 Jun 15:40 │       └── 'other prefix[758].txt' -> /private/tmp/src/'other prefix[758].txt'
drwxr-xr-x - sam wheel  7 Jun 15:40 └── 8
drwxr-xr-x - sam wheel  7 Jun 15:40     └── 5
lrwxr-xr-x - sam wheel  7 Jun 15:40         └── 'other prefix[85].txt' -> /private/tmp/src/'other prefix[85].txt'
drwxr-xr-x - sam wheel  7 Jun 14:35 src
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[85].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[468].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[547].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[758].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[1343].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[3462].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[6532].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[24546].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── 'other prefix[654345].txt'
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── pref[11]suf.txt
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── pref[12]suf.txt
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── pref[14]suf.txt
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── pref[52]suf.txt
.rw-r--r-- 0 sam wheel  7 Jun 14:57 ├── pref[74]suf.txt
.rw-r--r-- 0 sam wheel  7 Jun 14:57 └── pref[458]suf.txt
```

</details>
