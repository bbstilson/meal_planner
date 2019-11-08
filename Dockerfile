FROM python:3.7

WORKDIR /usr/local/src/app

# Install dependencies.
COPY requirements.txt .
RUN pip install -r requirements.txt

# Copy everything over.
COPY . .

# Set the python path so modules "work".
ENV PYTHONPATH "/usr/local/src/app${PYTHONPATH:+:${PYTHONPATH}}"

ENTRYPOINT [ "/bin/bash" ]
