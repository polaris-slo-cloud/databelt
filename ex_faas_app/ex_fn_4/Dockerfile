# Use a lightweight Python image
FROM python:3.9-slim

# Set the working directory
WORKDIR ./

# Copy the Python code into the container
COPY app.py .

# Install the Redis library
RUN pip install redis
RUN pip install flask

# Set the entry point to run the script
ENTRYPOINT ["python", "app.py"]
