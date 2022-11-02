
---

To build the project:
```shell
$ cargo build
$ sudo ./target/debug/connect
received event, destination: 1.0.0.127, source: 1.0.0.127
received event, destination: 1.0.0.127, source: 1.0.0.127
received event, destination: 4.0.18.172, source: 4.0.18.172
received event, destination: 1.0.0.127, source: 1.0.0.127
received event, destination: 1.0.0.127, source: 1.0.0.127
^C
```

---

To generate an updated `vmlinux.h`:
```shell
$ bpftool btf dump file /sys/kernel/btf/vmlinux format c > .src/bpf/vmlinux.h
```

BTF might also be found at `/boot/vmlinux-$(uname -r)`, depending on which
linux distribution you run.

You can see if your kernel is compiled with BTF by running:
```shell
$ zgrep CONFIG_DEBUG_INFO_BTF /proc/config.gz
```
