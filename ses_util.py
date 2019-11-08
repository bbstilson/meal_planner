import boto3
from datetime import datetime
from typing import Any, Dict, List

class SESUtil:
    def __init__(self, from_email: str, to_emails: List[str]):
        self.from_email = from_email
        self.to_emails = to_emails
        self.ses = boto3.client('ses')

    def _mk_subject(self) -> str:
        now = datetime.now().strftime('%m/%d/%Y')
        return f'Your weekly meal plan is here! - {now}'

    def _mk_body(self, meals) -> str:
        email = ''

        for meal in meals:
            email += f'<h2>{meal["name"]}</h2>\n'
            email += f'<a href={meal["url"]}>View on Trello.</a>'
            email += '<br>\n'
            email += '<strong><p>Ingredients:</p></strong>'
            email += '<ul>'
            for ingredient in filter(None, meal["ingredients"].split('\n')):
                email += f'<li>{ingredient}</li>'
            email += '</ul>'
            email += '\n'

        return email

    def send_email(self, meals: List[Dict[str, str]]) -> None:
        print('Sending email.')
        response = self.ses.send_email(
            Source=self.from_email,
            Destination={ 'ToAddresses': self.to_emails },
            Message={
                'Subject': {
                    'Data': self._mk_subject(),
                    'Charset': 'utf8'
                },
                'Body': {
                    'Html': {
                        'Data': self._mk_body(meals),
                        'Charset': 'utf8'
                    }
                }
            },
            ReplyToAddresses=[ self.from_email ]
        )
        print(f"MessageId = {response['MessageId']}")
