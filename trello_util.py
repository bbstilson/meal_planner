import re
import requests
from typing import Dict, List

class Card:
    def __init__(self, id: str, name: str, url: str, ingredients: List[str]):
        self.id = id
        self.name = name
        self.url = url
        self.ingredients = ingredients

class TrelloUtil:
    def __init__(self, key, token, url_base, dinner_list_id):
        self.url = f'{url_base}/lists/{dinner_list_id}/cards'
        self.auth = { 'key': key, 'token': token }

    def _split_once(self, word, text):
        return re.compile(word, flags=re.IGNORECASE).split(text, 1)

    def _parse_ingredients(self, desc) -> List[str]:
        try:
            [ingredients, _] = self._split_once('directions', desc)
            [_, cleaned_ingredients] = self._split_once('ingredients', ingredients)
            removed_hash = cleaned_ingredients.replace('#', '')
            removed_newlines = removed_hash.split('\n')
            return list(filter(None, map(lambda s: s.strip(), removed_newlines)))
        except Exception as e:
            return ['I couldn\'t parse the description 😕. Check the ingredients by clicking the link.']

    def _resp_to_card(self, c: Dict[str, str]) -> Card:
        return Card(
            id=c['id'],
            name=c['name'],
            url=c['url'],
            ingredients=self._parse_ingredients(c['desc'])
        )

    def get_meals(self):
        print('Fetching cards from Trello.')
        cards = requests.get(url=self.url,params=self.auth).json()
        return [self._resp_to_card(card) for card in cards]
