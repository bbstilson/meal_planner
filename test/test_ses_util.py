import unittest

from ses_util import SESUtil
from trello_util import Card

class TestSESUtil(unittest.TestCase):
    def test__mk_body(self):
        meals = [Card(
            id='56f2214727212e',
            name='Sausage',
            ingredients=['1 Tbsp olive oil','4 veggies sausages'],
            url='https://trello.com/c/abc123/12-sausage'
        )]
        expected_body = '<h2>Sausage</h2><a href="https://trello.com/c/abc123/12-sausage">View on Trello.</a><br><strong><p>Ingredients:</p></strong><p>1 Tbsp olive oil<br>4 veggies sausages</p>'
        ses = SESUtil('from_email', ['to_emails'])
        body = ses._mk_body(meals)
        assert body == expected_body, f'{expected_body} did not equal {body}'
