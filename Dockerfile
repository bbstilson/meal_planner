FROM python:3.7

# Install zip so we can zip the files and libs from inside the container.
RUN apt-get update && apt-get install zip

RUN mkdir -p /usr/src/app

# Install dependencies.
COPY requirements.txt /usr/src/app/
RUN pip install --target /usr/src/app/package -r /usr/src/app/requirements.txt

# Copy everything over.
COPY . /usr/src/app/

# Set the python path so modules "work".
ENV PYTHONPATH "/usr/src/app${PYTHONPATH:+:${PYTHONPATH}}"

# Set working directory.
WORKDIR /usr/src/app

ENTRYPOINT [ "/bin/bash" ]
