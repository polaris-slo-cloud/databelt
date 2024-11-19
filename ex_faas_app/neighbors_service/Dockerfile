FROM python:3.9-slim
WORKDIR ./
COPY ns.py .
COPY requirements.txt .
RUN pip install -r requirements.txt
ENTRYPOINT ["python", "ns.py"]
