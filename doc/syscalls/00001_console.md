---
driver number: 0x00001
---

# Console

## Overview

The console driver allows the process to write buffers to serial device. To
write a buffer, a process must share the buffer using `allow` then initiate the
write using a `command` call. It may also using `subscribe` to receive a
callback when the write has completed.

Once the write has completed, the buffer shared with the driver is released, so
can be deallocated by the process. This also means that it is necessary to
share a buffer for every write transaction, even if it's the same buffer.

## Command

  * ### Command number: `0`

    **Description**: Does the driver exist?

    **Argument 1**: unused

    **Argument 2**: unused

    **Returns**: SUCCESS if it exists, otherwise ENODEVICE

  * ### Command number: `1`

    **Description**: Initiate a write transaction of a buffer shared using `allow`.
    At the end of the transaction, a callback will be delivered if the process
    has `subscribed`.

    **Argument 1**: The maximum number of bytes to write.

    **Argument 2**: unused

    **Returns**: SUCCESS if the command was successful, EBUSY if no buffer was
    shared, or ENOMEM if the driver failed to allocate memory for the
    transaction.

## Subscribe

  * ### Subscribe number: `1`

    **Description**: Subscribe to write transaction completion event. The
    callback will be called whenever a write transaction completes.

    **Callback signature**: The callback receives a single argument, the number
    of bytes written in the transaction. The value of the remaining arguments
    is undefined.

    **Returns**: SUCCESS if the subscribe was successful or ENOMEM if the
    driver failed to allocate memory for the transaction.

## Allow

  * ### Allow number: `1`

    **Description**: Sets a shared buffer to be used as a source of data for
    the next write transaction. A shared buffer is released if it is replaced
    by a subsequent call and after a write transaction is completed. Replacing
    the buffer after beginning a write transaction but before receiving a
    completion callback is undefined (most likely either the original buffer or
    new buffer will be written in its entirety but not both).

    **Returns**: SUCCESS if the subscribe was successful or ENOMEM if the
    driver failed to allocate memory for the transaction.

