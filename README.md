# simple_on_shutdown
This Rust library consists of a convenient macro to specify on shutdown callbacks.
This is useful for all runtimes you do not have control over.

The generated main() function of an "actix" web server is a good example. With the 
exported macro `on_shutdown` you can easily specify code, that should run during program
termination.

IMPORTANT: Use this on the top level of your main() or whatever your current runtimes main
function is! The code gets executed when the context it lives in gets dropped.
This can be called multiple times (at least with stable Rust 1.48.0) without problem.