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


Changelog
========================================

v0.3.0 (Plan)
------------------------------

* custom HTML template support
* can run as CGI server


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

I've only test on my x86_64 Linux.
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
