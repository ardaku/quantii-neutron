# Quantii Neutron

Implementation of Quantii suite on the neutron kernel.

## Wasm

Requires [krustlet](https://github.com/krustlet/krustlet) built and kubernetes. To test it out, use a docker container.

On Neutron, `krustlet` is built in `/sys/bin` when the `wasm` option is checked in the installer phase 2.

For userspace applications, `wasmer` is used. It should be installed in `/sys/bin`. All apps that can be compiled to `wasm32-wasi` can run with `wasmer`.

## OCI

Open Container Interface. Very cool containerisation framework to build, load and manage containers on distributed systems. Can be used with krustlet  kubernetes KCI.

## Apps

The quantii suite includes things like a window manager, desktop environment, and misc executable tools and libraries.

To use them, build for `wasm32-wasi` as usual. Or if possible, `riscv64gc-neutron`.
