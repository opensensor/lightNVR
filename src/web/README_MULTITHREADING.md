# Mongoose Multithreading Support

This implementation adds multithreading support to the Mongoose web server, allowing it to handle multiple requests in parallel. This is particularly useful for long-running operations that would otherwise block the main event loop.

## How It Works

The implementation follows the pattern described in the Mongoose multithreading example:

1. When a request is received, the server can choose to handle it in a separate thread
2. The thread processes the request asynchronously
3. When the thread is done, it sends a wakeup event back to the main event loop
4. The main event loop then sends the response to the client

This approach allows the server to handle multiple requests in parallel without blocking the main event loop.

## Files

- `include/web/mongoose_server_multithreading.h`: Header file with function declarations
- `src/web/mongoose_server_multithreading.c`: Implementation of the multithreading functionality
- `src/web/test_multithreading.c`: A simple test program that demonstrates the multithreading functionality

## Integration with Mongoose Server

The multithreading functionality is integrated with the Mongoose server in `src/web/mongoose_server.c`. The server now handles the `MG_EV_WAKEUP` event, which is sent by worker threads when they complete their processing.

## Building and Running the Test Program

A simple Makefile is provided to build the test program:

```bash
cd src/web
make -f Makefile.test
./test_multithreading
```

The test program starts a web server on port 8000 with two endpoints:

- `/fast`: Handled in the main thread, responds immediately
- Any other path: Handled in a worker thread, responds after a 2-second delay

You can test it with curl:

```bash
# Fast path (main thread)
curl http://localhost:8000/fast

# Slow path (worker thread)
curl http://localhost:8000/slow
```

## Using the Multithreading Functionality

To use the multithreading functionality in your own code:

1. Include the header file:
   ```c
   #include "web/mongoose_server_multithreading.h"
   ```

2. Initialize the wakeup functionality in your Mongoose manager:
   ```c
   mg_wakeup_init(mgr);
   ```

3. Use the `mg_handle_request_with_threading` function to handle requests in a separate thread:
   ```c
   if (mg_handle_request_with_threading(c, hm, is_fast_path)) {
     return; // Request is being handled in a separate thread
   }
   ```

4. Handle the `MG_EV_WAKEUP` event in your event handler:
   ```c
   if (ev == MG_EV_WAKEUP) {
     mg_handle_wakeup_event(c, ev_data);
   }
   ```

## Performance Considerations

- Use multithreading only for long-running operations that would block the main event loop
- For simple requests that can be handled quickly, it's more efficient to handle them in the main thread
- Be careful with shared resources - use proper synchronization mechanisms when accessing shared data
