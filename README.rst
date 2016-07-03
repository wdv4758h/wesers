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
	GET http://127.0.0.1:8000/ -> 404 Not Found (0.031623 ms)
	GET http://127.0.0.1:8000/MYFILE -> 200 OK (4.916772 ms)


License
========================================

wesers is licensed under the AGPL License - see the ``LICENSE`` file for details
