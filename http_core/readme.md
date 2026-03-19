# Http Core Library

This library is the core of the http library and contains all the types needed to recieve requests and send responses.  It does not having anything needed to connect to the client.

## Error

Allows for easily sending a response with the appropriate status code when an error occurs.

## Headers

A wrapper around a HashMap and helper structs to easily read and set the headers of requests and responses.

## Method

An enum that represents the method of the request.

## Request

Holds all the data in the request, including the ability to read the request body.  

## Response

Holds all the data that the server will be sending back to the user.

## Result

The result of a Router function.

## Status

Allows for easily setting the status of the response.

## Url

All the parts of the a url, usually used to reperesent the full route of the http request.

## Version

A representation of the version used by the client when they sent their request.