#Async Library

This is library contains the methods needed to implement a basic async environment, and a wrapper around the std library to make it as async as possible.

## Events
Bassic Event Emitter and Listener pattern.

## Executor
Multi-thread pool, Job Queue, and Async Task Handler. A job is an intensive sync task running in a seperate thread, with a future waiting for the thread to return a response.

## Future
Stdlib implementation with poll/async methods.