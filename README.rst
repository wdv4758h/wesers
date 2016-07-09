===========================================
wesers - a simple HTTP/HTTPS server in Rust
===========================================

.. contents:: Table of Contents


Installation
========================================

If you want to build from source, ``cargo install wesers`` may not work.
(As it don't look ``Cargo.lock`` currently)


Install with

.. code-block:: sh

    cargo install --git https://github.com/wdv4758h/wesers

or

.. code-block:: sh

    cargo install --git https://github.com/wdv4758h/wesers --tag v0.3.1


If you want to download prebuilt binary,
you can visit `GitHub's release page <https://github.com/wdv4758h/wesers/releases>`_


Usage
========================================

.. code-block:: sh

    $ wesers --help
    wesers 0.3.1
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


.. code-block:: sh

    $ wesers
    Simple HTTP Server running on http://127.0.0.1:8000/
    GET http://127.0.0.1:8000/ -> 200 OK (0.110684 ms)
    GET http://127.0.0.1:8000/target -> 200 OK (0.043674 ms)
    GET http://127.0.0.1:8000/target/release -> 200 OK (0.082394 ms)
    GET http://127.0.0.1:8000/target/release/wesers -> 200 OK (5.063098 ms)


.. code-block:: sh

    $ wesers --ip 127.0.0.1 --port 8080 --template ./custom.mustache
    Simple HTTP Server running on http://127.0.0.1:8080/


.. code-block:: sh

    $ wesers --https --cert mycert.pem --key mykey.pem
    Simple HTTP Server running on https://127.0.0.1:8000/


Binary Size
========================================

x86_64 Linux:

+----------+---------+------------+--------------+-----------+
| Filename | Version | Stripped ? | Size (Bytes) | Size (MB) |
+----------+---------+------------+--------------+-----------+
| wesers   | v0.3.1  | No         | 3760376      | 3.6M      |
+----------+---------+------------+--------------+-----------+
| wesers   | v0.3.1  | Yes        | 2929208      | 2.8M      |
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
* log client IP (waiting PR https://github.com/iron/logger/pull/76)
* unicode url support (issue https://github.com/iron/staticfile/issues/76)


v0.4.0 (Plan)
------------------------------

Features
++++++++++++++++++++


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
