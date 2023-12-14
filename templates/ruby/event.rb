require "functions_framework"

# Register a CloudEvent function with the Functions Framework
FunctionsFramework.cloud_event "my_cloudevent_function" do |cloud_event|
  # Your code here
  # Access the CloudEvent data payload via cloud_event.data
end
