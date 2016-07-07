========================================
wesers - a simple HTTP server in Rust
========================================

Usage
========================================

.. code-block:: sh

    $ wesers --help
    wesers 0.1.0
    Chiu-Hsiang Hsu <wdv4758h@gmail.com>
    a simple HTTP server in Rust

    USAGE:
        wesers [OPTIONS]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
            --ip <ip>        binding IP [default: 127.0.0.1]
            --port <port>    binding port [default: 8000]


.. code-block:: sh

    $ wesers
    Simple HTTP Server running on http://127.0.0.1:8000/
    GET http://127.0.0.1:8000/ -> 200 OK (0.110684 ms)
    GET http://127.0.0.1:8000/target -> 200 OK (0.043674 ms)
    GET http://127.0.0.1:8000/target/release -> 200 OK (0.082394 ms)
    GET http://127.0.0.1:8000/target/release/wesers -> 200 OK (5.063098 ms)


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

v0.4.0 (Plan)
------------------------------

Features
++++++++++++++++++++

* support limit request times
* code refactoring to improve performance


v0.3.0 (Plan)
------------------------------

Features
++++++++++++++++++++

* custom HTML template support
* can run as CGI server
* HTTPS support


v0.2.0 (Plan)
------------------------------

Features
++++++++++++++++++++

* Bash completion by clap
* detect index.html
* log client IP


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



Special Thanks
========================================

* `rust-everywhere <https://github.com/japaric/rust-everywhere/>`_ for CI integration
* `clap-rs <https://github.com/kbknapp/clap-rs>`_ for arguments parsing
* `iron <https://github.com/iron/iron>`_ for Rust web framework
* `Rust Team <https://www.rust-lang.org/team.html>`_



License
========================================

wesers is licensed under the AGPL License - see the ``LICENSE`` file for details
