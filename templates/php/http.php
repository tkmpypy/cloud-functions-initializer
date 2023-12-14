<?php

use Google\CloudFunctions\FunctionsFramework;
use Psr\Http\Message\ServerRequestInterface;

// Register an HTTP function with the Functions Framework
FunctionsFramework::http('myHttpFunction', 'myHttpHandler');

// Define your HTTP handler
function myHttpHandler(ServerRequestInterface $request): string
{
    // Your code here

    // Return an HTTP response
    return 'OK';
}
