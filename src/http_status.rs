/** Http Status Codes
 * 
 * Values From: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
 * 
 * @author Alex malotky
 */

 /// Get Message From Code
pub fn get_message(value:u16) -> &'static str{

    //Information Responses
    if value == 100 {
        return "CONTINUE";
    } else if value == 101 {
        return "SWITCHING PROTOCOLS"
    } else if value == 102 {
        return "PROCESSING"
    } else if value < 200 {
        return "UNKOWN"

    // OK Responses    
    }else if value == 200 {
        return "SUCCESS"
    } else if value == 201 {
        return "CREATED"
    } else if value == 202 {
        return "ACCEPTED"
    } else if value == 203 {
        return "NON_AUHTORITATIVE INFORMATION"
    } else if value == 204 {
        return "NO CONTENT"
    } else if value == 205 {
        return "RESET CONTENT"
    } else if value == 206 {
        return "PARTIAL CONTENT"
    } else if value == 207 {
        return "MULTI-STATUS"
    } else if value == 208 {
        return "ALREADY REPORTED"
    } else if value == 226 {
        return "IM USED"
    } else if value < 300 {
        return "OK"

    //Redirection Messages
    } else if value == 300 {
        return "MULTIPLE CHOICES"
    } else if value == 301 {
        return "MOVED PERMANENTLY"
    } else if value == 302 {
        return "FOUND"
    } else if value == 303 {
        return "SEE OTHER"
    } else if value == 304 {
        return "NOT MODIFIED"
    } else if value == 305 {
        return "USE PROXY"
    } else if value == 306 {
        return "UNUSED"
    } else if value == 307 {
        return "TEMPORARY REDIRECT"
    } else if value == 308 {
        return "PERMANENT REDIRECT"
    } else if value < 400 {
        return "REDIRECT"

    //Client Errors
    } else if value == 400 {
        return "BAD REQUEST"
    } else if value == 401 {
        return "UNAUTHORIZED"
    } else if value == 402 {
        return "PAYMENT REQURED"
    } else if value == 403 {
        return "FORBIDDEN"
    } else if value == 404 {
        return "NOT FOUND"
    } else if value == 405 {
        return "mETHOD NOT aLLOWED"
    } else if value == 406 {
        return "NOT ACCEPTABLE"
    } else if value == 407 {
        return "PROXY AUTHENTICATION REQUIRED"
    } else if value == 408 {
        return "REQUEST TIMEOUT"
    } else if value == 409 {
        return "CONFLICT"
    } else if value == 410 {
        return "GONE"
    } else if value == 411 {
        return "LENGTH REQUIRED"
    } else if value == 412 {
        return "PRECONDITION FAILED"
    } else if value == 413 {
        return "PAYLOAD TOO LARGE"
    } else if value == 414 {
        return "URI TOO LONG"
    } else if value == 415 {
        return "UNSUPPORTED MEDIA TYPE"
    } else if value == 416 {
        return "RANGE NOT SATISFIABLE"
    } else if value == 417 {
        return "EXPECTATION FAILED"
    } else if value == 421 {
        return "MISDIRECTED REQUEST"
    } else if value == 422 {
        return "UNPROCESSABLE CONTENT"
    } else if value == 423 {
        return "LOCKED"
    } else if value == 424 {
        return "FAILED DEPENDENCY"
    } else if value == 425 {
        return "TOO EARLY"
    } else if value == 428 {
        return "PRECONDITION REQUIRED"
    } else if value == 429 {
        return "TOO MANY REQUESTS"
    } else if value == 431 {
        return "REQUEST HEADER FIELDS TOO LARGE"
    } else if value == 451 {
        return "UNABLE FOR LEAGAL REASONS"
    } else if value < 500 {
        return "CLIENT ERROR"

        //Server Error
    } else if value == 500 {
        return "BAD REQUEST"
    } else if value == 501 {
        return "NOT IMPLEMENTED"
    } else if value == 502 {
        return "BAD GATEWAY"
    } else if value == 503 {
        return "SERVICE UNAVAILABLE"
    } else if value == 504 {
        return "GATEWAY TIMEOUT"
    } else if value == 505 {
        return "HTTP VERSION NOT SUPPORTED"
    } else if value == 506 {
        return "VARIANT ALSO NEGOTIATES"
    } else if value == 507 {
        return "INSUFFICIENT STORAGE"
    } else if value == 508 {
        return "LOOP DETECTED"
    } else if value == 510 {
        return "NOT EXTENDED"
    } else if value == 511 {
        return "NETWORK AUTHENTICATION REQUIRED"
    } else {
        return "INTERNAL SERVER ERROR"
    }
}