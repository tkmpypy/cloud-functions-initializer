const functions = require('@google-cloud/functions-framework');

// Register a CloudEvent function with the Functions Framework
functions.cloudEvent('myCloudEventFunction', cloudEvent => {
  // Your code here
  // Access the CloudEvent data payload via cloudEvent.data
});
