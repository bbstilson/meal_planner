import boto3
from typing import Dict

class S3Util:
    def __init__(self, bucket, key):
        self.bucket = bucket
        self.key = key
        self.s3 = boto3.client('s3')

    def get_suggest_counts(self) -> Dict[str, int]:
        """
        Fetches the suggest count object from s3. Parses the flat csv-esque
        files into a dict of card_id -> suggest_count.
        """
        print('Downloading suggest_counts from S3.')
        body = self.s3.get_object(Bucket=self.bucket, Key=self.key)['Body'].read().decode('utf-8')
        parsed = map(lambda x: tuple(x.split(',')), filter(lambda x: ',' in x, body.split('\n')))
        cleaned = map(lambda x: (x[0], int(x[1])), parsed)
        return dict(cleaned)

    def update_suggest_counts(self, suggest_counts: Dict[str, int]):
        """Builds up a new file from new suggest_counts dict."""
        print('Uploading suggest_counts to S3.')
        new_suggest_counts = ''
        for k,v in suggest_counts.items():
            new_suggest_counts += f'{k},{v}\n'
        return self.s3.put_object(Bucket=self.bucket, Key=self.key, Body=new_suggest_counts)
