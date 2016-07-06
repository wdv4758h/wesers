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


License
========================================

wesers is licensed under the AGPL License - see the ``LICENSE`` file for details
