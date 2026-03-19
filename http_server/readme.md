# Http Server Library

This library contains joining of the Http Core Library and Async Library, to create a server that can asynchronously send and recieve http requests.  

## Arguments

Gets input from the user or a config file to set up the server.

## Protocol

The layer between the tcp stream and the Http Core Library Request & Response.

### Stream Parser & Chunk

Custom parser that iterates over the TcpStream in Chunks.

### Tokenizer & Tokens

Additional layer of the parser that converts Chunks into tokens.

### Uri

Used to parse tokens into a valid request uri, is then converted to a url for the server.

### Version

Used to parse tokens into a valid http version.

## Server & Router

Listens for the TcpRequest, takes the information converted by the parser and runs through the possible request matches, before returning a response.