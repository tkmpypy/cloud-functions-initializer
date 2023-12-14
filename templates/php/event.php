<?php

use CloudEvents\V1\CloudEventInterface;
use Google\CloudFunctions\FunctionsFramework;

// Register a CloudEvent function with the Functions Framework
FunctionsFramework::cloudEvent('myCloudEventFunction', 'myCloudEventHandler');

// Define your CloudEvent handler
function myCloudEventHandler(CloudEventInterface $event): void
{
    // Your code here
    // Access the CloudEvent data payload via $event->getData()
}
