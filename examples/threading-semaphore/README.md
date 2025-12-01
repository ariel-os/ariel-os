# Threading Semaphores

## About

This application demonstrates the usage of a
[`Semaphore`](https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/thread/sync/struct.Semaphore.html)
as synchronization method for threads.

The example starts three threads (0 through 2).
A static `Semaphore` is created as synchronization mechanism for the threads.
All threads wait for the semaphore to become available,
printing output before and after taking it.
Only thread 2 is different in that
it gives the semaphore three times before waiting on that same semaphore.
All threads must thus wait for thread 2 to release the semaphore
before they can continue.

## How to run

In this directory, run

    laze build -b nrf52840dk run

The application will start three threads with different priorities.
All but one threads block on a global `Semaphore` until the one thread sets it.

## Example output

When run, this example shows the following output:

    [INFO ] [ThreadId(0)@RunqueueId(3)] Taking semaphore...
    [INFO ] [ThreadId(1)@RunqueueId(2)] Taking semaphore...
    [INFO ] [ThreadId(2)@RunqueueId(1)] Giving semaphore...
    [INFO ] [ThreadId(0)@RunqueueId(3)] Done.
    [INFO ] [ThreadId(2)@RunqueueId(1)] Give semaphore returned.
    [INFO ] [ThreadId(2)@RunqueueId(1)] Giving semaphore...
    [INFO ] [ThreadId(1)@RunqueueId(2)] Done.
    [INFO ] [ThreadId(2)@RunqueueId(1)] Give semaphore returned.
    [INFO ] [ThreadId(2)@RunqueueId(1)] Giving semaphore...
    [INFO ] [ThreadId(2)@RunqueueId(1)] Give semaphore returned.
    [INFO ] [ThreadId(2)@RunqueueId(1)] Taking semaphore...
    [INFO ] [ThreadId(2)@RunqueueId(1)] Done.
    [INFO ] [ThreadId(2)@RunqueueId(1)] All three threads should have reported "Done.". exiting.
