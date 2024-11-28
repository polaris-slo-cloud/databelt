FROM python:3.9-slim
WORKDIR ./
RUN mkdir settings
COPY node_service.py .
COPY requirements.txt .
COPY ./settings ./settings
RUN pip install -r requirements.txt
ENTRYPOINT ["python", "node_service.py"]
