FROM python:3.9-slim
WORKDIR ./
COPY app.py .
COPY requirements.txt .
RUN pip install -r requirements.txt
ENTRYPOINT ["python", "app.py"]
