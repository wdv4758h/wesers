===========================================
wesers - a simple HTTP/HTTPS server in Rust
===========================================

.. contents:: Table of Contents


Installation
========================================

(You need to compile with nightly, now)

If you want to build from source, make sure you are using nightly toolchain !


Install with

.. code-block:: sh

    cargo install --git https://github.com/wdv4758h/wesers

or

.. code-block:: sh

    cargo install --git https://github.com/wdv4758h/wesers --tag v0.4.0


If you want to download prebuilt binary,
you can visit `GitHub's release page <https://github.com/wdv4758h/wesers/releases>`_


Usage
========================================

.. code-block:: sh

    $ wesers --help
    wesers 0.4.0
    Chiu-Hsiang Hsu <wdv4758h@gmail.com>
    a simple HTTP/HTTPS server in Rust

    USAGE:
        wesers [FLAGS] [OPTIONS]

    FLAGS:
        -h, --help       Prints help information
            --https      use HTTPS instead of HTTP
        -V, --version    Prints version information

    OPTIONS:
            --cert <cert>            SSL certificate file (needed for HTTPS)
            --index <index>          auto detect index.html [default: true]
            --ip <ip>                binding IP [default: 127.0.0.1]
            --key <key>              SSL key file (needed for HTTPS)
            --port <port>            binding port [default: 8000]
            --root <root>            root directory [default: .]
            --template <template>    HTML template path


run :

.. code-block:: sh

    $ wesers
    Simple HTTP Server running on http://127.0.0.1:8000/
    127.0.0.1:54400 GET http://127.0.0.1:8000/ -> 200 OK (0.268528 ms)


run with custom address and HTML template :

.. code-block:: sh

    $ wesers --ip 127.0.0.1 --port 8080 --template ./custom.mustache
    Simple HTTP Server running on http://127.0.0.1:8080/


run in HTTPS :

.. code-block:: sh

    $ wesers --https --cert mycert.pem --key mykey.pem
    Simple HTTP Server running on https://127.0.0.1:8000/


Binary Size
========================================

x86_64 Linux:

+----------+---------+------------+--------------+-----------+
| Filename | Version | Stripped ? | Size (Bytes) | Size (MB) |
+----------+---------+------------+--------------+-----------+
| wesers   | v0.4.0  | No         | 3573904      | 3.5M      |
+----------+---------+------------+--------------+-----------+
| wesers   | v0.4.0  | Yes        | 2769832      | 2.7M      |
+----------+---------+------------+--------------+-----------+



Benchmarks
========================================

This is a simple load testing by `Herd <https://github.com/imjacobclark/Herd>`_
(Just a simple test, not indicate a lot)

.. code-block:: sh

    # Running both herd and wesers on Intel Core i5-2400 with 4GB RAM
    $ ./target/release/herd -t 4 -r 1000 http://127.0.0.1:8000

    ...
    Spawning virtual user 1
    ...
    Spawning virtual user 3
    ...
    I made a total of 4000 requests, the mean response time was: 0.00042591915919911117 seconds.



Changelog
========================================

Not Implemented Yet (Plan)
------------------------------

* more files information (type, size, last modified time)
* more documentation
* QR code support
* optional userdir
* support limit request times
* RESTful API for files and directories
* can run as CGI server
* template live reload
* handle POST for upload files
* Android support
* more template engine support
* fix everything discover by linter (e.g. clippy)
* code refactoring to improve performance
* reduce binary size
* unicode url support (issue https://github.com/iron/staticfile/issues/76)


v0.4.1
------------------------------

Fix
++++++++++++++++++++

* HTTPS build


v0.4.0 (2016-08-03)
------------------------------

Features
++++++++++++++++++++

* log client IP
* update all dependencies


v0.3.1 (2016-07-09)
------------------------------

Fix
++++++++++++++++++++

* make HTTPS optional (because of the dependency of OpenSSL)


v0.3.0 (2016-07-08)
------------------------------

Features
++++++++++++++++++++

* custom HTML template support (with `mustache <https://mustache.github.io/>`_ )
* custom root directory support
* HTTPS support (with OpenSSL)


v0.2.0 (2016-07-08)
------------------------------

Features
++++++++++++++++++++

* Bash completion by clap
* optional index.html detection
* handle 404 better


v0.1.0 (2016-07-07)
------------------------------

Features
++++++++++++++++++++

* serve static files
* list files in directory



Notice
========================================

I've only tested on my x86_64 Linux.
Other platforms are built by CI.
If they don't work properly, please tell me.



Developement
========================================

Making Release
------------------------------

1. update version in ``src/arguments.yml``
2. update version in ``Cargo.toml``
3. update version in ``Cargo.lock``
4. add git tag


Dependency of Shared Libraries
------------------------------

x86_64, Linux, no HTTPS

.. code-block:: sh

    $ ldd ./target/release/wesers
            linux-vdso.so.1 (0x00007fff05f4c000)
            libdl.so.2 => /usr/lib/libdl.so.2 (0x00007f1531e71000)
            libpthread.so.0 => /usr/lib/libpthread.so.0 (0x00007f1531c54000)
            libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x00007f1531a3e000)
            libc.so.6 => /usr/lib/libc.so.6 (0x00007f153169d000)
            /lib64/ld-linux-x86-64.so.2 (0x00007f1532075000)


x86_64, Linux, with HTTPS

.. code-block:: sh

    $ ldd ./target/release/wesers
            linux-vdso.so.1 (0x00007fffdbe85000)
            libssl.so.1.0.0 => /usr/lib/libssl.so.1.0.0 (0x00007f168b810000)
            libcrypto.so.1.0.0 => /usr/lib/libcrypto.so.1.0.0 (0x00007f168b399000)
            libdl.so.2 => /usr/lib/libdl.so.2 (0x00007f168b195000)
            libpthread.so.0 => /usr/lib/libpthread.so.0 (0x00007f168af78000)
            libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x00007f168ad62000)
            libc.so.6 => /usr/lib/libc.so.6 (0x00007f168a9c1000)
            /lib64/ld-linux-x86-64.so.2 (0x00007f168ba81000)


x86_64, Linux, musl, no HTTPS

.. code-block:: sh

    $ ldd ./target/x86_64-unknown-linux-musl/release/wesers
            not a dynamic executable


x86_64, Linux, musl, with HTTPS

.. code-block:: sh

    $ ldd ./target/x86_64-unknown-linux-musl/release/wesers
            linux-vdso.so.1 (0x00007ffc55496000)
            libssl.so.1.0.0 => /usr/lib/libssl.so.1.0.0 (0x00007f69cb9c8000)
            libcrypto.so.1.0.0 => /usr/lib/libcrypto.so.1.0.0 (0x00007f69cb551000)
            libc.so.6 => /usr/lib/libc.so.6 (0x00007f69cb1b0000)
            libdl.so.2 => /usr/lib/libdl.so.2 (0x00007f69cafac000)
            /lib/ld64.so.1 (0x00007f69cbc39000)



Special Thanks
========================================

* `rust-everywhere <https://github.com/japaric/rust-everywhere/>`_ for CI integration
* `clap-rs <https://github.com/kbknapp/clap-rs>`_ for arguments parsing
* `iron <https://github.com/iron/iron>`_ for Rust web framework
* `mustache <https://github.com/nickel-org/rust-mustache>`_ for HTML template
* `Rust Team <https://www.rust-lang.org/team.html>`_
* and every project I've used



License
========================================

wesers is licensed under the AGPL License - see the ``LICENSE`` file for details
