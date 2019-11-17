import unittest

from trello_util import Card, TrelloUtil

class TestTrelloUtil(unittest.TestCase):
    def test__resp_to_card(self):
        trello = TrelloUtil('API_KEY', 'TOKEN', 'URL_BASE', 'LIST_ID')
        mock_response = {
            'id': '1',
            'desc': '### INGREDIENTS \n\na \nb \nc \n\n\n### DIRECTIONS\n\n1) Mince the garlic. ',
            'name': 'Lentil Soup',
            'url': 'https://trello.com/c/abc/1-lentil-soup'
        }

        card = trello._resp_to_card(mock_response)
        assert card.id == '1', card.id
        assert card.name == 'Lentil Soup', card.name
        assert card.url == 'https://trello.com/c/abc/1-lentil-soup', card.url
        assert card.ingredients == ['a', 'b', 'c'], card.ingredients

    def test__parse_ingredients(self):
        trello = TrelloUtil('API_KEY', 'TOKEN', 'URL_BASE', 'LIST_ID')
        mock_desc = '### INGREDIENTS \n\n2 cloves garlic \n3 ribs celery \n\n15 oz can black beans \n1/4 tsp cayenne pepper\n\n\n### DIRECTIONS\n\n1) Mince the garlic. '
        ingredients = trello._parse_ingredients(mock_desc)
        assert ingredients == [
            '2 cloves garlic',
            '3 ribs celery',
            '15 oz can black beans',
            '1/4 tsp cayenne pepper'
        ], ingredients

