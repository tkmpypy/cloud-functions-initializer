require "functions_framework"

# Register an HTTP function with the Functions Framework
FunctionsFramework.http "my_http_function" do |request|
  # Your code here

  # Return an HTTP response
  "OK"
end
