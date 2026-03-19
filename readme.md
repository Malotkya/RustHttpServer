# Rust Http Server Library

## To Do:
* Add User Interface to http_server
    - Read User Input on main thread??
* Add EventEmitter to server!
    - Include Options to add event listeners
    - Add routers to event listeners?
    - Events to Include: connection, start, error, close, routing, connection_close
* Add Additional Logging Functions to Util
    - server! will parse options? and setup op static loggers
    - functions to use loggers can be imported and then used accordingly
    - hopefully moving accross threads wont be an issue
    - add debugging printing to be removed on release
    - logging options include:
        print to stdout, stderr
        print to files
        levels of logging (every connection open/close, or requests handled/sent)
        option to occasionally dump logs or clean logs up after a certain amount of time
        additional info included in every log, (thread_id, date/time, log level?(critical, handled_error, unexpected_error, warning, info, debug))